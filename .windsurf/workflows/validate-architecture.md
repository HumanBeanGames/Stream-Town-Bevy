---
description: Validate 3-layer architecture compliance for Stream Town Reloaded project
---

# Validate Architecture Compliance

This workflow validates that the codebase follows the strict 3-layer architecture defined in `Assets/Scripts/Core/Architecture/ARCHITECTURE.md`.

## Architecture Overview

The project uses a strict 3-layer architecture to separate concerns and maintain clean dependencies:

### Layer 1: Data Layer (ScriptableObjects)
**Purpose:** Store data only.

**Rules:**
- **Simple data access methods only** - ScriptableObjects may contain simple methods (3-5 lines) that get or set a single field with basic safety checks. Complex logic must be in processors.
- **No references to processors** - ScriptableObjects must not reference any processor classes
- **No references to other ScriptableObjects** - ScriptableObjects must not reference each other
- **No injections** - ScriptableObjects must not use `[Inject]` attributes or any dependency injection
- **Public properties with setters** - All private data fields must be exposed via public properties with getters and setters
- **Namespace: ScriptablesProcessorInfrastructure** - All IDataScriptable and IRuntimeDataScriptable implementations must use the `ScriptablesProcessorInfrastructure` namespace to enforce access control
- **Registered in SceneScope** - ScriptableObjects are loaded in the Scene Scope on a scene-by-scene basis as needed
- **RuntimeDatas have no installers** - RuntimeDatas DO NOT get installer classes. The processor instantiates and installs the RuntimeData directly on creation
- **RuntimeInstaller prohibition** - IRuntimeDataScriptable types must NOT have dedicated installer classes. No `*RuntimeInstaller.cs` files should exist in the project.

### Layer 2: Processor Layer
**Purpose:** Contain logic only.

**Rules:**
- **No data fields** - Processors must not have any data fields or state
- **Injected data objects only** - Processors may only have fields that are injected ScriptableObjects (data layer)
- **No state** - Processors must be stateless; all state must be stored in injected ScriptableObjects
- **Properties are allowed** - Properties are acceptable if they simply pass through data from injected objects
- **Functions only** - All logic must be implemented as methods
- **Dependency injection** - Processors use `[Inject]` attributes to receive data objects
- **IInstaller implementation** - Processors implement `IInstaller` to register themselves as singletons
- **InjectRuntimeData method** - Processors implement `InjectRuntimeData(ContainerBuilder containerBuilder)`; processors that manage RuntimeData instantiate and register it there, processors without RuntimeData implement a documented no-op
- **InstallBindings call pattern** - `InstallBindings(ContainerBuilder containerBuilder)` must register `this` and call `InjectRuntimeData(containerBuilder)`
- **No coroutines** - Processors must not use coroutines; use the `Process()` function for per-frame logic instead
- **No Awake or Start** - Processors must not have Awake or Start methods. All initialization logic must be in the `Initialize()` method
- **No OnEnable or OnDisable** - Processors must not have OnEnable or OnDisable methods

**Temporary Legacy Exclusion Policy:**
- Processors with serialized scene/UI references (e.g., `[SerializeField]` `Button`, `TextMeshProUGUI`, `GameObject`, `Image`) may be temporarily excluded from strict injected-only field conformance.
- Excluded files must include a standardized `TODO(Architecture)` note near the class declaration.
- Excluded processors MAY need migration to a non-processor pattern.

### Layer 3: Implementation Layer
**Purpose:** Use processors to interact with the system.

**Rules:**
- **No references to data objects** - Implementation layer must not reference any ScriptableObjects (data layer)
- **Only processor references** - Implementation layer may only reference processors
- **Data through processors** - Any data needed must be obtained by calling processor functions or accessing processor properties
- **No direct data access** - Never bypass processors to access ScriptableObjects directly
- **Respect namespace boundaries** - The `ScriptablesProcessorInfrastructure` namespace enforces that only processors should reference these objects; implementation layer code must not use `using ScriptablesProcessorInfrastructure`
- **Installer exemption** - Installer classes (SettingsInstaller, RuntimeInstaller) in `Core/Installers` are DI infrastructure and are exempt from the namespace restriction; they may use `using ScriptablesProcessorInfrastructure` to reference ScriptableObjects they install

## SettingsInstallers

**Purpose:** MonoBehaviour wrappers for IDataScriptable assets that implement IInstaller for dependency injection.

