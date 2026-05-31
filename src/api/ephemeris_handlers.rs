//! `/api/v1/ephemeris/*` — JSON over Ephemerust (see `Guides/API_EPHEMERIS.md`).

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use ephemerust::{
    celestial::{calculate_position, CelestialObject, ObserverLocation},
    julian_date, greenwich_mean_sidereal_time, local_sidereal_time,
    planets::Planet,
    ground_track, look_angles, predict_passes, propagate, subpoint, Tle,
};
use serde::Deserialize;
use serde_json::{json, Value};

/// HTTP error body per `Guides/API_EPHEMERIS.md`.
#[derive(Debug)]
pub struct EphemerisReject {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
    pub hint: Option<String>,
}

impl EphemerisReject {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        message: impl Into<String>,
        hint: Option<String>,
    ) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            hint,
        }
    }
}

impl IntoResponse for EphemerisReject {
    fn into_response(self) -> Response {
        let mut body = json!({
            "error": self.code,
            "message": self.message,
        });
        if let Some(h) = self.hint {
            body["hint"] = Value::String(h);
        }
        (self.status, Json(body)).into_response()
    }
}

fn map_astro_error(e: ephemerust::AstroError) -> EphemerisReject {
    let hint = e.hint().map(str::to_string);
    let msg = e.to_string();
    match &e {
        ephemerust::AstroError::InvalidCoordinate(_)
        | ephemerust::AstroError::InvalidTime(_)
        | ephemerust::AstroError::Tle(_)
        | ephemerust::AstroError::CalculationError(_)
        | ephemerust::AstroError::SatelliteError(_) => {
            EphemerisReject::new(StatusCode::UNPROCESSABLE_ENTITY, "ephemeris_compute", msg, hint)
        }
        ephemerust::AstroError::IoError(io) => EphemerisReject::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal",
            io.to_string(),
            None,
        ),
    }
}

fn parse_utc(s: &str) -> Result<DateTime<Utc>, EphemerisReject> {
    s.parse::<DateTime<Utc>>()
        .map_err(|_| EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_time",
            format!("could not parse utc as RFC3339 / ISO-8601: {s}"),
            None,
        ))
}

// --- time ---

#[derive(Debug, Deserialize)]
pub struct EphemerisTimeRequest {
    pub utc: String,
    pub observer_longitude_deg: Option<f64>,
}

pub async fn ephemeris_time(
    Json(req): Json<EphemerisTimeRequest>,
) -> Result<Json<Value>, EphemerisReject> {
    let dt = parse_utc(&req.utc)?;
    let jd = julian_date(dt);
    let gmst = greenwich_mean_sidereal_time(jd);
    let lst_hours = req
        .observer_longitude_deg
        .map(|lon| local_sidereal_time(gmst, lon));

    Ok(Json(json!({
        "utc": dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "julian_date": jd,
        "gmst_hours": gmst,
        "lst_hours": lst_hours,
    })))
}

// --- position ---

#[derive(Debug, Deserialize)]
pub struct EphemerisPositionRequest {
    pub object: String,
    pub utc: String,
}

fn parse_celestial_object(s: &str) -> Result<CelestialObject, EphemerisReject> {
    let key = s.trim().to_lowercase();
    match key.as_str() {
        "sun" => Ok(CelestialObject::Sun),
        "moon" => Ok(CelestialObject::Moon),
        "earth" => Err(EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "unknown_object",
            "object 'earth' is not supported for geocentric position",
            None,
        )),
        _ => {
            if let Some(p) = Planet::from_name(&key) {
                if p == Planet::Earth {
                    return Err(EphemerisReject::new(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "unknown_object",
                        "object 'earth' is not supported for geocentric position",
                        None,
                    ));
                }
                Ok(CelestialObject::Planet(p))
            } else {
                Err(EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "unknown_object",
                    format!("unknown object: {s}"),
                    None,
                ))
            }
        }
    }
}

pub async fn ephemeris_position(
    Json(req): Json<EphemerisPositionRequest>,
) -> Result<Json<Value>, EphemerisReject> {
    let dt = parse_utc(&req.utc)?;
    let object = parse_celestial_object(&req.object)?;
    let ra_dec = calculate_position(object, dt).map_err(map_astro_error)?;

    Ok(Json(json!({
        "object": req.object.trim().to_lowercase(),
        "utc": dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "ra_hours": ra_dec.ra,
        "dec_deg": ra_dec.dec,
        "frame": "geocentric_equatorial_mean_of_date_approx",
    })))
}

// --- satellite track ---

const TLE_MAX_LEN: usize = 4096;
const PASSES_HOURS_MAX: i64 = 168;
const GROUND_HOURS_MAX: i64 = 24;

#[derive(Debug, Deserialize)]
pub struct ObserverJson {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    #[serde(default)]
    pub elevation_m: f64,
}

#[derive(Debug, Deserialize)]
pub struct EphemerisTrackRequest {
    pub tle: String,
    pub utc: String,
    pub mode: String,
    pub observer: Option<ObserverJson>,
    pub predict_passes_hours: Option<f64>,
    pub ground_track_hours: Option<f64>,
    #[serde(default = "default_pass_min_el")]
    pub pass_min_elevation_deg: f64,
}

fn default_pass_min_el() -> f64 {
    10.0
}

fn to_observer(o: &ObserverJson) -> ObserverLocation {
    ObserverLocation {
        latitude: o.latitude_deg,
        longitude: o.longitude_deg,
        elevation: o.elevation_m,
    }
}

