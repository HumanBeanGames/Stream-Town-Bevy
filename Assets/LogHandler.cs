using System.IO;
using System.Runtime.CompilerServices;
using UnityEngine;

public static class CustomLogger
{
    public static string GetColor(string name)
    {
        var hue = (uint)name.GetHashCode()/(float)uint.MaxValue;
        var color = Color.HSVToRGB(hue, 0.6f, 1.0f);
        return ColorUtility.ToHtmlStringRGB(color);
    }

    public static void Log(object message, [CallerFilePath] string file = "")
    {
        Debug.Log(message);
    }
}
