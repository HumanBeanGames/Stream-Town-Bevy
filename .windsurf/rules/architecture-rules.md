---
trigger: manual
---

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
- **RuntimeData is data-only** - RuntimeData classes must not store processors, command handlers, service objects, scene objects, or other composed behavior references
- **No injections** - ScriptableObjects must not use `[Inject]` attributes or any dependency injection
- **Public properties with setters** - All private data fields must be exposed via public properties with getters and setters
- **Namespace: ScriptablesProcessorInfrastructure for IDataScriptable** - IDataScriptable implementations must use the `ScriptablesProcessorInfrastructure` namespace to enforce access control
- **Namespace: Processors for IRuntimeDataScriptable** - IRuntimeDataScriptable implementations must use the `Processors` namespace to allow access from implementation layer and other processors
- **Registered in SceneScope** - ScriptableObjects are loaded in the Scene Scope on a scene-by-scene basis as needed
- **RuntimeDatas have no installers** - RuntimeDatas DO NOT get installer classes. The processor instantiates and installs the RuntimeData in the InjectRuntimeData() call during the binding/installation phase
- **RuntimeInstaller prohibition** - IRuntimeDataScriptable types must NOT have dedicated installer classes. No `*RuntimeInstaller.cs` files should exist in the project.
- **No dependency injection into RuntimeData** - RuntimeData must not require processor, service, command, or scene-object dependencies; processors compose behavior separately

### Layer 2: Processor Layer
**Purpose:** Contain logic only.

**Rules:**
- **No data fields** - Processors must not have any data fields or state
- **Injected data objects only** - Processors may only have fields that are injected ScriptableObjects (data layer)
- **No state** - Processors must be stateless; all state must be stored in injected ScriptableObjects
- **Exception: Local runtime data reference** - Processors may have a private field for their own RuntimeData without `[Inject]` if they instantiate and bind it in `InjectRuntimeData()`. This is necessary because when processors use `AddSingleton(this)`, Reflex doesn't auto-inject fields on the manually-added instance. The processor must assign the field directly after creating the RuntimeData and bind it to the container for other components to inject.
- **Properties are allowed** - Properties are acceptable if they simply pass through data from injected objects
- **Functions only** - All logic must be implemented as methods
- **Dependency injection** - Processors use `[Inject]` attributes to receive data objects
- **IInstaller implementation** - Processors implement `IInstaller` to register themselves as singletons
- **InjectRuntimeData method** - Processors implement `InjectRuntimeData(ContainerBuilder containerBuilder)`; processors that manage RuntimeData instantiate and register it there, processors without RuntimeData implement a documented no-op
- **Install-before-initialize** - RuntimeData must be installed in `InjectRuntimeData()` before `Initialize()` executes
- **Initialize never allocates RuntimeData** - `Initialize()` may populate installed runtime data, but must not first create it
- **Behavior composition stays out of RuntimeData** - Command dictionaries, senders, and other service objects must be owned by processors or processor-owned services
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
- **No references to IDataScriptable** - Implementation layer must not reference IDataScriptable (settings ScriptableObjects)
- **May reference IRuntimeDataScriptable** - Implementation layer may reference IRuntimeDataScriptable classes in the `Processors` namespace for direct data access when needed
- **Processor references preferred** - Implementation layer should prefer referencing processors for logic, but may directly access RuntimeData for data reading
- **Data through processors preferred** - Data should generally be obtained by calling processor functions or accessing processor properties, but direct RuntimeData access is allowed for performance or convenience
- **Respect namespace boundaries** - The `ScriptablesProcessorInfrastructure` namespace enforces that only processors and installers should reference IDataScriptable; implementation layer code must not use `using ScriptablesProcessorInfrastructure` except for installer classes
- **Installer exemption** - Installer classes (SettingsInstaller) in `Core/Installers` are DI infrastructure and are exempt from the namespace restriction; they may use `using ScriptablesProcessorInfrastructure` to reference IDataScriptable they install

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
- Use `Processors` namespace (not ScriptablesProcessorInfrastructure)
- Implement `IRuntimeDataScriptable` interface (empty marker interface)
- This allows implementation layer and other processors to access RuntimeData directly

