using Newtonsoft.Json;

public class WeatherDataService
{
    private static readonly HttpClient httpClient = new HttpClient();

    public static async Task<WeatherResponse> GetWeatherForecastAsync(double latitude, double longitude)
    {
        string url = $"https://api.open-meteo.com/v1/forecast?" +
                     $"latitude={latitude}&longitude={longitude}" +
                     $"&daily=weather_code,temperature_2m_max,temperature_2m_min,temperature_2m_mean" +
                     $"&current=is_day,weather_code";

        var response = await httpClient.GetStringAsync(url);
        return JsonConvert.DeserializeObject<WeatherResponse>(response);
    }
}

public class WeatherResponse
{
    public double latitude { get; set; }
    public double longitude { get; set; }
    public double generationtime_ms { get; set; }
    public int utc_offset_seconds { get; set; }
    public string timezone { get; set; }
    public string timezone_abbreviation { get; set; }
    public double elevation { get; set; }
    public Units current_units { get; set; }
    public Current current { get; set; }
    public DailyUnits daily_units { get; set; }
    public Daily daily { get; set; }
}

public class Units
{
    public string time { get; set; }
    public string interval { get; set; }
    public string is_day { get; set; }
    public string weather_code { get; set; }
}

public class Current
{
    public string time { get; set; }
    public int interval { get; set; }
    public int is_day { get; set; }
    public int weather_code { get; set; }
}

public class DailyUnits
{
    public string time { get; set; }
    public string weather_code { get; set; }
    public string temperature_2m_max { get; set; }
    public string temperature_2m_min { get; set; }
    public string temperature_2m_mean { get; set; }
}

public class Daily
{
    public List<string> time { get; set; }
    public List<int> weather_code { get; set; }
    public List<double> temperature_2m_max { get; set; }
    public List<double> temperature_2m_min { get; set; }
    public List<double> temperature_2m_mean { get; set; }
}
