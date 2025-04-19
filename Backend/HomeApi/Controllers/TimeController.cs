using HomeApi.Services;
using Microsoft.AspNetCore.Mvc;

namespace HomeApi.Controllers;

[ApiController]
[Route("[controller]")]
public class TimeController : ControllerBase
{
    [HttpGet("GetCurrentTime")]
    public string GetCurrentTime()
    {
        return DateTimeOffset.Now.ToUnixTimeMilliseconds().ToString();
    }
    
    [HttpGet("Image/{no}")]
    public async Task<IActionResult> GenerateBmp(int no)
    {
        //TEMP DEMO
        var imageUrl = "https://picsum.photos/230/100";
        if (no == 0)
        {
            //TODO: configuration via frontend
            imageUrl = "http://localhost:5278/WeatherImage/Main?lat=53.617068&long=-0.2111111";
        }
        //END TEMP DEMO
        
        var bmpBytes = await BmpService.ProcessImage(imageUrl);

        return File(bmpBytes, "image/bmp");
    }
}