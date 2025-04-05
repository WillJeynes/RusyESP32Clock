
using System.Text;
using Microsoft.AspNetCore.Mvc;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Bmp;
using SixLabors.ImageSharp.PixelFormats;
using SixLabors.ImageSharp.Processing;
using SkiaSharp;
using Svg.Skia;

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
    static int counter = 0;
    [HttpGet("Image/{no}")]
    public IActionResult GenerateBmp(int no)
    {
        var svg = LoadAndCustomizeSvg("hello, world");
        var data = RenderSvgToPng(svg);
        return File(data, "image/bmp");
    }
    
    string LoadAndCustomizeSvg(string message)
    {
        string templatePath = Path.Combine("Templates", "Template0.svg");
        string svg = System.IO.File.ReadAllText(templatePath);
        svg =  svg.Replace("{{DESCRIPTION}}", message);
        svg =  svg.Replace("{{DAY0HREF}}", "https://picsum.photos/200");
        svg =  svg.Replace("{{IMAGE0}}", "https://picsum.photos/200");

        return svg;
    }
    byte[] RenderSvgToPng(string svgContent, int width = 230, int height = 100)
    {
        using var stream = new MemoryStream(Encoding.UTF8.GetBytes(svgContent));
        var svg = new SKSvg();
        svg.Load(stream);

        var info = new SKImageInfo(width, height);
        using var surface = SKSurface.Create(info);
        var canvas = surface.Canvas;
        canvas.Clear(SKColors.White);
        
        canvas.DrawPicture(svg.Picture, new SKPoint(0,0));

        using var simage = surface.Snapshot();
        using var data = simage.Encode(SKEncodedImageFormat.Png, 100);
        

        
        using var inputStream = new MemoryStream(data.ToArray());
        using Image image = Image.Load<Rgba32>(inputStream);

        using var outputStream = new MemoryStream();
        image.Save(outputStream, new BmpEncoder()); 

        return outputStream.ToArray();
    }
}