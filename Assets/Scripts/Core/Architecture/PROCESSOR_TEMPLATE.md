# Processor Template

This document serves as a template and reference for creating new processors in the Stream-Town-Reloaded project.

## Architecture Overview

Processors are the core logic units of the game system. Each processor:
- Implements `IProcessor` and `IInstaller` interfaces
- Uses Reflex dependency injection for all dependencies
- Stores runtime state in ScriptableObjects (RuntimeData)
- Registers itself and its RuntimeData as singletons in the DI container
- Only exists in ProjectScope (not scene-specific)

## Processor Structure

```csharp
using UnityEngine;
using Reflex.Attributes;
using Reflex.Core;
using ScriptablesProcessorInfrastructure;

namespace Processors
{
    /// <summary>
    /// Brief description of what this processor manages.
    /// Detailed description of its responsibilities and scope.
    /// </summary>
    public class ExampleProcessor : MonoBehaviour, IInstaller, IProcessor
    {
        #region Dependencies (Settings)
        
        /// <summary>
        /// ScriptableObject containing editor-configurable settings.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ExampleSettingsScriptable _exampleSettings;
        
        #endregion
        
        #region Dependencies (Runtime Data from Other Processors)
        
        /// <summary>
        /// ScriptableObject containing runtime data from another processor.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private OtherRuntimeData _otherData;
        
        #endregion
        
        #region Own Runtime Data
        
        /// <summary>
        /// ScriptableObject containing this processor's runtime state.
        /// Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private ExampleRuntimeData _exampleData;
        
        #endregion
        
        #region IProcessor Implementation
        
        /// <summary>
        /// Instantiates and registers this processor's RuntimeData as a singleton.
        /// Called during container initialization before dependency injection.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InjectRuntimeData(ContainerBuilder containerBuilder)
        {
            ExampleRuntimeData exampleData = ScriptableObject.CreateInstance<ExampleRuntimeData>();
            containerBuilder.AddSingleton(exampleData);
        }
        
        /// <summary>
        /// Initializes the processor.
        /// Called once during game initialization after all dependencies are injected.
        /// Use this instead of Awake/Start.
        /// </summary>
        public void Initialize()
        {
            // Initialize processor state here
            // Subscribe to events if needed
            // Set up initial values
        }
        
        /// <summary>
        /// Processes logic every frame.
        /// Called every frame by the Coordinator.
        /// Leave empty if processor doesn't need per-frame updates.
        /// </summary>
        public void Process()
        {
            // Per-frame logic here
        }
        
        #endregion
        
        #region IInstaller Implementation
        
        /// <summary>
        /// Registers this processor as a singleton in the dependency injection container.
        /// Called by Reflex during container initialization.
        /// Processors are registered in ProjectScope only.
        /// Also calls InjectRuntimeData to instantiate and register RuntimeData.
        /// </summary>
        /// <param name="containerBuilder">The container builder to register bindings with.</param>
        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            containerBuilder.AddSingleton(this);
            InjectRuntimeData(containerBuilder);
        }
        
        #endregion
        
        #region Public API
        
        /// <summary>
        /// Example public method that other processors can call.
        /// </summary>
        public void DoSomething()
        {
            // Implementation
        }
        
        /// <summary>
        /// Example property for exposing data.
        /// </summary>
        public int SomeValue => _exampleData.SomeValue;
        
        #endregion
        
        #region Private Methods
        
        /// <summary>
        /// Example private helper method.
        /// </summary>
        private void HelperMethod()
        {
            // Implementation
        }
        
        #endregion
    }
}
```

## Key Rules

### 1. **No Fields Other Than Injected Dependencies**
- All fields must be marked with `[Inject]` attribute
- No private fields that are not injected
- All state must be stored in RuntimeData ScriptableObjects

### 2. **No Awake/Start Methods**
- Use `Initialize()` instead of Awake/Start
- `Initialize()` is called after all dependencies are injected
- This ensures proper initialization order
- Do not implement `OnEnable()` or `OnDisable()` in processors

### 3. **RuntimeData Pattern**
If a processor manages its own runtime state:
- Create a corresponding `RuntimeData` ScriptableObject class
- Implement `InjectRuntimeData(ContainerBuilder containerBuilder)`
- Instantiate the RuntimeData using `ScriptableObject.CreateInstance<<>>()`
- Register it as a singleton using `containerBuilder.AddSingleton()`
- Keep the RuntimeData field marked as `[Inject]` so it gets injected

If a processor doesn't manage its own runtime state:
- Implement `InjectRuntimeData(ContainerBuilder containerBuilder)` as a no-op with a comment explaining why

### 4. **ProjectScope Only**
- Processors are registered as singletons in ProjectScope
- They persist across scene loads
- Do not register processors in scene-specific scopes

### 5. **Data Retrieval Sections**
Organize dependencies into clear sections:
- **Settings**: Editor-configurable ScriptableObjects (SettingsScriptable)
- **Runtime Data from Other Processors**: RuntimeData from other processors
- **Own Runtime Data**: This processor's own RuntimeData (if any)

### 6. **No Coroutines in Processors**
- Avoid coroutines in processor logic
- Use the Coordinator or dedicated coroutine processors if needed
- Keep processors stateless and logic-focused

### 7. **Temporary Legacy Exclusion Policy**
- Processors with serialized scene/UI references (for example `[SerializeField]` `Button`, `TextMeshProUGUI`, `GameObject`, `Image`) may be temporarily excluded from strict injected-only field conformance
- Add a standardized `TODO(Architecture)` note near the class declaration in excluded files
- Excluded processors MAY need migration to a non-processor pattern

## RuntimeData

For RuntimeData template and guidelines, see [RUNTIME_DATA_TEMPLATE.md](RUNTIME_DATA_TEMPLATE.md).

## Settings Scriptable

For Settings Scriptable template and guidelines, see [SETTINGS_SCRIPTABLE_TEMPLATE.md](SETTINGS_SCRIPTABLE_TEMPLATE.md).

## Checklist for New Processors

- [ ] Implement `IProcessor` and `IInstaller` interfaces
- [ ] Inherit from `MonoBehaviour`
- [ ] Add `[Inject]` attribute to all dependency fields
- [ ] Implement `InjectRuntimeData(ContainerBuilder containerBuilder)` if managing own RuntimeData
- [ ] Implement `Initialize()` for initialization logic
- [ ] Implement `Process()` for per-frame logic (leave empty if not needed)
- [ ] Implement `InstallBindings(ContainerBuilder containerBuilder)` to register as singleton and call InjectRuntimeData
- [ ] Create RuntimeData ScriptableObject in `ScriptablesProcessorInfrastructure` namespace if managing state (see [RUNTIME_DATA_TEMPLATE.md](RUNTIME_DATA_TEMPLATE.md))
- [ ] Create Settings ScriptableObject if editor-configurable settings are needed (see [SETTINGS_SCRIPTABLE_TEMPLATE.md](SETTINGS_SCRIPTABLE_TEMPLATE.md))
- [ ] No Awake/Start methods
- [ ] No OnEnable/OnDisable methods
- [ ] No private fields that aren't injected
- [ ] Document all public methods and properties
- [ ] Add XML comments to all public members