**Rules:**
- **Naming convention** - Must be named `[Name]SettingsInstaller` matching the Settings (e.g., `GridSettingsInstaller` for `GridSettings`)
- **File location** - Must be placed in `Assets/Scripts/Core/Installers/`
- **SceneScope placement** - Must be added to SceneScope in Unity scenes
- **IInstaller implementation** - Must implement `IInstaller` interface
- **MonoBehaviour inheritance** - Must inherit from `MonoBehaviour`
- **SerializeField field** - Must have `[SerializeField]` private field for the SettingsScriptable asset
- **Public property** - Must provide public property to expose the SettingsScriptable
- **InstallBindings** - `InstallBindings` must register `this` as singleton using `containerBuilder.AddSingleton(this)`
- **Asset reference** - SettingsScriptable assets are created in Unity Editor and referenced via inspector

## RuntimeData Template

RuntimeData ScriptableObjects store processor runtime state. Key rules:

### 1. **Namespace and Interface**
- Use `ScriptablesProcessorInfrastructure` namespace
- Implement `IRuntimeDataScriptable` interface (empty marker interface)
- Keep consistent with other ScriptableObject types

### 2. **CreateAssetMenu**
- Add `[CreateAssetMenu]` attribute for easy creation in Unity editor
- Set appropriate menu path under "Data/Runtime/"

### 3. **State Management**
- Use `[SerializeField]` for private fields to allow Unity serialization
- Expose state through public properties with both getters and setters
- Processors are the only code that will access RuntimeData and are meant to set the state

### 4. **Events**
- Define events for state changes or important occurrences
- Provide helper methods to invoke events (e.g., `InvokeOnSomethingHappened`)
- Keep event invocation logic encapsulated

### 5. **Initialization**
- Provide an `Initialize()` method to set default values
- Called by the processor when the RuntimeData is instantiated

### 6. **No Logic Beyond State**
- RuntimeData should only store state and manage events
- Business logic belongs in the processor, not RuntimeData

## Validation Checklist

### Data Layer (ScriptableObjects)

For each ScriptableObject in the project:

- [ ] **No complex methods** - The ScriptableObject contains only data fields, properties, and simple data access methods (3-5 lines max)
- [ ] **No processor references** - The ScriptableObject does not reference any processor classes
- [ ] **No ScriptableObject references** - The ScriptableObject does not reference other ScriptableObjects
- [ ] **No dependency injection** - The ScriptableObject does not use `[Inject]` attributes or any DI framework
- [ ] **Public properties with setters** - All private fields are exposed via public properties with both getters and setters
- [ ] **Correct namespace** - IDataScriptable and IRuntimeDataScriptable implementations use the `ScriptablesProcessorInfrastructure` namespace
- [ ] **RuntimeDatas have no installers** - RuntimeDatas DO NOT have installer classes; the processor instantiates and installs them directly on creation
- [ ] **No RuntimeInstaller files exist** - No `*RuntimeInstaller.cs` files exist in the project
- [ ] **Events declared but not invoked** - Events are declared but not invoked within ScriptableObjects (invocation happens in Processors)
- [ ] **RuntimeData follows template** - RuntimeData classes follow the RUNTIME_DATA_TEMPLATE.md structure with Initialize() method and event helpers

### Processor Layer

For each Processor in the project:

- [ ] **No data fields** - The Processor has no data fields or state variables (only injected ScriptableObjects)
- [ ] **Injected data objects only** - All fields are either `[Inject]` ScriptableObjects or primitive types/constants
- [ ] **Stateless** - The Processor does not maintain any state between method calls
- [ ] **Properties pass through data** - Any properties simply return data from injected ScriptableObjects
- [ ] **Logic in methods only** - All logic is implemented as methods, not in property getters/setters
- [ ] **Uses dependency injection** - The Processor uses `[Inject]` attributes for ScriptableObject dependencies
- [ ] **Implements IInstaller** - The Processor implements `IInstaller` and registers itself as singleton
- [ ] **Implements InjectRuntimeData signature** - The Processor implements `InjectRuntimeData(ContainerBuilder containerBuilder)`
- [ ] **InjectRuntimeData behavior** - If owning RuntimeData, instantiate/register there; otherwise keep a documented no-op
- [ ] **InstallBindings call flow** - `InstallBindings` registers `this` and calls `InjectRuntimeData(containerBuilder)`
- [ ] **No Awake or Start** - The Processor does not have Awake or Start methods (use Initialize() instead)
- [ ] **No OnEnable or OnDisable** - The Processor does not have OnEnable or OnDisable methods
- [ ] **No coroutines** - The Processor does not use coroutines
- [ ] **Legacy exclusion marked** - If processor has serialized scene/UI references, it includes a `TODO(Architecture)` note near class declaration

