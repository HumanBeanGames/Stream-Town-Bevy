using Reflex.Attributes;
using SavingAndLoading;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEngine;
using UnityEngine.Rendering.Universal;
using UnityEngine.Rendering;
using Reflex.Core;

public class GraphicsProcessor : UIGameObjectAccessor, IInstaller, IProjectUIInjectable
{
    public new void InstallBindings(ContainerBuilder containerBuilder)
    {
        base.InstallBindings(containerBuilder);
        containerBuilder.AddSingleton(this);
    }

    public void OnProjectUIInjected()
    {
        if (_initialized)
            return;

        if (!Directory.Exists(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + GameIO.SAVE_FILEPATH))
        {
            Directory.CreateDirectory(System.Environment.GetFolderPath(System.Environment.SpecialFolder.MyDocuments) + GameIO.SAVE_FILEPATH);
        }
        SetUpPipelineAndPostProcessing();
        SetUpResolution();
        _initialized = true;
    }

    //prviate variables
    private bool _initialized;
    private List<Resolution> _cachedResolutions;
    public List<Resolution> Resolutions
    {
        get
        {
            if (_cachedResolutions == null)
                SetUpResolution();
            return _cachedResolutions;
        }
    }
    public LiftGammaGain LiftGammaGain;
    public ColorAdjustments ColorAdjustments;

    [Inject] Volume _postProcessVolume;
    [Inject] UniversalRenderPipelineAsset _renderPipeline;
    private void SetUpPipelineAndPostProcessing()
    {
        _postProcessVolume.profile.TryGet(out LiftGammaGain);
        _postProcessVolume.profile.TryGet(out ColorAdjustments);

        GraphicsSettings.defaultRenderPipeline = _renderPipeline;
        QualitySettings.renderPipeline = _renderPipeline;
    }

    private void SetUpResolution()
    {
        _cachedResolutions = Screen.resolutions
            .GroupBy(r => (r.width, r.height))
            .Select(g => g.OrderByDescending(r => r.refreshRateRatio).First())
            .OrderBy(r => r.width).ThenBy(r => r.height)
            .ToList();

        /* REVISIT: This doesn't belong here, as it is UI related
        var options = _cachedResolutions
            .Select(r => $"{r.width} x {r.height}")
            .ToList();

        _resolutionDropdown.dropDown.ClearOptions();
        _resolutionDropdown.dropDown.AddOptions(options);

        var cur = Screen.currentResolution;
        var idx = _cachedResolutions.FindIndex(r =>
            r.width == cur.width && r.height == cur.height);
        if (idx < 0) idx = 0;

        _resolutionDropdown.val = idx;
        _resolutionDropdown.dropDown.RefreshShownValue();
        */
    }
}
