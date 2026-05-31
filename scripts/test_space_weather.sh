#!/bin/bash
# Simple script to test the Rusty Server space weather API
# Usage: ./scripts/test_space_weather.sh [server_url]

SERVER_URL="${1:-http://localhost:3000}"

echo "============================================================"
echo "Testing Rusty Server Space Weather API"
echo "============================================================"
echo ""

# Test health endpoint first
echo "1. Testing Health Endpoint..."
if HEALTH_RESPONSE=$(curl -s "$SERVER_URL/health" 2>/dev/null); then
    echo "   ✓ Health Check: $(echo $HEALTH_RESPONSE | jq -r '.status')"
    echo "   Service: $(echo $HEALTH_RESPONSE | jq -r '.service')"
    echo "   Version: $(echo $HEALTH_RESPONSE | jq -r '.version')"
else
    echo "   ✗ Health Check Failed"
    exit 1
fi

echo ""

# Test current conditions endpoint
echo "2. Testing Current Conditions Endpoint..."
if RESPONSE=$(curl -s "$SERVER_URL/api/v1/space-weather/current" 2>/dev/null); then
    echo "   ✓ Request Successful"
    echo ""
    
    # Display metadata
    echo "   Metadata:"
    echo "   - Timestamp: $(echo $RESPONSE | jq -r '.metadata.timestamp')"
    echo "   - Source: $(echo $RESPONSE | jq -r '.metadata.source')"
    echo "   - Cached: $(echo $RESPONSE | jq -r '.metadata.cached')"
    echo ""
    
    # Display KP Index
    if [ "$(echo $RESPONSE | jq -r '.data.kp_index')" != "null" ]; then
        echo "   KP Index:"
        echo "   - Value: $(echo $RESPONSE | jq -r '.data.kp_index.value')"
        echo "   - Level: $(echo $RESPONSE | jq -r '.data.kp_index.level')"
        echo "   - Timestamp: $(echo $RESPONSE | jq -r '.data.kp_index.timestamp')"
    else
        echo "   KP Index: Not available"
    fi
    echo ""
    
    # Display Solar Wind
    if [ "$(echo $RESPONSE | jq -r '.data.solar_wind')" != "null" ]; then
        echo "   Solar Wind:"
        echo "   - Speed: $(echo $RESPONSE | jq -r '.data.solar_wind.speed') km/s"
        echo "   - Density: $(echo $RESPONSE | jq -r '.data.solar_wind.density') protons/cm³"
        echo "   - Temperature: $(echo $RESPONSE | jq -r '.data.solar_wind.temperature') K"
        BZ=$(echo $RESPONSE | jq -r '.data.solar_wind.bz')
        if [ "$BZ" != "null" ]; then
            echo "   - Bz: $BZ nT"
        else
            echo "   - Bz: Not available"
        fi
        echo "   - Timestamp: $(echo $RESPONSE | jq -r '.data.solar_wind.timestamp')"
    else
        echo "   Solar Wind: Not available"
    fi
    echo ""
    
    # Display Solar Flare
    if [ "$(echo $RESPONSE | jq -r '.data.solar_flare')" != "null" ]; then
        echo "   Solar Flare:"
        echo "   - Class: $(echo $RESPONSE | jq -r '.data.solar_flare.class')"
        echo "   - Peak Time: $(echo $RESPONSE | jq -r '.data.solar_flare.peak_time')"
        echo "   - Begin Time: $(echo $RESPONSE | jq -r '.data.solar_flare.begin_time')"
        echo "   - End Time: $(echo $RESPONSE | jq -r '.data.solar_flare.end_time')"
        SOURCE=$(echo $RESPONSE | jq -r '.data.solar_flare.source_location')
        if [ "$SOURCE" != "null" ]; then
            echo "   - Source Location: $SOURCE"
        fi
    else
        echo "   Solar Flare: None detected"
    fi
    echo ""
    
    # Display Geomagnetic Storm
    if [ "$(echo $RESPONSE | jq -r '.data.geomagnetic_storm')" != "null" ]; then
        echo "   Geomagnetic Storm:"
        echo "   - Level: $(echo $RESPONSE | jq -r '.data.geomagnetic_storm.level')"
        echo "   - KP Index: $(echo $RESPONSE | jq -r '.data.geomagnetic_storm.kp_index')"
        START=$(echo $RESPONSE | jq -r '.data.geomagnetic_storm.start_time')
        if [ "$START" != "null" ]; then
            echo "   - Start Time: $START"
        fi
        END=$(echo $RESPONSE | jq -r '.data.geomagnetic_storm.end_time')
        if [ "$END" != "null" ]; then
            echo "   - End Time: $END"
        fi
    else
        echo "   Geomagnetic Storm: None detected"
    fi
    echo ""
    
    # Display Radiation
    if [ "$(echo $RESPONSE | jq -r '.data.radiation')" != "null" ]; then
        echo "   Radiation:"
        PROTON=$(echo $RESPONSE | jq -r '.data.radiation.proton_flux')
        if [ "$PROTON" != "null" ]; then
            echo "   - Proton Flux: $PROTON"
        fi
        ELECTRON=$(echo $RESPONSE | jq -r '.data.radiation.electron_flux')
        if [ "$ELECTRON" != "null" ]; then
            echo "   - Electron Flux: $ELECTRON"
        fi
        ALERT=$(echo $RESPONSE | jq -r '.data.radiation.alert_level')
        if [ "$ALERT" != "null" ]; then
            echo "   - Alert Level: $ALERT"
        fi
        TS=$(echo $RESPONSE | jq -r '.data.radiation.timestamp')
        if [ "$TS" != "null" ]; then
            echo "   - Timestamp: $TS"
        fi
    else
        echo "   Radiation: Not available"
    fi
    
else
    echo "   ✗ Request Failed"
    exit 1
fi

echo ""
echo "============================================================"
echo "Test Complete!"
echo "============================================================"

