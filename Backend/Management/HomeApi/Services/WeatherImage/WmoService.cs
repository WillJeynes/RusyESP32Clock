using Newtonsoft.Json;

namespace HomeApi.Templates;

public class WmoService
{
    public static WeatherInfo GetForCode(int code, bool isDay)
    {

        string templatePath = Path.Combine("Assets", "WmoCodes.json");

        var json = File.ReadAllText(templatePath);
        var weatherData = JsonConvert.DeserializeObject<Dictionary<string, WeatherEntry>>(json);
        
        if (weatherData.TryGetValue(code.ToString(), out WeatherEntry entry))
        {
            return isDay? entry.day : entry.night;
        }
        else
        {
            return weatherData.First().Value.day;
        }
    }
}
public class WeatherInfo
{
    public string description { get; set; }
    public string image { get; set; }
}

public class WeatherEntry
{
    public WeatherInfo day { get; set; }
    public WeatherInfo night { get; set; }
}