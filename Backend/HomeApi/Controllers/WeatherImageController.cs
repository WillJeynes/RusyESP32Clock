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
        var model = await WeatherImageService.ProcessWeatherResult(lat, lng);
        var svgText = await SvgService.RetreiveSvgString("WeatherTemplate.svg", model);
        
        var data = SvgService.RenderSvgToPng(svgText);
        return File(data, "image/png");
    }
}