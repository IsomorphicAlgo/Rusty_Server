Architectural Integration of Multi-Source Astronomical Data and Machine Learning Frameworks
The rapid digitization of astronomical observation has transformed the discipline from a field characterized by sparse measurements into a data-intensive science requiring sophisticated computational architectures. Developing a robust feature set for astronomical monitoring systems necessitates a multi-layered approach that integrates established planetary catalogs, real-time heliophysics alerts, and in-situ meteorological data from robotic explorers. By synthesizing high-fidelity data streams with autonomous systems and machine learning models, engineers can create platforms capable of predictive analysis, ranging from solar flare forecasting to the autonomous identification of exoplanetary candidates.
Multi-Source Integration for Exoplanetary and Stellar Systems
The foundation of a comprehensive astronomical feature set lies in the integration of specialized databases that extend beyond simple planet listings. While basic statistics for known planets can be retrieved through the Planets API, which provides key parameters such as mass, radius, and temperature for approximately 6,924 objects, professional-grade systems require the depth of the NASA Exoplanet Archive.1 This archive acts as a centralized repository for confirmed exoplanets and candidates discovered by missions such as Kepler, K2, and the Transiting Exoplanet Survey Satellite (TESS).3
To achieve high robustness, a system must utilize the Table Access Protocol (TAP) service provided by the NASA Exoplanet Science Institute. TAP allows for programmatic access to massive data tables using Structured Query Language (SQL) or its astronomical variant, Astronomical Data Query Language (ADQL).5 Unlike legacy APIs that returned static datasets, TAP enables developers to construct precise, server-side queries that filter results based on complex physical constraints.
Database Table
Content Description
Key Parameters for Feature Integration
ps
Planetary Systems: Primary record of all confirmed exoplanets.
pl_name, disc_year, discoverymethod, ra, dec
pscomppars
Composite Parameters: Best-estimate data compiled from multiple sources.
pl_bmassj, pl_radj, pl_orbper, st_teff
cumulative
KOI Cumulative List: Aggregate list of all Kepler Objects of Interest.
koi_disposition, koi_period, koi_prad
ml
Microlensing: Data on planets discovered via gravitational lens events.
ml_name, ml_mass_planet, ml_distance

The transition from retired API services to the TAP protocol reflects a broader trend toward interoperability within the Virtual Observatory community.5 For a robust implementation, the system should specifically query the ps (Planetary Systems) table, which provides a single-row-per-planet view containing self-contained sets of stellar and planetary parameters.7 Integrating discovery metadata, such as disc_pubdate (Discovery Publication Date) and disc_year (Discovery Year), allows for the creation of temporal tracking features that notify users of new additions to the scientific record.5
Beyond confirmed catalogs, a truly robust system incorporates discovery alert streams. The TESS mission, for instance, maintains a "TOI+" list containing the most recent candidates, which includes group vetting comments and priority rankings for follow-up observations.8 These data products are available as High-Level Science Products (HLSPs) at the Mikulski Archive for Space Telescopes (MAST) and can be retrieved as CSV files for periodic polling.9
Real-Time Space Weather and Heliophysics Monitoring
While terrestrial weather updates are commonly sourced from the National Oceanic and Atmospheric Administration (NOAA), space weather monitoring requires specialized streams from the NASA Space Weather Database of Notifications, Knowledge, and Information (DONKI). DONKI provides a comprehensive API for tracking Coronal Mass Ejections (CMEs), Solar Flares (FLR), and Geomagnetic Storms (GST).10
A significant challenge in space weather integration is the latency and multi-stage nature of solar events. A solar flare may cause an immediate radio blackout, but the associated CME may take 24 to 72 hours to reach Earth and trigger a geomagnetic storm.12 A robust feature set should utilize the DONKI notifications endpoint, which distributes alerts about these events between space- and ground-based observatories.10 This allows the server to act as a machine-to-machine node within the General Coordinates Network (GCN), processing rapid communications about high-energy and transient phenomena.13
Space Weather Event
Classification System
Technical Impacts and Indicators
Solar Flare
B, C, M, X-Class
Radio blackouts, satellite telemetry interference.
CME
Speed (km/s), Half-Angle
Driver of geomagnetic storms; tracked via SOHO/SDO.
Geomagnetic Storm
Kp-Index (0-9)
Aurora visibility; power grid voltage control problems.
Solar Particle Event
Flux Levels
Radiation hazards for high-altitude flight and astronauts.

