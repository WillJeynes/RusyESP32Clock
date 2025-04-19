using HomeApi.Services;
using HomeApi.Templates;
using Microsoft.AspNetCore.Mvc;

namespace HomeApi.Controllers;

[ApiController]
[Route("[controller]")]
public class WeatherImageController : ControllerBase
{
    [HttpGet("Main")]
    public async Task<IActionResult> GenerateWeatherPng([FromQuery] double lat, [FromQuery] double lng)
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
                Day =  $"{DateTime.Parse(weatherResponse.daily.time[index]):ddd}: {(int)weatherResponse.daily.temperature_2m_mean[index]}", 
                Image = daysWmo[index].image
            }).ToList();
        
        WeatherViewModel model = new WeatherViewModel()
        {
            MainDescription = currentWmo.description,
            MainImage = currentWmo.image,
            MainMax = (int)weatherResponse.daily.temperature_2m_max[0],
            MainMin = (int)weatherResponse.daily.temperature_2m_min[0],
            Days = entries
        };

        var svgText = await SvgService.RetreiveSvgString("WeatherTemplate.svg", model);
        
        var data = SvgService.RenderSvgToPng(svgText);
        return File(data, "image/png");
    }
}