# SettingsInstaller Template

This document serves as a template for creating SettingsInstaller MonoBehaviour wrappers for IDataScriptable assets.

## SettingsInstaller Structure

```csharp
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
    /// <summary>
    /// MonoBehaviour wrapper for [Name]Settings that implements IInstaller.
    /// References the serialized asset created in-editor.
    /// </summary>
    public class [Name]SettingsInstaller : MonoBehaviour, IInstaller
    {
        [SerializeField]
        private [Name]Settings _[name]Settings;

        public [Name]Settings [Name]Settings => _[name]Settings;

        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            containerBuilder.AddSingleton(this);
        }
    }
}
```

## Key Rules

### 1. **Naming Convention**
- Class name: `[Name]SettingsInstaller` (matching the Settings)
- Example: `GridSettingsInstaller` for `GridSettings`
- File name: `[Name]SettingsInstaller.cs`

### 2. **Namespace**
- Use `Data.Containers` namespace
- This namespace is exempt from `ScriptablesProcessorInfrastructure` restriction as DI infrastructure

### 3. **File Location**
- Place in `Assets/Scripts/Core/Installers/`
- All SettingsInstallers must be in this directory

### 4. **SceneScope Placement**
- Add the SettingsInstaller GameObject to SceneScope in Unity scenes
- This ensures the SettingsScriptable is available for dependency injection

### 5. **Interface Implementation**
- Must implement `IInstaller` interface
- Must inherit from `MonoBehaviour`

### 6. **SerializeField Field**
- Use `[SerializeField]` private field for the Settings asset
- Field name format: `_[name]Settings` (private, underscore prefix)
- The Settings asset is referenced via Unity inspector

### 7. **Public Property**
- Provide public property to expose the Settings
- Property name format: `[Name]Settings` (public, PascalCase)
- Property returns the private field

### 8. **InstallBindings**
- `InstallBindings` must register `this` as singleton
- Use `containerBuilder.AddSingleton(this)`
- No other bindings needed (the SettingsScriptable is accessed via the public property)

### 9. **XML Documentation**
- Add XML comments to the class
- Explain that it's a MonoBehaviour wrapper for the Settings
- Note that it references the serialized asset created in-editor

## Example: GridSettingsInstaller

```csharp
using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
    /// <summary>
    /// MonoBehaviour wrapper for GridSettings that implements IInstaller.
    /// References the serialized asset created in-editor.
    /// </summary>
    public class GridSettingsInstaller : MonoBehaviour, IInstaller
    {
        [SerializeField]
        private GridSettings _gridSettings;

        public GridSettings GridSettings => _gridSettings;

        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            containerBuilder.AddSingleton(this);
        }
    }
}
```

## Usage in Processor

Processors inject the SettingsInstaller, not the SettingsScriptable directly:

```csharp
public class GridProcessor : MonoBehaviour, IInstaller, IProcessor
{
    [Inject] private GridSettingsInstaller _gridSettingsInstaller;

    public void Initialize()
    {
        // Access settings through the installer
        var settings = _gridSettingsInstaller.GridSettings;
        var cellSize = settings.CellSize;
    }
}
```

## Checklist for New SettingsInstallers

- [ ] File named `[Name]SettingsInstaller.cs`
- [ ] Class named `[Name]SettingsInstaller`
- [ ] Namespace: `Data.Containers`
- [ ] Implements `IInstaller`
- [ ] Inherits from `MonoBehaviour`
- [ ] `[SerializeField]` private field for SettingsScriptable
- [ ] Public property to expose SettingsScriptable
- [ ] `InstallBindings` registers `this` as singleton
- [ ] XML documentation comments
- [ ] File placed in `Assets/Scripts/Core/Installers/`
- [ ] Added to SceneScope in Unity scenes
- [ ] SettingsScriptable asset assigned via inspector

## Common Patterns

### Multiple Settings in One Installer
If a processor needs multiple SettingsScriptables, create separate SettingsInstallers for each:

```csharp
// AudioSettingsInstaller.cs
public class AudioSettingsInstaller : MonoBehaviour, IInstaller
{
    [SerializeField] private AudioSettings _audioSettings;
    public AudioSettings AudioSettings => _audioSettings;
    public void InstallBindings(ContainerBuilder containerBuilder) => containerBuilder.AddSingleton(this);
}

// MusicSettingsInstaller.cs
public class MusicSettingsInstaller : MonoBehaviour, IInstaller
{
    [SerializeField] private MusicSettings _musicSettings;
    public MusicSettings MusicSettings => _musicSettings;
    public void InstallBindings(ContainerBuilder containerBuilder) => containerBuilder.AddSingleton(this);
}
```

## Troubleshooting

### Settings Not Injecting
- Verify SettingsInstaller is in SceneScope
- Verify Settings asset is assigned in inspector
- Verify SettingsInstaller GameObject is active in scene
- Check Unity console for DI container errors

### Null Reference in Processor
- Verify processor injects the SettingsInstaller, not the Settings
- Verify SettingsInstaller's public property returns the field correctly
- Check that the Settings asset exists and is valid
