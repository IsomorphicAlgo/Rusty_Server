# Rusty Server - Overview

## What is Rusty Server?

**Rusty Server** is a comprehensive astronomical data platform that serves as a centralized hub for space weather monitoring, exoplanet discovery tracking, and astronomical calculations. Think of it as your personal mission control center for understanding what's happening in space—from solar storms that could affect satellites to newly discovered planets orbiting distant stars.

The project hosts multiple services and databases on a powerful home server, providing real-time data, historical archives, and predictive capabilities for space weather events. It's designed to complement the CLI_Astro_Calc tool by providing a server-based infrastructure that can be accessed from anywhere.

---

## Why Does This Matter?

### Space Weather: The Invisible Force Affecting Our Technology

Every day, the Sun sends out streams of charged particles and radiation that can impact our modern technology. This "space weather" affects:

- **Satellites**: Solar storms can damage satellite electronics and cause communication blackouts
- **GPS Systems**: Space weather can degrade GPS accuracy, affecting navigation systems
- **Power Grids**: Strong geomagnetic storms can induce currents in power lines, potentially causing blackouts
- **Radio Communications**: Solar activity can disrupt radio signals used by aviation, maritime, and emergency services
- **Astronaut Safety**: High radiation levels during solar events pose risks to astronauts in space

By monitoring and predicting space weather, we can prepare for these events and protect critical infrastructure.

### Exoplanets: Discovering New Worlds

Since the first exoplanet (a planet outside our solar system) was discovered in 1995, astronomers have found thousands more. Each discovery helps us understand:
- How common Earth-like planets might be
- The diversity of planetary systems in our galaxy
- The conditions necessary for life to exist elsewhere

Rusty Server tracks these discoveries, maintaining a database of exoplanet data that can be queried and analyzed.

### Machine Learning Predictions: Looking into the Future

One of the most exciting aspects of this project is using machine learning to predict solar flares before they happen. By analyzing patterns in historical data, we can forecast space weather events with increasing accuracy—potentially giving satellite operators and mission planners hours of advance warning.

---

## What Does Rusty Server Do?

### Core Services

1. **Space Weather Monitoring**
   - Fetches real-time data from NOAA (National Oceanic and Atmospheric Administration)
   - Tracks solar flares from NASA's DONKI (Space Weather Database)
   - Monitors geomagnetic storms, radiation levels, and solar wind
   - Stores historical data for analysis and pattern recognition

2. **Exoplanet Discovery Database**
   - Integrates with NASA's Exoplanet Archive
   - Tracks newly discovered planets
   - Stores detailed information about planetary systems
   - Provides query capabilities for researchers and enthusiasts

3. **Astronomical Calculations Server**
   - Hosts the CLI_Astro_Calc tool as a server-based service
   - Provides astronomical calculation capabilities via API
   - Enables complex calculations without requiring local installation

4. **Machine Learning Predictions**
   - Uses historical data to predict solar flare events
   - Currently implements CPU-optimized models (XGBoost)
   - Future plans to integrate the Surya foundation model (NASA/IBM collaboration)
   - Tracks prediction accuracy and continuously improves

5. **Satellite Tracking & Deorbit Prediction** (Future)
   - Will track satellites using Two-Line Element (TLE) data
   - Calculate orbital positions and decay rates
   - Predict when satellites will re-enter Earth's atmosphere
   - Use machine learning to improve decay predictions

---

## Understanding Space Weather

### The Sun's Activity Cycle

The Sun goes through an approximately 11-year cycle of activity. During solar maximum, the Sun is more active with more sunspots, solar flares, and coronal mass ejections. During solar minimum, activity is lower. Understanding this cycle helps predict when space weather events are more likely.

### Solar Flares: Explosions on the Sun

**What are they?**
Solar flares are sudden, intense bursts of radiation from the Sun's surface. They occur when magnetic energy stored in the Sun's atmosphere is suddenly released.

**Classification:**
- **A-class**: Smallest flares (background level)
- **B-class**: Small flares
- **C-class**: Medium flares (minor effects)
- **M-class**: Large flares (can cause brief radio blackouts)
- **X-class**: Largest flares (can cause major radio blackouts and radiation storms)

**Why they matter:**
- X-ray and ultraviolet radiation from flares can ionize Earth's upper atmosphere
- This affects radio communications and GPS signals
- Strong flares can pose radiation risks to astronauts
- They often precede coronal mass ejections (CMEs), which can cause geomagnetic storms

