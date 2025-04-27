using HomeApi.Services.Interfaces;

namespace HomeApi.Templates;

public class SunupViewModel : ISvgViewModel
{
    public string Url { get; set; }
    public double Pos { get; set; }
    public string SunupTime { get; set; }
    public string SundownTime { get; set; }
}