The Kp-index is the primary metric for aurora prediction, ranging from G1 (Minor, Kp=5) to G5 (Extreme, Kp=9).12 By integrating real-time solar wind speed data—often provided in the HSS (High Speed Stream) and WSAEnlilSimulations endpoints of DONKI—a system can provide users with predictive lead times for aurora visibility.10 Furthermore, the IPS (Interplanetary Shock) endpoint provides critical data on the shock fronts that precede major geomagnetic disturbances, offering another layer of early warning capability.10
Martian Meteorology and In-Situ Planetary Data
Mars weather tracking is uniquely supported by several generations of robotic landers and rovers. The Mars Environmental Dynamics Analyzer (MEDA) on the Perseverance rover and the Rover Environmental Monitoring Station (REMS) on Curiosity provide the most detailed in-situ meteorological records available for another planet.14 These datasets are archived in the Planetary Data System (PDS) and include variables such as air temperature, ground temperature, humidity, wind velocity, and atmospheric pressure.14
Integrating Mars weather requires handling various data processing levels. Raw data (EDRs) are converted from Science Data Frames (SDF), while calibrated data (RDRs) provide physical environmental magnitudes.15 A sophisticated implementation would focus on the "Derived" data products, which use models to calculate unique magnitudes such as atmospheric dust opacity and water vapor columns.15
Sensor Suite
Data Provided
Scientific and Operational Utility
TIRS (Thermal IR)
Ground/Air Temperature
Tracking diurnal cycles and thermal inertia.
WS (Wind Sensor)
Horizontal/Vertical Speed
Identifying dust lifting events and storm fronts.
RDS (Radiation/Dust)
Solar Flux, Opacity
Monitoring atmospheric clarity for solar power.
PS (Pressure Sensor)
Surface Pressure
Mapping global circulation and CO2 cycle.

For a "global" Mars weather feature, developers can connect to the Mars Climate Database (MCD). This database is formed by averaging model output from Global Circulation Models (GCMs) and provides a three-dimensional spatial grid of the Martian atmosphere.18 This allows an application to predict conditions at any longitude and latitude, effectively providing a "weather map" for the entire planet based on historical trends and current orbital observations.18
Implementation of Autonomous Systems and Machine Learning
The integration of autonomous systems for predicting solar and planetary phenomena has shifted from simple heuristic models to deep learning architectures. The primary breakthrough in this field is the development of the Surya foundation model, a collaboration between NASA and IBM.20 Surya is a 366-million-parameter transformer model trained on nine years of high-resolution imagery from the Solar Dynamics Observatory (SDO).21
Unlike traditional machine learning models that require extensive labeling, Surya is a foundation model that learns general-purpose solar representations. This allows it to be fine-tuned for diverse downstream tasks, such as solar flare forecasting, active region segmentation, and solar wind prediction.23 A significant advantage of deploying a model like Surya on a local server is its ability to provide visual predictions of solar flares up to two hours before they occur, representing a 16% improvement in classification accuracy over previous benchmarks.25
The hardware requirements for running such a model are substantial. While inference can be performed on CUDA-capable GPUs, the original pre-training utilized 128 NVIDIA A100 GPUs.20 To handle the massive 4096x4096-pixel solar images, Surya employs a custom spatiotemporal transformer architecture with spectral gating and long-short range attention.23
Model Component
Technical Specification
Operational Benefit
Base Architecture
Spatiotemporal Transformer
Analyzes evolution of solar features over time.
Parameter Count
366 Million
High capacity for complex physical representations.
Pre-training Data
218 TB (SDO AIA/HMI)
Captures patterns across a full solar cycle.
Optimization
Autoregressive Rollout
Enables stable multi-step-ahead forecasting.

For exoplanet identification, autonomous vetting systems such as ExoMiner utilize deep neural networks to classify light curve signals.28 These models analyze the photometric time-series data to solve a binary classification task: determining if a transit signal is caused by a real planet or a false positive like an eclipsing binary star.30 Local servers can implement these models using Python libraries such as TensorFlow or PyTorch, training on publicly available datasets from the NASA Exoplanet Archive to automate the identification of new candidates in TESS or Kepler data.31
Machine Learning for Mars Weather Forecasting
Predicting Martian weather, particularly hazardous dust storms, is critical for mission safety. Current research utilizes the OpenMARS dataset—a reanalysis product merging spacecraft observations with the Mars Global Circulation Model—to train time-series forecasting models.17 Architectures such as Temporal Convolutional Networks (TCN) and TiDE have shown superior performance in forecasting surface temperature and pressure variables.17
One innovative approach involves "transfer learning," where models trained on terrestrial weather data, such as Microsoft’s "Aurora," are adapted for Martian atmospheric variables.19 This involves re-mapping surface and atmospheric components to fit the dimensions and units of the Martian environment, such as converting pressure levels to those relevant for the thinner Martian atmosphere.19
Forecasting Variable
Model Architecture
Best Performing Task
Temperature/Pressure
TCN, TiDE
Short-term diurnal cycle prediction.
Dust Optical Depth
Transformer, LSTM
Tracking storm growth after initiation.
Wind Stress
Deep Learning (CNN)
Global migration patterns of dust devils.

