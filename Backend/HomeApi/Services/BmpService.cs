using System.Collections.Concurrent;
using BitFaster.Caching;
using BitFaster.Caching.Lfu;
using HomeApi.Templates;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Bmp;
using SixLabors.ImageSharp.PixelFormats;

namespace HomeApi.Services;

public class BmpService
{
    private static readonly ICache<string, byte[]> Cache = new ConcurrentLfuBuilder<string, byte[]>()
        .WithCapacity(1000) // Set max cache size
        .Build();

    private static readonly HttpClient HttpClient = new();
    private static readonly Lazy<Task<byte[]>> FallbackImage = new(() => CreateFallbackImage("Processing..."));
    
    private static readonly ConcurrentDictionary<string, Task> InflightTasks = new();

    public static async Task<byte[]> ProcessImage(string url)
    {
        _ = FetchAndCacheImageAsync(url);
        if (Cache.TryGet(url, out var cachedImage))
        { 
            return cachedImage;
        }
        else
        {
            return await FallbackImage.Value;
        }
    }

    private static Task FetchAndCacheImageAsync(string url)
    {
        return InflightTasks.GetOrAdd(url, async _ =>
        {
            try
            {
                var pngBytes = await RetrieveImageAsync(url);
                var bmpBytes = RenderPngToBmp(pngBytes);

                Cache.AddOrUpdate(url, bmpBytes);
            }
            catch (TaskCanceledException)
            {
                Cache.AddOrUpdate(url, await CreateFallbackImage("Request timeout"));
            }
            catch (HttpRequestException ex)
            {
                Cache.AddOrUpdate(url, await CreateFallbackImage($"Received: {ex.StatusCode}"));
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Failed to fetch image from {url}: {ex.Message}");
                Cache.AddOrUpdate(url, await CreateFallbackImage(ex.Message));
            }
            finally
            {
                InflightTasks.TryRemove(url, out var _); // Clean up inflight task
            }
        });
    }

    private static byte[] RenderPngToBmp(byte[] data)
    {
        using var inputStream = new MemoryStream(data);
        using Image image = Image.Load<Rgba32>(inputStream);

        using var outputStream = new MemoryStream();
        image.Save(outputStream, new BmpEncoder());

        return outputStream.ToArray();
    }

    private static async Task<byte[]> RetrieveImageAsync(string imageUrl)
    {
        using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(10));
        var response = await HttpClient.GetAsync(imageUrl, cts.Token);

        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsByteArrayAsync();
    }

    private async static Task<byte[]> CreateFallbackImage(string message)
    {
        var viewModel = new ErrorViewModel()
        {
            Description = message
        };
        var svgString = await SvgService.RetreiveSvgString("ErrorTemplate.svg", viewModel);
        var imageResult = SvgService.RenderSvgToPng(svgString);

        return RenderPngToBmp(imageResult);
    }
}