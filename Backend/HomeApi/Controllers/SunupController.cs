using HomeApi.Services;
using HomeApi.Templates;
using Microsoft.AspNetCore.Mvc;

namespace HomeApi.Controllers;

[ApiController]
[Route("[controller]")]
public class SunupController : ControllerBase
{
    [HttpGet("Main")]
    public async Task<IActionResult> GenerateWeatherPng([FromQuery] double lat, [FromQuery] double lng)
    {
        var model = await SunupService.ProcessSunupResult(lat, lng);
        var svgText = await SvgService.RetreiveSvgString("SunupTemplate.svg", model);
        
        var data = SvgService.RenderSvgToPng(svgText);
        return File(data, "image/png");
    }
}