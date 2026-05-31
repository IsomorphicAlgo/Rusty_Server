# Simple script to test solar wind data from Rusty Server API
param([string]$ServerUrl = "http://localhost:3000")

$response = Invoke-RestMethod -Uri "$ServerUrl/api/v1/space-weather/current"

Write-Host "Solar Wind Data:" -ForegroundColor Cyan
if ($response.data.solar_wind) {
    $sw = $response.data.solar_wind
    Write-Host "  Speed: $($sw.speed) km/s"
    Write-Host "  Density: $($sw.density) protons/cm³"
    Write-Host "  Temperature: $($sw.temperature) K"
    Write-Host "  Bz: $($sw.bz) nT"
} else {
    Write-Host "  Not available" -ForegroundColor Yellow
}
