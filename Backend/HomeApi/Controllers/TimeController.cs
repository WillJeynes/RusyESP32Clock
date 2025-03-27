using Microsoft.AspNetCore.Mvc;

namespace HomeApi.Controllers;

[ApiController]
[Route("[controller]")]
public class TimeController : ControllerBase
{
    [HttpGet("GetCurrentTime")]
    public string GetCurrentTime()
    {
        return DateTime.Now.ToString() + " : " + Random.Shared.Next(0,3000);
    }
}