### Implementation Layer

For each MonoBehaviour, UI script, or game logic component:

- [ ] **No ScriptableObject references** - The component does not reference any ScriptableObjects directly
- [ ] **Only processor references** - The component only references processors via `[Inject]` or other means
- [ ] **Data through processors** - All data access goes through processor methods or properties
- [ ] **No bypassing processors** - The component never accesses ScriptableObjects directly
- [ ] **Processor-based actions** - All actions are performed by calling processor methods
- [ ] **No ScriptablesProcessorInfrastructure imports** - The component does not use `using ScriptablesProcessorInfrastructure`

### SettingsInstallers

For each IDataScriptable in the project:

- [ ] **SettingsInstaller exists** - Every IDataScriptable has a corresponding `[Name]SettingsInstaller` class
- [ ] **Correct naming** - SettingsInstaller is named `[Name]SettingsInstaller` matching the Settings
- [ ] **Correct location** - SettingsInstaller is placed in `Assets/Scripts/Core/Installers/`
- [ ] **Implements IInstaller** - SettingsInstaller implements `IInstaller` interface
- [ ] **Inherits MonoBehaviour** - SettingsInstaller inherits from `MonoBehaviour`
- [ ] **SerializeField field** - SettingsInstaller has `[SerializeField]` private field for the SettingsScriptable
- [ ] **Public property** - SettingsInstaller provides public property to expose the SettingsScriptable
- [ ] **InstallBindings registers this** - InstallBindings registers `this` as singleton using `containerBuilder.AddSingleton(this)`
- [ ] **Added to SceneScope** - SettingsInstaller is added to SceneScope in Unity scenes

### General Checks

- [ ] **Third-party libraries exempt** - Astar, Reflex, and other third-party libraries are exempt from these rules
- [ ] **Installer classes exempt** - Installer classes (SettingsInstaller) in `Core/Installers` are exempt as DI infrastructure; they may reference ScriptableObjects and use `ScriptablesProcessorInfrastructure` namespace
- [ ] **Data flow** - Data flows: Implementation → Processor → ScriptableObject (never the reverse)
- [ ] **Circular dependencies** - No circular dependencies between layers
- [ ] **Single responsibility** - Each class has a single, clear responsibility within its layer

## Quick Verification Commands

Run these commands to quickly verify architecture compliance:

```bash
# Check for processor references in ScriptableObjects (should return no results)
grep -r "Processor" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for [Inject] in ScriptableObjects (should return no results)
grep -r "\[Inject\]" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for correct namespace in ScriptableObjects (should be ScriptablesProcessorInfrastructure)
grep -r "namespace Scriptables" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for ScriptablesProcessorInfrastructure usage outside of Processors and Installers (should return NO results)
grep -r "using ScriptablesProcessorInfrastructure" Assets/Scripts/ --include="*.cs" | grep -v "Processors/" | grep -v "Core/Installers/"

# Check for RuntimeInstaller files (should return NO results)
find Assets/Scripts/ -name "*RuntimeInstaller.cs"
```

## Migration Pattern

When refactoring existing code to this architecture:

1. **Identify the ScriptableObject** - Create or use existing ScriptableObject for data storage
2. **Expose data as properties** - Add public properties with getters and setters for all private fields
3. **Remove methods from ScriptableObject** - Move all logic out of ScriptableObjects (keep only simple 3-5 line data access methods)
4. **Create or update Processor** - Ensure Processor has injected ScriptableObject references
5. **Move logic to Processor** - Implement all logic as methods in Processor
6. **Access data directly** - Processor methods should access ScriptableObject data via properties
7. **Update call sites** - Ensure all code calls Processor methods instead of ScriptableObject methods
8. **Verify no data references** - Ensure implementation layer has no direct ScriptableObject references
9. **Create SettingsInstaller** - For IDataScriptable, create corresponding SettingsInstaller in `Core/Installers/`
10. **Add to SceneScope** - Add SettingsInstaller to SceneScope in Unity scenes
