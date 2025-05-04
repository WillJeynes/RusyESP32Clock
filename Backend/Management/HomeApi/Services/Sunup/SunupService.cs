using HomeApi.Services.Sunup;

namespace HomeApi.Templates;

public class SunupService
{
    public static async Task<SunupViewModel> ProcessSunupResult(double lat, double lng)
    {
        var sunriseResponse = await SunriseDataService.GetSunriseAsync(lat, lng);

        var sunrise = DateTime.Parse(sunriseResponse.daily.sunrise[0]);
        var sunset = DateTime.Parse(sunriseResponse.daily.sunset[0]);
        
        var (progress, isSunTime) = CalculateSunOrMoonPosition(sunrise ,sunset, DateTime.Now);
        Console.WriteLine(progress);
        Console.WriteLine(isSunTime);
        SunupViewModel model = new SunupViewModel()
        {
            Pos = (progress * 230f) - 75f,
            Url = isSunTime ? "http://127.0.0.1:5278/SunupAssets/Sun.png" : "http://127.0.0.1:5278/SunupAssets/Moon.png",
            BgUrl = isSunTime ? "http://127.0.0.1:5278/SunupAssets/BgSun.jpg" : "http://127.0.0.1:5278/SunupAssets/BgMoon.jpg",
            SunupTime = sunrise.ToString("t"),
            SundownTime = sunset.ToString("t"),
        };

        return model;
    }
    
    public static (double progress, bool isSunTime) CalculateSunOrMoonPosition(DateTime sunrise, DateTime sundown, DateTime currentTime)
    {
        if (sunrise > sundown)
            throw new ArgumentException("Sunrise must be earlier than sundown");

        if (currentTime >= sunrise && currentTime <= sundown)
        {
            // Daytime: sun moving
            TimeSpan dayDuration = sundown - sunrise;
            TimeSpan timeSinceSunrise = currentTime - sunrise;
            double progress = timeSinceSunrise.TotalSeconds / dayDuration.TotalSeconds;
            return (Math.Clamp(progress, 0.0, 1.0), true); // true = sun
        }
        else
        {
            // Nighttime: moon moving
            DateTime nextSunrise = sunrise.AddDays(1);

            // If it's before today's sunrise, adjust sundown to yesterday
            if (currentTime < sunrise)
                sundown = sundown.AddDays(-1);

            TimeSpan nightDuration = nextSunrise - sundown;
            TimeSpan timeSinceSundown = currentTime - sundown;
            double progress = timeSinceSundown.TotalSeconds / nightDuration.TotalSeconds;
            return (Math.Clamp(progress, 0.0, 1.0), false); // false = moon
        }
    }
}