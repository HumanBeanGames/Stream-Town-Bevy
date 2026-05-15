# Stream Town Reloaded - Architecture Documentation

## Overview

This project uses a strict 3-layer architecture to separate concerns and maintain clean dependencies. The architecture ensures that data, logic, and implementation are properly isolated.

---

## Layer 1: Data Layer (ScriptableObjects)

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

**Examples of acceptable methods:**
```csharp
// Simple getter with safety check
public Node_SO GetGoalNode(Goal goal)
{
    if (_goalsFollowed.ContainsKey(goal))
        return _goalsFollowed[goal];
    return null;
}

// Simple setter with safety check
public void SetResourceAmount(Resource resource, int amount)
{
    if (_resources.ContainsKey(resource))
        _resources[resource].Amount = amount;
}

// Simple add with safety check
public void AddResource(Resource resource, int amount, bool triggerEvent = true)
{
    if (_resources.ContainsKey(resource))
    {
        _resources[resource].Amount += amount;
        if (triggerEvent)
            _onAnyResourceChangeEvent?.Invoke(resource, amount, true);
    }
}
```

**Examples:**
- `ResourceRuntimeData` - Stores resource lists (wood, ore, food, gold, recruits)
- `FoliageRuntimeData` - Stores foliage lists (on land, underwater)
- `BuildingSettings` - Stores building configuration (costs, max levels, ages)
- `BuildingRuntimeData` - Stores building runtime state (placers, buildings, counts, unlocked status)
- `AudioRuntimeData` - Stores audio handler queue
- `GameEventRuntimeData` - Stores game event state
- `PlayerRuntimeData` - Stores player state
- `UtilDisplayRuntimeData` - Stores UI display state
- `ObjectPoolingRuntimeData` - Stores object pooling state

---

## Layer 2: Processor Layer

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
- **No coroutines** - Processors must not use coroutines; coroutines inherently cause processors to have states. Use the `Process()` function for per-frame logic instead
- **No Awake or Start** - Processors must not have Awake or Start methods. All initialization logic must be in the `Initialize()` method
- **No OnEnable or OnDisable** - Processors must not have OnEnable or OnDisable methods

**Temporary Legacy Exclusion Policy:**
- Processors with serialized scene/UI references (e.g., `[SerializeField]` `Button`, `TextMeshProUGUI`, `GameObject`, `Image`) may be temporarily excluded from strict injected-only field conformance.
- Excluded files must include a standardized `TODO(Architecture)` note near the class declaration.
- Excluded processors MAY need migration to a non-processor pattern.

**Examples:**
- `ResourceProcessor` - Handles resource management logic
- `FoliageProcessor` - Handles foliage management logic
- `BuildingProcessor` - Handles building placement, removal, leveling logic
- `AudioSourcesProcessor` - Handles audio source processing logic
- `GameEventProcessor` - Handles game event logic
- `PlayerProcessor` - Handles player logic
- `UtilDisplayProcessor` - Handles UI display logic
- `ObjectPoolingProcessor` - Handles object pooling logic

**Processor Pattern:**
```csharp
public class ExampleProcessor : MonoBehaviour, IInstaller, IProcessor
{
    [Inject] private ExampleSettingsScriptable _settingsData;

    [Inject] private ExampleRuntimeData _runtimeData;

    public void InjectRuntimeData(ContainerBuilder containerBuilder)
    {
        // Instantiate and register RuntimeData singleton
        ExampleRuntimeData runtimeData = ScriptableObject.CreateInstance<ExampleRuntimeData>();
        containerBuilder.AddSingleton(runtimeData);
    }

    public void Initialize()
    {
        // Initialize data objects directly via properties
        _runtimeData.DataField = new DataType();
    }

    public void SomeLogic()
    {
        // Access data directly from RuntimeData
        var data = _runtimeData.DataField;

        // Perform logic
        // ...

        // Update data directly
        _runtimeData.DataField = newData;
    }

    public void InstallBindings(ContainerBuilder containerBuilder)
    {
        // Register processor singleton, then install runtime data
        containerBuilder.AddSingleton(this);
        InjectRuntimeData(containerBuilder);
    }
}
```

---

## Layer 3: Implementation Layer

**Purpose:** Use processors to interact with the system.

**Rules:**
- **No references to data objects** - Implementation layer must not reference any ScriptableObjects (data layer)
- **Only processor references** - Implementation layer may only reference processors
- **Data through processors** - Any data needed must be obtained by calling processor functions or accessing processor properties
- **No direct data access** - Never bypass processors to access ScriptableObjects directly
- **Respect namespace boundaries** - The `ScriptablesProcessorInfrastructure` namespace enforces that only processors should reference these objects; implementation layer code must not use `using ScriptablesProcessorInfrastructure`
- **Installer exemption** - Installer classes (SettingsInstaller, RuntimeInstaller) in `Data/Containers` are DI infrastructure and are exempt from the namespace restriction; they may use `using ScriptablesProcessorInfrastructure` to reference ScriptableObjects they install

**Examples:**
- MonoBehaviour components (building scripts, player controllers, etc.)
- UI scripts
- Save/Load systems
- Any game logic that isn't a processor