pub async fn ephemeris_satellite_track(
    Json(req): Json<EphemerisTrackRequest>,
) -> Result<Json<Value>, EphemerisReject> {
    let tle_text = req.tle.trim();
    if tle_text.is_empty() {
        return Err(EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_tle",
            "tle must not be empty",
            None,
        ));
    }
    if tle_text.len() > TLE_MAX_LEN {
        return Err(EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_tle",
            format!("tle exceeds max length ({TLE_MAX_LEN} characters)"),
            None,
        ));
    }
    if !tle_text.is_ascii() {
        return Err(EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_tle",
            "tle must be ASCII only",
            None,
        ));
    }

    let utc = parse_utc(&req.utc)?;
    let tle = Tle::parse(tle_text).map_err(map_astro_error)?;

    let mode = req.mode.trim().to_lowercase();
    match mode.as_str() {
        "state" => {
            let state = propagate(&tle, utc).map_err(map_astro_error)?;
            Ok(Json(json!({
                "mode": "state",
                "catalog_number": tle.catalog_number,
                "utc": utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "teme": {
                    "position_km": state.position_km,
                    "velocity_km_s": state.velocity_km_s,
                }
            })))
        }
        "subpoint" => {
            let sp = subpoint(&tle, utc).map_err(map_astro_error)?;
            Ok(Json(json!({
                "mode": "subpoint",
                "catalog_number": tle.catalog_number,
                "utc": utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "geodetic": {
                    "latitude_deg": sp.latitude_deg,
                    "longitude_deg": sp.longitude_deg,
                    "height_km": sp.altitude_km,
                }
            })))
        }
        "look" => {
            let obs = req.observer.as_ref().ok_or_else(|| {
                EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "missing_observer",
                    "observer is required for mode look",
                    None,
                )
            })?;
            let la = look_angles(&tle, utc, to_observer(obs)).map_err(map_astro_error)?;
            Ok(Json(json!({
                "mode": "look",
                "catalog_number": tle.catalog_number,
                "utc": utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "look": {
                    "azimuth_deg": la.azimuth_deg,
                    "elevation_deg": la.elevation_deg,
                    "range_km": la.range_km,
                    "range_rate_km_s": la.range_rate_km_s,
                }
            })))
        }
        "passes" => {
            let obs = req.observer.as_ref().ok_or_else(|| {
                EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "missing_observer",
                    "observer is required for mode passes",
                    None,
                )
            })?;
            let hours = req.predict_passes_hours.ok_or_else(|| {
                EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "missing_field",
                    "predict_passes_hours is required for mode passes",
                    None,
                )
            })?;
            if hours <= 0.0 || hours > PASSES_HOURS_MAX as f64 {
                return Err(EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "invalid_window",
                    format!("predict_passes_hours must be in (0, {PASSES_HOURS_MAX}]"),
                    None,
                ));
            }
            let window_end = utc + Duration::seconds((hours * 3600.0).round() as i64);
            let passes = predict_passes(
                &tle,
                to_observer(obs),
                utc,
                window_end,
                req.pass_min_elevation_deg,
            )
            .map_err(map_astro_error)?;
            let passes_json: Vec<Value> = passes
                .iter()
                .map(|p| {
                    json!({
                        "aos": p.aos.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        "culmination": p.culmination.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        "los": p.los.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        "max_elevation_deg": p.max_elevation_deg,
                        "aos_azimuth_deg": p.aos_azimuth_deg,
                        "los_azimuth_deg": p.los_azimuth_deg,
                    })
                })
                .collect();
            Ok(Json(json!({
                "mode": "passes",
                "catalog_number": tle.catalog_number,
                "window_start_utc": utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "window_end_utc": window_end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "pass_min_elevation_deg": req.pass_min_elevation_deg,
                "passes": passes_json,
            })))
        }
        "ground" => {
            let hours = req.ground_track_hours.ok_or_else(|| {
                EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "missing_field",
                    "ground_track_hours is required for mode ground",
                    None,
                )
            })?;
            if hours <= 0.0 || hours > GROUND_HOURS_MAX as f64 {
                return Err(EphemerisReject::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "invalid_window",
                    format!("ground_track_hours must be in (0, {GROUND_HOURS_MAX}]"),
                    None,
                ));
            }
            let window_end = utc + Duration::seconds((hours * 3600.0).round() as i64);
            let step = Duration::seconds(60);
            let samples = ground_track(&tle, utc, window_end, step).map_err(map_astro_error)?;
            let track: Vec<Value> = samples
                .iter()
                .map(|s| {
                    json!({
                        "utc": s.time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                        "latitude_deg": s.subpoint.latitude_deg,
                        "longitude_deg": s.subpoint.longitude_deg,
                        "height_km": s.subpoint.altitude_km,
                    })
                })
                .collect();
            Ok(Json(json!({
                "mode": "ground",
                "catalog_number": tle.catalog_number,
                "window_start_utc": utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "window_end_utc": window_end.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                "step_seconds": 60,
                "samples": track,
            })))
        }
        _ => Err(EphemerisReject::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_mode",
            format!("unknown mode: {} (expected state|subpoint|look|passes|ground)", req.mode),
            None,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn time_j2000_gmst() {
        let req = EphemerisTimeRequest {
            utc: "2000-01-01T12:00:00Z".to_string(),
            observer_longitude_deg: None,
        };
        let Json(v) = ephemeris_time(Json(req)).await.unwrap();
        assert!((v["julian_date"].as_f64().unwrap() - 2451545.0).abs() < 1e-5);
        let gmst = v["gmst_hours"].as_f64().unwrap();
        assert!((gmst - 18.697374558).abs() < 1e-4);
        assert!(v["lst_hours"].is_null());
    }
}