### Geomagnetic Storms: Earth's Magnetic Field Under Attack

When the Sun sends out a coronal mass ejection (CME)—a massive cloud of charged particles—it can interact with Earth's magnetic field, causing a geomagnetic storm.

**The KP Index:**
Scientists measure geomagnetic activity on a scale from 0 to 9:
- **KP 0-2**: Quiet conditions
- **KP 3-4**: Unsettled to active
- **KP 5**: Minor storm (can affect power grids)
- **KP 6**: Moderate storm
- **KP 7**: Strong storm (significant impacts)
- **KP 8-9**: Severe to extreme storm (rare, major impacts)

**Effects:**
- Beautiful auroras (Northern and Southern Lights)
- Disruptions to power grids
- Satellite communication issues
- GPS accuracy degradation

### Solar Wind: The Constant Stream

The Sun continuously emits a stream of charged particles called the solar wind. This wind:
- Travels at speeds of 300-800 km/s (sometimes faster)
- Carries the Sun's magnetic field into space
- Interacts with Earth's magnetic field
- Can intensify during solar storms

**Key Measurements:**
- **Speed**: How fast particles are traveling
- **Density**: How many particles per unit volume
- **Bz Component**: The orientation of the magnetic field (negative Bz allows more particles to enter Earth's atmosphere)

---

## Understanding Exoplanets

### What Makes an Exoplanet?

An exoplanet is any planet that orbits a star other than our Sun. As of 2024, astronomers have discovered over 5,000 confirmed exoplanets, with thousands more candidates awaiting confirmation.

### Discovery Methods

1. **Transit Method**: Detects planets when they pass in front of their star, causing a tiny dip in brightness
2. **Radial Velocity**: Detects planets by measuring the star's "wobble" caused by the planet's gravity
3. **Direct Imaging**: Takes actual pictures of planets (very difficult, only works for large, distant planets)
4. **Gravitational Microlensing**: Uses the bending of light by gravity to detect planets

### Types of Exoplanets

- **Hot Jupiters**: Gas giants very close to their stars
- **Super-Earths**: Rocky planets larger than Earth but smaller than Neptune
- **Mini-Neptunes**: Planets with thick atmospheres, smaller than Neptune
- **Earth-like**: Rocky planets in the "habitable zone" where liquid water could exist

### The Habitable Zone

Also called the "Goldilocks zone," this is the region around a star where conditions might be right for liquid water to exist on a planet's surface—a key ingredient for life as we know it.

---

## Machine Learning in Space Weather Prediction

### Why Predict Solar Flares?

Predicting solar flares is challenging but valuable. If we can forecast a major flare hours in advance, we can:
- Protect satellites by putting them in safe mode
- Warn astronauts to seek shelter
- Prepare power grids for potential geomagnetic storms
- Plan space missions around quiet periods

### Current Approach: CPU-Optimized Models

The project currently uses XGBoost, a machine learning algorithm that works well on standard computer processors (CPUs). This model:
- Analyzes historical solar flare data
- Identifies patterns in space weather conditions
- Makes predictions based on current conditions
- Provides confidence scores for each prediction

### Future: The Surya Foundation Model

**Surya** is a cutting-edge AI model developed by NASA and IBM:
- A 366-million-parameter transformer model
- Trained on 9 years of Solar Dynamics Observatory (SDO) imagery
- Can predict solar flares up to 2 hours in advance
- Originally designed for GPU acceleration

The future goal is to host and integrate the Surya model when GPU hardware becomes available, providing even more accurate predictions.

### How Machine Learning Works for Space Weather

1. **Data Collection**: Gather years of historical space weather data and solar flare records
2. **Feature Engineering**: Extract meaningful patterns (solar wind speed, magnetic field orientation, sunspot activity, etc.)
3. **Model Training**: Teach the algorithm to recognize patterns that precede solar flares
4. **Prediction**: Use current conditions to predict future events
5. **Validation**: Compare predictions to actual events to improve accuracy

---

## Satellite Tracking & Deorbit Prediction (Future)

### Why Track Satellites?

With thousands of satellites in orbit, understanding their positions and predicting when they'll fall back to Earth is crucial for:
- **Space Traffic Management**: Avoiding collisions
- **Re-entry Safety**: Predicting where and when satellites will re-enter
- **Orbital Decay Understanding**: Learning how atmospheric drag affects different orbits

### Two-Line Element (TLE) Data

Satellites are tracked using TLE data—a standardized format that describes a satellite's orbit. This data is updated regularly as orbits change due to:
- Atmospheric drag (friction with the upper atmosphere)
- Solar activity (affects atmospheric density)
- Gravitational perturbations

### Orbital Decay

Over time, satellites lose altitude due to atmospheric drag. The rate of decay depends on:
- **Altitude**: Lower orbits decay faster
- **Solar Activity**: More active Sun = expanded atmosphere = more drag
- **Satellite Size**: Larger satellites experience more drag
- **Orbital Inclination**: Some orbits decay faster than others

### Machine Learning for Deorbit Prediction

By analyzing historical TLE data, machine learning models can:
- Predict when a satellite will re-enter Earth's atmosphere
- Estimate the decay rate more accurately than simple physics models
- Account for complex interactions between solar activity and atmospheric density
- Provide uncertainty estimates for predictions

---

## Project Architecture (Simplified)

```
┌─────────────────────────────────────────────────────────┐
│                    Rusty Server                        │
│  (Hosted on powerful home server with 16 CPU cores)      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌──────────────────┐  ┌──────────────────────────┐   │
│  │  Space Weather   │  │   Exoplanet Database     │   │
│  │     Database     │  │                          │   │
│  └──────────────────┘  └──────────────────────────┘   │
│                                                         │
│  ┌──────────────────┐  ┌──────────────────────────┐   │
│  │  CLI_Astro_Calc  │  │   ML Prediction Service   │   │
│  │     Server       │  │   (Solar Flare Models)    │   │
│  └──────────────────┘  └──────────────────────────┘   │
│                                                         │
│  ┌──────────────────────────────────────────────────┐  │
│  │         REST API (Accessible via HTTP)           │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
   NOAA Space Weather   NASA DONKI         NASA Exoplanet
   Prediction Center     (Solar Flares)     Archive
```

### Data Flow

1. **Data Collection**: Server automatically fetches data from NASA and NOAA APIs
2. **Storage**: Data is stored in MySQL databases for long-term access
3. **Caching**: Frequently accessed data is cached in memory for fast responses
4. **Processing**: Machine learning models analyze data to make predictions
5. **Serving**: REST API provides access to all data and predictions
6. **Access**: Users can query data via web interface, API, or CLI tools

---

## Educational Resources

### Understanding Space Weather

- **NOAA Space Weather Prediction Center**: Provides real-time space weather forecasts
- **NASA Space Weather**: Educational resources about space weather impacts
- **Aurora Forecasts**: Predictions of when and where auroras will be visible

### Learning About Exoplanets

- **NASA Exoplanet Archive**: Comprehensive database of confirmed exoplanets
- **Exoplanet Exploration**: NASA's educational site about exoplanet science
- **Planetary Habitability**: Research on potentially habitable worlds

### Machine Learning in Astronomy

- **AstroML**: Machine learning applications in astronomy
- **Time Series Forecasting**: How ML predicts future events from historical data
- **Neural Networks**: Understanding how AI models learn patterns

---

## Future Vision

### Short-Term Goals

- Complete integration of CLI_Astro_Calc as a server-based service
- Expand exoplanet database with automatic discovery notifications
- Improve solar flare prediction accuracy with more training data
- Add visualization tools for space weather data

### Long-Term Goals

- **Surya Model Integration**: Host the full Surya foundation model for advanced solar flare prediction
- **Satellite Tracking System**: Implement TLE data integration and orbital mechanics calculations
- **ML-Based Deorbit Prediction**: Use machine learning to predict satellite re-entry times
- **Mars Weather Forecasting**: Expand to include Mars weather data and predictions
- **Real-Time Discovery Pipeline**: Automatically process and notify about new astronomical discoveries

### The Big Picture

Rusty Server aims to become a comprehensive platform for astronomical data and calculations. By combining real-time monitoring, historical archives, and predictive capabilities, it provides a powerful tool for understanding our place in the universe—from the space weather affecting our technology to the distant worlds we're discovering.

---

## Getting Started

For technical details, setup instructions, and API documentation, see:
- **README.md**: Technical documentation and setup guide
- **DEVELOPMENT_PLAN.md**: Detailed development roadmap and implementation plans
- **Guides/**: Comprehensive guides for database setup, API usage, and deployment

---

*This overview is designed to provide a high-level, educational understanding of the Rusty Server project. For technical implementation details, please refer to the README and development documentation.*
