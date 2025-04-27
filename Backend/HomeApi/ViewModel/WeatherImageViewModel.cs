using HomeApi.Services.Interfaces;

namespace HomeApi.Templates;

public class WeatherImageViewModel : ISvgViewModel
{
    public string MainImage { get; set; }
    public string MainDescription { get; set; }
    public int MainMin { get; set; }
    public int MainMax { get; set; }
    public List<QuickEntry> Days { get; set; }
}

public class QuickEntry
{
    public string Day { get; set; }
    public string Image { get; set; }
}