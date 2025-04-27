using Newtonsoft.Json;

namespace HomeApi.Services.Sunup;

public class SunriseDataService
{
    private static readonly HttpClient httpClient = new HttpClient();

    public static async Task<SunriseResponse> GetSunriseAsync(double latitude, double longitude)
    {
        string url = $"https://api.open-meteo.com/v1/forecast?" +
                     $"latitude={latitude}&longitude={longitude}" +
                     $"&daily=sunset,sunrise";

        var response = await httpClient.GetStringAsync(url);
        return JsonConvert.DeserializeObject<SunriseResponse>(response);
    }
}

public class SunriseResponse
{
    public Daily daily { get; set; }
}

public class Daily
{
    public List<string> sunset { get; set; }
    public List<string> sunrise { get; set; }
}
