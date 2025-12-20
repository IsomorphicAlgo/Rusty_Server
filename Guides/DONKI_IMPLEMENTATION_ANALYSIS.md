# DONKI API Implementation Analysis

## Question 1: Can we safely implement DONKI? What would it take?

**Answer: YES, it can be safely implemented.** Here's what it would take:

---

## Safety Assessment

### ✅ **Safe to Implement**

1. **API Requirements:**
   - ✅ Free NASA API key (register at https://api.nasa.gov)
   - ✅ Well-documented REST API
   - ✅ Standard HTTP/JSON (no special protocols)
   - ✅ Rate limits are reasonable (1,000/hour with registered key)

2. **Rate Limits (Safe):**
   - **Registered API Key**: 1,000 requests/hour
   - **DEMO_KEY**: 30 requests/hour (for testing)
   - Your current caching layer will help stay within limits
   - Rate limit headers provided: `X-RateLimit-Limit`, `X-RateLimit-Remaining`

3. **No Breaking Changes:**
   - DONKI is separate from NOAA API (different base URL)
   - Can be added alongside existing NOAA integration
   - Won't affect current functionality

4. **Error Handling:**
   - Your existing retry logic in `noaa.rs` can be reused
   - Graceful degradation already implemented (returns None on failure)

---

## Implementation Requirements

### 1. **Configuration Changes**

**Add to `src/config.rs`:**
```rust
pub struct DonkiConfig {
    pub base_url: String,        // "https://api.nasa.gov/DONKI"
    pub api_key: Option<String>, // Required for DONKI
    pub timeout_seconds: u64,
}
```

**Add to `config.example.toml`:**
```toml
[donki]
base_url = "https://api.nasa.gov/DONKI"
api_key = ""  # Get free key from https://api.nasa.gov
timeout_seconds = 30
```

### 2. **New Service Module**

**Create `src/services/donki.rs`:**
- Similar structure to `noaa.rs`
- Methods:
  - `fetch_solar_flares(start_date, end_date)` → FLR endpoint
  - `fetch_cmes(start_date, end_date)` → CME endpoint
  - `fetch_geomagnetic_storms(start_date, end_date)` → GST endpoint
  - `fetch_with_retry()` → Reuse retry logic pattern

### 3. **Parsing Module Updates**

**Update `src/services/parsing.rs`:**
- Add `parse_donki_flare()` function
- Parse DONKI FLR JSON response format
- Map to existing `SolarFlare` model

**DONKI FLR Response Format:**
```json
{
  "flrID": "2025-12-15T00:00:00-FLR-001",
  "beginTime": "2025-12-15T00:00Z",
  "peakTime": "2025-12-15T00:10Z",
  "endTime": "2025-12-15T00:20Z",
  "classType": "C1.0",
  "sourceLocation": "N10W10",
  "activeRegionNum": "12345"
}
```

### 4. **Integration Points**

**Update `src/services/noaa.rs`:**
- Modify `get_current_conditions()` to also call DONKI client
- Merge solar flare data from DONKI
- Keep existing NOAA data (KP index, solar wind)

**Update `src/state.rs`:**
- Add `donki_client: DonkiClient` to `AppState`

**Update `src/api/handlers.rs`:**
- No changes needed! Existing handlers will automatically get solar flare data

### 5. **Database Schema**

**Already Compatible!** ✅
- Your `space_weather_observations` table already has:
  - `solar_flare_class`
  - `solar_flare_peak_time`
  - `solar_flare_begin_time`
  - `solar_flare_end_time`
  - `solar_flare_source_location`

### 6. **Testing**

**Add tests:**
- Unit tests for DONKI parsing
- Integration tests for DONKI client
- Test rate limit handling
- Test error handling (API key missing, rate limit exceeded)

---

## Implementation Steps (Estimated: 4-6 hours)

1. **Get NASA API Key** (5 minutes)
   - Register at https://api.nasa.gov
   - Add to config/environment variables

2. **Add DONKI Config** (30 minutes)
   - Update `config.rs`
   - Update `config.example.toml`
   - Add to state initialization

3. **Create DONKI Client** (1-2 hours)
   - Create `src/services/donki.rs`
   - Implement FLR endpoint fetching
   - Add retry logic (reuse pattern from `noaa.rs`)

4. **Add Parsing Logic** (1 hour)
   - Add `parse_donki_flare()` to `parsing.rs`
   - Map DONKI format to `SolarFlare` model
   - Add validation

5. **Integrate with NOAA Client** (1 hour)
   - Update `get_current_conditions()` to fetch from DONKI
   - Merge solar flare data
   - Handle errors gracefully

6. **Testing** (1 hour)
   - Unit tests for parsing
   - Integration tests
   - Test with real API (using DEMO_KEY first)

7. **Documentation** (30 minutes)
   - Update README with DONKI info
   - Document API key setup

---

## Potential Challenges & Solutions

### Challenge 1: API Key Management
**Solution:** Use environment variables (already supported in your config system)
```bash
RUSTY_SERVER__DONKI__API_KEY=your_key_here
```

### Challenge 2: Rate Limits
**Solution:** 
- Your caching layer already helps
- Add rate limit monitoring (check `X-RateLimit-Remaining` header)
- Implement exponential backoff (you already have this)

### Challenge 3: Date Range Queries
**Solution:**
- DONKI requires `startDate` parameter (required)
- `endDate` is optional (defaults to today)
- For "last 7 days", calculate: `startDate = today - 7 days`

### Challenge 4: Data Format Differences
**Solution:**
- DONKI uses different field names than NOAA
- Your parsing layer already handles this pattern
- Map DONKI fields to your `SolarFlare` model

---

## Safety Checklist

- ✅ API is publicly available and documented
- ✅ Free API key available (no cost)
- ✅ Rate limits are reasonable and documented
- ✅ Error handling patterns already exist
- ✅ Won't break existing functionality
- ✅ Database schema already supports solar flares
- ✅ Can be tested with DEMO_KEY first
- ✅ Can be disabled via config if needed

---

## Recommendation

**YES, proceed with implementation.** It's:
- Low risk (separate service, won't break existing code)
- Well-documented API
- Free to use
- Fits your existing architecture
- Addresses Priority A (solar flare data)

**Suggested Approach:**
1. Start with FLR (solar flares) endpoint only
2. Test thoroughly with DEMO_KEY
3. Add other DONKI endpoints (CME, GST) later if needed
4. Monitor rate limits in production

---

## Next Steps After Implementation

Once DONKI is integrated:
1. Test with real API key
2. Verify data is stored correctly in database
3. Check that web UI (when built) displays solar flares
4. Monitor rate limit usage
5. Consider adding other DONKI endpoints (CME, GST) if needed
