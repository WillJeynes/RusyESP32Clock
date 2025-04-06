
using System.Text;
using HomeApi.Templates;
using Microsoft.AspNetCore.Mvc;
using RazorLight;
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
    [HttpGet("WeatherImage")]
    public async Task<IActionResult> GenerateWeatherPng()
    {
        var svg = await LoadAndCustomizeSvg("*Weather*");
        var data = RenderSvgToPng(svg);
        return File(data, "image/png");
    }
    
    [HttpGet("Image/{no}")]
    public async Task<IActionResult> GenerateBmp(int no)
    {
        var imageUrl = "https://picsum.photos/230/100";
        if (no == 0)
        {
            //TODO: configuration via frontend
            imageUrl = "http://localhost:5278/Time/WeatherImage";
        }

        var pngBytes = await RetreiveImageAsync(imageUrl);
        var bmpBytes = RenderPngToBmp(pngBytes);

        return File(bmpBytes, "image/bmp");
    }
    
    async Task<string> LoadAndCustomizeSvg(string message)
    {
        string templatePath = Path.Combine("Templates", "WeatherTemplate.svg");
        string svg = await System.IO.File.ReadAllTextAsync(templatePath);

        var engine = new RazorLightEngineBuilder()
            .UseEmbeddedResourcesProject(typeof(WeatherViewModel))
            .SetOperatingAssembly(typeof(WeatherViewModel).Assembly)
            .UseMemoryCachingProvider()
            .Build();

        var weatherResponse = await WeatherDataFetcher.GetWeatherForecastAsync(53.617068, -0.2111111);
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
        
        return await engine.CompileRenderStringAsync("templateKey", svg, model);
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
        
        return data.ToArray();
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