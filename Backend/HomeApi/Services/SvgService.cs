using System.Text;
using HomeApi.Services.Interfaces;
using RazorLight;
using SkiaSharp;
using Svg.Skia;

namespace HomeApi.Services;

public class SvgService
{
    public static async Task<string> RetreiveSvgString<T>(string filePath, T viewModel) where T : SvgViewModel
    {
        string templatePath = Path.Combine("Templates", filePath);
        string svg = await File.ReadAllTextAsync(templatePath);

        var engine = new RazorLightEngineBuilder()
            .UseEmbeddedResourcesProject(typeof(T))
            .SetOperatingAssembly(typeof(T).Assembly)
            .UseMemoryCachingProvider()
            .Build();

        return await engine.CompileRenderStringAsync("templateKey", svg, viewModel);
    }
    
    public static byte[] RenderSvgToPng(string svgContent, int width = 230, int height = 100)
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
}