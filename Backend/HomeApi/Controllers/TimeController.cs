using System.Drawing.Imaging;
using Microsoft.AspNetCore.Mvc;
using SixLabors.Fonts;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Drawing.Processing;
using SixLabors.ImageSharp.Formats.Bmp;
using SixLabors.ImageSharp.PixelFormats;
using SixLabors.ImageSharp.Processing;
using Color = System.Drawing.Color;
using PointF = System.Drawing.PointF;

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
        int width = 230, height = 100;
        counter++;
        using (var image = new Image<Rgba32>(width, height))
        {
            FontFamily fontFamily;
            
            if (!SystemFonts.TryGet("Comfortaa", out fontFamily))
                throw new Exception($"Couldn't find font");
            
            Font font = fontFamily.CreateFont(32f, FontStyle.Regular);

            var options = new TextOptions(font)
            {
                Dpi = 72,
                KerningMode = KerningMode.Standard
            };

            image.Mutate(x => x.DrawText("API: " + counter, font,SixLabors.ImageSharp.Color.White, new SixLabors.ImageSharp.PointF(10,10)));
            image.Mutate(x => x.DrawText("*Weather*", font,SixLabors.ImageSharp.Color.White, new SixLabors.ImageSharp.PointF(10,50)));
            // Convert image to BMP format in memory
            using (var ms = new MemoryStream())
            {
                image.Save(ms, new BmpEncoder());
                return File(ms.ToArray(), "image/bmp");
            }
        }
    }
}