**Implementation Pattern:**
```csharp
public class ExampleImplementation : MonoBehaviour
{
    [Inject] private ExampleProcessor _exampleProcessor;

    public void DoSomething()
    {
        // Get data through processor
        var data = _exampleProcessor.GetData();
        
        // Use processor to perform actions
        _exampleProcessor.PerformAction(data);
    }
}
```

---

## Exemptions

The following third-party libraries are exempt from these architecture rules:
- **Astar** - Pathfinding library
- **Reflex** - Dependency injection framework
- Any other third-party libraries used in the project

---

## Migration Pattern

When refactoring existing code to this architecture:

1. **Identify the ScriptableObject** - Create or use existing ScriptableObject for data storage
2. **Expose data as properties** - Add public properties with getters and setters for all private fields
3. **Remove methods from ScriptableObject** - Move all logic out of ScriptableObjects
4. **Create or update Processor** - Ensure Processor has injected ScriptableObject references
5. **Move logic to Processor** - Implement all logic as methods in Processor
6. **Access data directly** - Processor methods should access ScriptableObject data via properties
7. **Update call sites** - Ensure all code calls Processor methods instead of ScriptableObject methods
8. **Verify no data references** - Ensure implementation layer has no direct ScriptableObject references

---

## Benefits

- **Separation of concerns** - Clear distinction between data, logic, and implementation
- **Testability** - Processors can be easily tested by mocking data objects
- **Dependency injection** - Clean dependency management via Reflex
- **Maintainability** - Changes to data structure don't affect logic, and vice versa
- **Statelessness** - Processors are stateless, making them thread-safe and predictable
- **Serialization** - ScriptableObjects are easily serializable for save/load systems

---

## Architecture Compliance Checklist

Use this checklist to verify that the codebase follows the 3-layer architecture rules.

### Data Layer (ScriptableObjects)

For each ScriptableObject in the project:

- [ ] **No methods or functions** - The ScriptableObject contains only data fields and properties
- [ ] **No processor references** - The ScriptableObject does not reference any processor classes
- [ ] **No ScriptableObject references** - The ScriptableObject does not reference other ScriptableObjects
- [ ] **No dependency injection** - The ScriptableObject does not use `[Inject]` attributes or any DI framework
- [ ] **Public properties with setters** - All private fields are exposed via public properties with both getters and setters
- [ ] **No logic** - The ScriptableObject contains no conditional logic, loops, or calculations
- [ ] **No events** - Events should be declared but not invoked within ScriptableObjects (invocation happens in Processors)
- [ ] **RuntimeDatas have no installers** - RuntimeDatas DO NOT have installer classes; the processor instantiates and installs them directly on creation
- [ ] **Correct namespace** - IDataScriptable and IRuntimeDataScriptable implementations use the `ScriptablesProcessorInfrastructure` namespace

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
- [ ] **No direct ScriptableObject references in implementation** - The Processor's public API does not expose ScriptableObjects directly
- [ ] **No Awake or Start** - The Processor does not have Awake or Start methods
- [ ] **No OnEnable or OnDisable** - The Processor does not have OnEnable or OnDisable methods (if temporarily present in legacy code, it must have a `TODO(Architecture)` marker for removal)
- [ ] **No coroutines** - The Processor does not use coroutines

### Implementation Layer

For each MonoBehaviour, UI script, or game logic component:

- [ ] **No ScriptableObject references** - The component does not reference any ScriptableObjects directly
- [ ] **Only processor references** - The component only references processors via `[Inject]` or other means
- [ ] **Data through processors** - All data access goes through processor methods or properties
- [ ] **No bypassing processors** - The component never accesses ScriptableObjects directly
- [ ] **Processor-based actions** - All actions are performed by calling processor methods
- [ ] **No ScriptablesProcessorInfrastructure imports** - The component does not use `using ScriptablesProcessorInfrastructure`

### General Checks

- [ ] **Third-party libraries exempt** - Astar, Reflex, and other third-party libraries are exempt from these rules
- [ ] **Installer classes exempt** - Installer classes (SettingsInstaller, RuntimeInstaller) in `Data/Containers` are exempt as DI infrastructure; they may reference ScriptableObjects and use `ScriptablesProcessorInfrastructure` namespace
- [ ] **Data flow** - Data flows: Implementation → Processor → ScriptableObject (never the reverse)
- [ ] **Circular dependencies** - No circular dependencies between layers
- [ ] **Single responsibility** - Each class has a single, clear responsibility within its layer

### Quick Verification Commands

To quickly verify architecture compliance:

```bash
# Check for methods in ScriptableObjects (should return minimal results)
grep -r "public void\|public bool\|public int" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for processor references in ScriptableObjects (should return no results)
grep -r "Processor" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for [Inject] in ScriptableObjects (should return no results)
grep -r "\[Inject\]" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for ScriptableObject references in Processors (should only be via [Inject])
grep -r "Scriptable" Assets/Scripts/Managers/ --include="*.cs" | grep -v "\[Inject\]"

# Check for correct namespace in ScriptableObjects (should be ScriptablesProcessorInfrastructure)
grep -r "namespace Scriptables" Assets/Scripts/Scriptables/ --include="*.cs"

# Check for ScriptablesProcessorInfrastructure usage outside of Processors and Installers (should return minimal results)
grep -r "using ScriptablesProcessorInfrastructure" Assets/Scripts/ --include="*.cs" | grep -v "Processors/" | grep -v "Data/Containers/"
```