While these models are effective at short-term forecasting, they still face challenges with the stochastic nature of dust storm initiation. Research indicates that using deep learning to track the migration of "dust devils" in orbital imagery provides a distributed characterization of near-surface winds, which is essential for more accurate global models.33 Integrating these derived wind stresses into a predictive model allows for a more robust understanding of atmospheric dust sourcing and its impact on the global Martian climate.33
Tracking Celestial Retrograde Motion in Computational Tools
The calculation of planetary retrograde motion—the apparent backward movement of a planet in the sky—can be efficiently implemented in a CLI-based calculator using high-precision ephemeris libraries like Skyfield or Astropy.35 From a geocentric perspective, retrograde motion is defined by a decrease in the planet's Right Ascension ($\alpha$) over time.37
The mathematical condition for retrograde motion is expressed by the derivative of the Right Ascension with respect to time:


$$\frac{d\alpha}{dt} < 0$$

To implement this in a CLI tool, the script must perform the following computational steps:
Ephemeris Loading: Load a JPL development ephemeris file, such as de421.bsp or de440s.bsp, which provides the positions of the Earth and target planets relative to the solar system barycenter.35
Coordinate Transformation: Calculate the geocentric (Earth-centered) or topocentric (observer-centered) position of the target planet at two closely spaced time intervals, $t$ and $t + \Delta t$.35
RA Comparison: Extract the Right Ascension in the ICRS (International Celestial Reference System) frame. If the value at $t + \Delta t$ is less than at $t$, the planet is in retrograde.36
Stationary points occur when $d\alpha/dt = 0$, representing the moments the planet appears to stop and change direction.39 While comparing simple RA differences is common, a more robust method involves using vector calculus on the Cartesian $x, y, z$ coordinates of the planet relative to Earth to find the exact moment of velocity reversal.37
Celestial Body
Retrograde Frequency
Average Duration
Mercury
~3 times per year
~21 days
Venus
~every 1.6 years
~42 days
Mars
~every 2.1 years
~72 days
Jupiter
~once per year
~121 days

Using Skyfield, a CLI calculator can automate this by iterating through a date range and flagging periods of decreasing RA. This provides a professional-grade alternative to manual ephemeris lookups and can be integrated into larger mission planning or observational scheduling systems.35
Engineering Real-Time Discovery and Notification Pipelines
To finalize a robust astronomical system, a server-side notification pipeline should be established to handle the high volume of discovery alerts. The Vera C. Rubin Observatory’s Legacy Survey of Space and Time (LSST) is expected to produce 10 million alerts per night beginning in 2025.41 Accessing this stream requires the use of community brokers like Lasair, which provide APIs and real-time push notifications.41
These brokers allow users to upload "watchlists" of specific celestial coordinates. When an alert falls within a specified region of the sky, the broker cross-matches the transient event with extensive library catalogs and categorizes the object (e.g., supernova, active galaxy, or variable star).41 For exoplanet discoveries specifically, the TESS Objects of Interest (TOI) mailing list provides direct notification of new releases, which can be scraped or integrated via the TESS Exoplanet Vetter (TEV) website.8
The convergence of these streams—NASA DONKI for heliophysics, PDS for Mars meteorology, TAP for exoplanet databases, and GCN for transients—creates a unified, autonomous observatory. By hosting a Surya model instance and implementing automated ADQL queries, a developer can create a platform that not only tracks current celestial events but also predicts the evolution of the sun and the atmospheres of neighboring planets, providing a truly robust and insight-rich window into the universe.
Data Governance and System Scaling
Managing the diverse data formats inherent in these systems is a critical architectural challenge. Exoplanetary data from TAP is typically returned in VOTable, CSV, or JSON formats, whereas high-resolution solar images from SDO are large binary files requiring specialized processing pipelines.5 A robust system should implement a tiered storage architecture:
Hot Tier: Real-time JSON streams from DONKI and GCN for immediate alerts.10
Warm Tier: Indexed SQL databases (PostgreSQL/SQLite) containing frequently queried parameters from the ps and pscomppars tables.5
Cold Tier: Archival storage for raw sensor data (PDS) and full-resolution SDO images used for long-term model training or retrospective analysis.15
Rate limits must also be managed carefully. For instance, the NASA API DEMO_KEY is insufficient for production-level polling, necessitating an upgrade to a registered API key to avoid service interruptions.10 By implementing a robust data ingestion layer that respects these limits while maintaining local caches of critical datasets, developers can ensure their astronomical features remain responsive and reliable under varying network conditions and data volumes.

