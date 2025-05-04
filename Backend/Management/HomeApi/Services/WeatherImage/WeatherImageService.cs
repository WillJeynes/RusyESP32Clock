namespace HomeApi.Templates;

public class WeatherImageService
{
    public static async Task<WeatherImageViewModel> ProcessWeatherResult(double lat, double lng)
    {
        var weatherResponse = await WeatherDataService.GetWeatherForecastAsync(lat, lng);
        
        var currentWmo =  WmoService.GetForCode(weatherResponse.current.weather_code, weatherResponse.current.is_day == 1);
        
        weatherResponse.daily.time.RemoveAt(0);
        weatherResponse.daily.weather_code.RemoveAt(0);
        weatherResponse.daily.temperature_2m_mean.RemoveAt(0);
        
        var daysWmo = weatherResponse.daily.weather_code.ConvertAll((item) =>  WmoService.GetForCode(item, true));
        var entries = Enumerable.Range(0, 5).Select(index => 
            new QuickEntry()
            {
                Day =  $"{DateTime.Parse(weatherResponse.daily.time[index]):ddd}:{(int)weatherResponse.daily.temperature_2m_mean[index]}\u00b0", 
                Image = daysWmo[index].image
            }).ToList();
        
        WeatherImageViewModel model = new WeatherImageViewModel()
        {
            MainDescription = currentWmo.description,
            MainImage = currentWmo.image,
            MainMax = (int)weatherResponse.daily.temperature_2m_max[0],
            MainMin = (int)weatherResponse.daily.temperature_2m_min[0],
            Days = entries
        };

        return model;
    }
}