using Microsoft.AspNetCore.Mvc;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Bmp;
using SixLabors.ImageSharp.PixelFormats;

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
        var imageUrl = "https://picsum.photos/230/100";
        if (no == 0)
        {
            //TODO: configuration via frontend
            imageUrl = "http://localhost:5278/WeatherImage/Main?lat=53.617068&long=-0.2111111";
        }

        var pngBytes = await RetreiveImageAsync(imageUrl);
        var bmpBytes = RenderPngToBmp(pngBytes);

        return File(bmpBytes, "image/bmp");
    }
    
    byte[] RenderPngToBmp(byte[] data)
    {
        using var inputStream = new MemoryStream(data);
        using Image image = Image.Load<Rgba32>(inputStream);

        using var outputStream = new MemoryStream();
        image.Save(outputStream, new BmpEncoder()); 

        return outputStream.ToArray();
    }

    async Task<byte[]> RetreiveImageAsync(string imageUrl)
    {
        using var httpClient = new HttpClient();
        var response = await httpClient.GetAsync(imageUrl);

        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsByteArrayAsync();
    }
}