### Future work
Satellite Orbital Decay and Re-entry Prediction: Beyond flares, you could build a model to predict the orbital decay of satellites in Low Earth Orbit (LEO). By using historical Two-Line Element (TLE) data and physics-guided neural networks, you can estimate when a satellite will re-enter the atmosphere due to atmospheric drag, which is heavily influenced by solar activity.

Mars Dust Storm Forecasting: Since dust storms pose a major threat to mission safety on Mars, you could work on a project using the OpenMars dataset. Research shows that Temporal Convolutional Networks (TCN) are effective at forecasting surface temperature and pressure, though predicting the exact initiation of dust storms remains a "holy grail" for Martian weather ML.


Works cited
Planets API - API Ninjas, accessed December 19, 2025, https://api-ninjas.com/api/planets
NASA Exoplanet Archive Overview and Holdings, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/docs/intro.html
NASA Open APIs | Postman API Network, accessed December 19, 2025, https://www.postman.com/miguelolave/nasa-open-apis/overview
NASA Exoplanet Archive, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/
Retrieving Exoplanet Archive Data With Table Access Protocol, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/docs/TAP/usingTAP.html
Using the Application Programming Interface (API) - NASA Exoplanet Archive - Caltech, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/docs/program_interfaces.html
Planetary Systems and Planetary Systems Composite Parameters Data Column Definitions - NASA Exoplanet Archive, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/docs/API_PS_columns.html
TOI Releases - TESS - MIT, accessed December 19, 2025, https://tess.mit.edu/toi-releases/
Data Products From TESS Data Alerts - The TESS Team - MAST Archive, accessed December 19, 2025, https://archive.stsci.edu/prepds/tess-data-alerts/
NASA Open APIs, accessed December 19, 2025, https://api.nasa.gov/
Search Space Weather Notification Archive - NASA, accessed December 19, 2025, https://kauai.ccmc.gsfc.nasa.gov/DONKI/search/alerts
NASA Space Weather Events Dataset - Kaggle, accessed December 19, 2025, https://www.kaggle.com/datasets/edacelikeloglu/nasa-space-weather-data
General Coordinates Network(GSC-19205-1) - NASA Software Catalog, accessed December 19, 2025, https://software.nasa.gov/software/GSC-19205-1
MEDA: Mars Weather and Atmosphere Sensor Data - Kaggle, accessed December 19, 2025, https://www.kaggle.com/datasets/nikitamanaenkov/meda-mars-weather-and-atmosphere-sensor-data
MEDA - Mars Environmental Dynamics Analyzer, accessed December 19, 2025, https://pds-atmospheres.nmsu.edu/data_and_services/atmospheres_data/PERSEVERANCE/meda.html
Mars Data Archive - PDS Atmospheres Node, accessed December 19, 2025, https://pds-atmospheres.nmsu.edu/data_and_services/atmospheres_data/Mars/Mars.html
Weather Prediction on Mars as a Multivariate Time Series Forecasting Problem - CEUR-WS.org, accessed December 19, 2025, https://ceur-ws.org/Vol-4128/paper2.pdf
MARS CLIMATE DATABASE ACCESS, accessed December 19, 2025, https://www-mars.lmd.jussieu.fr/mars/access.html
Building the Foundation for Machine Learning-Based Mars Weather Forecasting - SC25, accessed December 19, 2025, https://sc25.supercomputing.org/proceedings/posters/poster_files/post186s2-file2.pdf
Meet Surya, a New AI Model From NASA and IBM That Can Predict ..., accessed December 19, 2025, https://www.cnet.com/tech/services-and-software/meet-surya-a-new-ai-model-from-nasa-and-ibm-that-can-predict-solar-flares/
IBM and NASA Release Groundbreaking Open-Source AI Model on Hugging Face to Predict Solar Weather and Help Protect Critical Technology, accessed December 19, 2025, https://newsroom.ibm.com/2025-08-20-ibm-and-nasa-release-groundbreaking-open-source-ai-model-on-hugging-face-to-predict-solar-weather-and-help-protect-critical-technology
nasa-ibm-ai4science/Surya-1.0 - Hugging Face, accessed December 19, 2025, https://huggingface.co/nasa-ibm-ai4science/Surya-1.0
Surya: Foundation Model for Heliophysics - NASA Technical Reports Server, accessed December 19, 2025, https://ntrs.nasa.gov/api/citations/20250008498/downloads/SuryaFM%20Paper.pdf?attachment=true
Surya: Foundation Model for Heliophysics - ResearchGate, accessed December 19, 2025, https://www.researchgate.net/publication/394790641_Surya_Foundation_Model_for_Heliophysics
NASA, IBM's 'Hot' New AI Model Unlocks Secrets of Sun, accessed December 19, 2025, https://science.nasa.gov/science-research/artificial-intelligence-model-heliophysics/
Introducing Surya, a new heliophysics foundation model - IBM Research, accessed December 19, 2025, https://research.ibm.com/blog/surya-heliophysics-ai-model-sun
IBM and NASA Trained the First Foundational Model for Heliophysics - InfoQ, accessed December 19, 2025, https://www.infoq.com/news/2025/08/surya-model-heliophysics/
Exoplanet Detection Using Machine Learning Models Trained on Synthetic Light Curves, accessed December 19, 2025, https://arxiv.org/html/2507.19520
2021 Exoplanet Archive News, accessed December 19, 2025, https://exoplanetarchive.ipac.caltech.edu/docs/2021news.html
dinismf/exoplanet_classification_thesis: Exoplanet Transit Detection using Deep Neural Networks written in Python - GitHub, accessed December 19, 2025, https://github.com/dinismf/exoplanet_classification_thesis
Machine Learning Models for Exoplanet Detection: A Comparative Analysis of Kepler Mission Data - NHSJS, accessed December 19, 2025, https://nhsjs.com/2025/machine-learning-models-for-exoplanet-detection-a-comparative-analysis-of-kepler-mission-data/
Deep learning exoplanets detection by combining real and synthetic data - PMC - NIH, accessed December 19, 2025, https://pmc.ncbi.nlm.nih.gov/articles/PMC9132280/
Dust devil migration patterns reveal strong near-surface winds across Mars - PMC, accessed December 19, 2025, https://pmc.ncbi.nlm.nih.gov/articles/PMC12506970/
generating synthetic satellite images of mars dust storms based on radiative transfer models, as - Scholarworks@UAEU, accessed December 19, 2025, https://scholarworks.uaeu.ac.ae/context/all_theses/article/2059/viewcontent/Fatima_Alkaabi___COS_Thesis.pdf
Skyfield — documentation - Rhodes Mill, accessed December 19, 2025, https://rhodesmill.org/skyfield/
Astronomical Coordinate Systems - Astropy, accessed December 19, 2025, https://docs.astropy.org/en/stable/coordinates/index.html
Mathematically calculate if a Planet is in Retrograde - Astronomy Stack Exchange, accessed December 19, 2025, https://astronomy.stackexchange.com/questions/18832/mathematically-calculate-if-a-planet-is-in-retrograde
alanwsmith/retrograde-calculator: Figuring out what's in ... - GitHub, accessed December 19, 2025, https://github.com/alanwsmith/retrograde-calculator
How to calculate planetary aspects at a stationary point? - Astronomy Stack Exchange, accessed December 19, 2025, https://astronomy.stackexchange.com/questions/18974/how-to-calculate-planetary-aspects-at-a-stationary-point
Almanac Computation — Skyfield documentation - Rhodes Mill, accessed December 19, 2025, https://rhodesmill.org/skyfield/almanac.html
Enabling science from the Rubin alert stream with Lasair | RAS Techniques and Instruments | Oxford Academic, accessed December 19, 2025, https://academic.oup.com/rasti/article/3/1/362/7712474
ElixirTeSS/TeSS: Training e-Support Service using Ruby on Rails. - GitHub, accessed December 19, 2025, https://github.com/ElixirTeSS/TeSS
Real-time Alerts - IceCube Neutrino Observatory, accessed December 19, 2025, https://icecube.wisc.edu/science/real-time-alerts/
