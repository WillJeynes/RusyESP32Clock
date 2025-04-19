using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Bmp;
using SixLabors.ImageSharp.PixelFormats;

namespace HomeApi.Services;

public class BmpService
{
    public static async Task<byte[]> ProcessImage(string url)
    {
        //TODO: Fallback image, caching, short timeout
        var pngBytes = await RetreiveImageAsync(url);
        var bmpBytes = RenderPngToBmp(pngBytes);

        return bmpBytes;
    }
    private static byte[] RenderPngToBmp(byte[] data)
    {
        using var inputStream = new MemoryStream(data);
        using Image image = Image.Load<Rgba32>(inputStream);

        using var outputStream = new MemoryStream();
        image.Save(outputStream, new BmpEncoder()); 

        return outputStream.ToArray();
    }

    private static async Task<byte[]> RetreiveImageAsync(string imageUrl)
    {
        using var httpClient = new HttpClient();
        var response = await httpClient.GetAsync(imageUrl);

        response.EnsureSuccessStatusCode();
        return await response.Content.ReadAsByteArrayAsync();
    }
}