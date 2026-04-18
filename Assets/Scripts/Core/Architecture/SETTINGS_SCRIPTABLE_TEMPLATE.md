# Settings Scriptable Template

This document serves as a template for creating Settings ScriptableObjects that store editor-configurable processor settings.

## Settings Scriptable Structure

```csharp
using UnityEngine;

namespace Scriptables
{
    /// <summary>
    /// ScriptableObject containing editor-configurable settings for ExampleProcessor.
    /// </summary>
    [CreateAssetMenu(fileName = "ExampleSettings", menuName = "Data/Settings/ExampleSettings")]
    public class ExampleSettingsScriptable : ScriptableObject, IDataScriptable
    {
        #region Editor-Configurable Settings
        
        /// <summary>
        /// Description of this setting.
        /// </summary>
        [Header("General Settings")]
        [SerializeField] private float _someValue;
        public float SomeValue => _someValue;
        
        /// <summary>
        /// Description of this setting.
        /// </summary>
        [SerializeField] private int _maxCount;
        public int MaxCount => _maxCount;
        
        /// <summary>
        /// Description of this setting.
        /// </summary>
        [SerializeField] private bool _isEnabled;
        public bool IsEnabled => _isEnabled;
        
        #endregion
        
        #region References to Unity Objects
        
        /// <summary>
        /// Reference to a Unity object (e.g., prefab, material, component).
        /// </summary>
        [Header("References")]
        [SerializeField] private GameObject _somePrefab;
        public GameObject SomePrefab => _somePrefab;
        
        /// <summary>
        /// Reference to a material.
        /// </summary>
        [SerializeField] private Material _someMaterial;
        public Material SomeMaterial => _someMaterial;
        
        #endregion
        
        #region Lists and Arrays
        
        /// <summary>
        /// Description of this list.
        /// </summary>
        [Header("Collections")]
        [SerializeField] private List<GameObject> _objectList;
        public List<GameObject> ObjectList => _objectList;
        
        /// <summary>
        /// Description of this array.
        /// </summary>
        [SerializeField] private float[] _valueArray;
        public float[] ValueArray => _valueArray;
        
        #endregion
    }
}
```

## Key Rules

### 1. **Namespace and Interface**
- Use `Scriptables` namespace
- Implement `IDataScriptable` interface (empty marker interface used by Coordinator for reflection)
- Keep consistent with other ScriptableObject types

### 2. **CreateAssetMenu**
- Add `[CreateAssetMenu]` attribute for easy creation in Unity editor
- Set appropriate menu path under "Data/Settings/"
- Use descriptive file name (e.g., "ExampleSettings" not "ExampleSettingsScriptable")

### 3. **Serialization**
- Use `[SerializeField]` for all fields to allow Unity serialization
- Expose settings through public properties (get-only)
- Never expose public set accessors (settings should only be set in editor)

### 4. **Organization**
- Use `[Header]` attributes to group related settings
- Organize into logical sections:
  - General Settings
  - References (Unity objects)
  - Collections (lists/arrays)
  - Advanced Settings (if applicable)

### 5. **Value Types**
- Settings should be value types or references to Unity objects
- Avoid complex object graphs
- Keep settings simple and editor-friendly

### 6. **No Runtime Changes**
- Settings are meant to be configured in the editor only
- Do not change settings at runtime
- Use RuntimeData for runtime state that changes

### 7. **Validation**
- Consider adding `OnValidate()` for basic validation
- Ensure default values are sensible
- Use `[Range]` attributes for numeric values where appropriate

## Common Patterns

### Numeric Values with Range
```csharp
[Header("Timing Settings")]
[Range(0.1f, 10f)]
[SerializeField] private float _updateInterval;
public float UpdateInterval => _updateInterval;

[Range(1, 100)]
[SerializeField] private int _maxObjects;
public int MaxObjects => _maxObjects;
```

### Toggles/Booleans
```csharp
[Header("Feature Flags")]
[SerializeField] private bool _enableLogging;
public bool EnableLogging => _enableLogging;

[SerializeField] private bool _useAdvancedMode;
public bool UseAdvancedMode => _useAdvancedMode;
```

### Color Settings
```csharp
[Header("Visual Settings")]
[SerializeField] private Color _primaryColor;
public Color PrimaryColor => _primaryColor;

[SerializeField] private Gradient _colorGradient;
public Gradient ColorGradient => _colorGradient;
```

### References
```csharp
[Header("Object References")]
[SerializeField] private GameObject _prefab;
public GameObject Prefab => _prefab;

[SerializeField] private Material _material;
public Material Material => _material;

[SerializeField] private AudioClip _soundEffect;
public AudioClip SoundEffect => _soundEffect;
```

### Lists of Objects
```csharp
[Header("Object Lists")]
[SerializeField] private GameObject[] _spawnPrefabs;
public GameObject[] SpawnPrefabs => _spawnPrefabs;

[SerializeField] private List<ScriptableObject> _dataList;
public List<ScriptableObject> DataList => _dataList;
```

### Vector Settings
```csharp
[Header("Position Settings")]
[SerializeField] private Vector3 _offset;
public Vector3 Offset => _offset;

[SerializeField] private Vector2 _range;
public Vector2 Range => _range;
```

### Layer/Tag Settings
```csharp
[Header("Layer Settings")]
[SerializeField] private LayerMask _interactionLayer;
public LayerMask InteractionLayer => _interactionLayer;

[SerializeField] private string[] _validTags;
public string[] ValidTags => _validTags;
```

## Advanced Features

### OnValidate for Validation
```csharp
#if UNITY_EDITOR
private void OnValidate()
{
    // Ensure values are within valid ranges
    _updateInterval = Mathf.Max(0.01f, _updateInterval);
    _maxObjects = Mathf.Max(1, _maxObjects);
    
    // Ensure lists are initialized
    if (_objectList == null)
        _objectList = new List<GameObject>();
}
#endif
```

### Conditional Display
```csharp
#if UNITY_EDITOR
private void OnValidate()
{
    // Example: Show warning if advanced mode is enabled without required settings
    if (_useAdvancedMode && _advancedSetting == 0)
        Debug.LogWarning("Advanced mode enabled but advanced setting is 0", this);
}
#endif
```

### Default Values
```csharp
private void Reset()
{
    _someValue = 1.0f;
    _maxCount = 10;
    _isEnabled = true;
}
```

## Naming Conventions

### File Naming
- Use PascalCase
- End with "Settings" not "SettingsScriptable"
- Examples: `ExampleSettings`, `PlayerInputSettings`, `AudioSettings`

### Field Naming
- Use private fields with underscore prefix: `_someValue`
- Use PascalCase for public properties: `SomeValue`

### Menu Naming
- Use descriptive menu paths
- Group related settings together
- Example: `Data/Settings/Input/PlayerInputSettings`

## Checklist for New Settings Scriptable

- [ ] Use `Scriptables` namespace
- [ ] Implement `IDataScriptable` interface
- [ ] Add `[CreateAssetMenu]` attribute with appropriate menu path
- [ ] Use `[SerializeField]` for all fields
- [ ] Expose settings through public properties (get-only)
- [ ] Use `[Header]` attributes to organize settings
- [ ] Use `[Range]` attributes for numeric values where appropriate
- [ ] Set sensible default values
- [ ] Add `OnValidate()` for validation if needed
- [ ] Add `Reset()` for default initialization
- [ ] Add XML comments to all public members
- [ ] Keep settings editor-configurable only (no runtime changes)