### 2. **CreateAssetMenu**
- Add `[CreateAssetMenu]` attribute for easy creation in Unity editor
- Set appropriate menu path under "Data/Runtime/"

### 3. **State Management**
- Use `[SerializeField]` for private fields to allow Unity serialization
- Expose state through public properties with both getters and setters
- RuntimeData is accessible from implementation layer and other processors

### 4. **Events**
- Define events for state changes or important occurrences
- Provide helper methods to invoke events (e.g., `InvokeOnSomethingHappened`)
- Keep event invocation logic encapsulated

### 5. **Initialization**
- Provide an `Initialize()` method to set default values
- Called by the processor after the RuntimeData has been instantiated and installed
- RuntimeData constructors should remain data-only and dependency-free

### 6. **Dependencies**
- RuntimeData must not require processor, service, or command dependencies
- Processors may compose behavior/services separately, but RuntimeData remains a plain state container
- Example: `new TwitchChatRuntimeData()`

### 7. **No Logic Beyond State**
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
- [ ] **Correct namespace for IDataScriptable** - IDataScriptable implementations use the `ScriptablesProcessorInfrastructure` namespace
- [ ] **Correct namespace for IRuntimeDataScriptable** - IRuntimeDataScriptable implementations use the `Processors` namespace
- [ ] **RuntimeDatas have no installers** - RuntimeDatas DO NOT have installer classes; the processor instantiates and installs them directly on creation
- [ ] **No RuntimeInstaller files exist** - No `*RuntimeInstaller.cs` files exist in the project
- [ ] **Events declared but not invoked** - Events are declared but not invoked within ScriptableObjects (invocation happens in Processors)
- [ ] **RuntimeData follows template** - RuntimeData classes follow the RUNTIME_DATA_TEMPLATE.md structure with Initialize() method and event helpers
- [ ] **RuntimeData is data-only** - RuntimeData does not hold processors, command handlers, service objects, scene objects, or behavior composition
- [ ] **RuntimeData has no behavior dependencies** - RuntimeData constructors do not take processors, commands, services, or scene objects

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
- [ ] **InjectRuntimeData behavior** - If owning RuntimeData, instantiate/register there before initialization; otherwise keep a documented no-op
- [ ] **Initialize does not allocate RuntimeData** - `Initialize()` does not first create runtime data
- [ ] **Behavior composition not stored in RuntimeData** - Processor-owned command/services are composed outside RuntimeData
- [ ] **InstallBindings call flow** - `InstallBindings` registers `this` and calls `InjectRuntimeData(containerBuilder)`
- [ ] **No Awake or Start** - The Processor does not have Awake or Start methods (use Initialize() instead)
- [ ] **No OnEnable or OnDisable** - The Processor does not have OnEnable or OnDisable methods
- [ ] **No coroutines** - The Processor does not use coroutines
- [ ] **Legacy exclusion marked** - If processor has serialized scene/UI references, it includes a `TODO(Architecture)` note near class declaration

### Implementation Layer

For each MonoBehaviour, UI script, or game logic component:

- [ ] **No IDataScriptable references** - The component does not reference IDataScriptable (settings ScriptableObjects) directly
- [ ] **May reference IRuntimeDataScriptable** - The component may reference IRuntimeDataScriptable classes in `Processors` namespace for direct data access
- [ ] **Processor references preferred** - The component should prefer referencing processors for logic
- [ ] **Data through processors preferred** - Data should generally be obtained through processor methods or properties, but direct RuntimeData access is allowed
- [ ] **No ScriptablesProcessorInfrastructure imports** - The component does not use `using ScriptablesProcessorInfrastructure` except for installer classes
- [ ] **May use Processors namespace** - The component may use `using Processors` to access RuntimeData classes

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

# Check for correct namespace in IDataScriptable (should be ScriptablesProcessorInfrastructure)
grep -r "namespace ScriptablesProcessorInfrastructure" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for correct namespace in IRuntimeDataScriptable (should be Processors)
grep -r "namespace Processors" Assets/Scripts/Scriptables/ --include="*.cs" | grep -i "runtime"

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
