# RuntimeData Template

This document serves as a template for creating RuntimeData ScriptableObjects that store processor runtime state.

## RuntimeData Structure

```csharp
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
    /// <summary>
    /// ScriptableObject containing runtime state for ExampleProcessor.
    /// </summary>
    public class ExampleRuntimeData : ScriptableObject, IRuntimeDataScriptable
    {
        #region Public State
        
        /// <summary>
        /// Description of this field.
        /// </summary>
        [SerializeField] private int _someValue;
        public int SomeValue => _someValue;
        
        /// <summary>
        /// Description of this field.
        /// </summary>
        [SerializeField] private List<GameObject> _someList;
        public List<GameObject> SomeList => _someList;
        
        #endregion
        
        #region Events
        
        /// <summary>
        /// Event fired when something happens.
        /// </summary>
        public event Action OnSomethingHappened;
        
        #endregion
        
        #region Public Methods
        
        /// <summary>
        /// Initializes the runtime data.
        /// </summary>
        public void Initialize()
        {
            _someValue = 0;
            _someList = new List<GameObject>();
        }
        
        /// <summary>
        /// Invokes the OnSomethingHappened event.
        /// </summary>
        public void InvokeOnSomethingHappened()
        {
            OnSomethingHappened?.Invoke();
        }
        
        #endregion
    }
}
```

## Key Rules

### 1. **Namespace and Interface**
- Use `ScriptablesProcessorInfrastructure` namespace (the protected namespace for runtime data ScriptableObjects)
- Implement `IRuntimeDataScriptable` interface (empty marker interface used by Coordinator for reflection)
- Keep consistent with other ScriptableObject types

### 2. **CreateAssetMenu**
- Add `[CreateAssetMenu]` attribute for easy creation in Unity editor
- Set appropriate menu path under "Data/Runtime/"

### 3. **State Management**
- Use `[SerializeField]` for private fields to allow Unity serialization
- Expose state through public properties (get-only)
- Never expose public set accessors to state

### 4. **Events**
- Define events for state changes or important occurrences
- Provide helper methods to invoke events (e.g., `InvokeOnSomethingHappened`)
- This keeps event invocation logic encapsulated

### 5. **Initialization**
- Provide an `Initialize()` method to set default values
- This is called by the processor when the RuntimeData is instantiated

### 6. **No Logic Beyond State**
- RuntimeData should only store state and manage events
- Business logic belongs in the processor, not RuntimeData
- Keep RuntimeData simple and state-focused

## Common Patterns

### Boolean Flags
```csharp
[SerializeField] private bool _isReady;
public bool IsReady => _isReady;
```

### Counters
```csharp
[SerializeField] private int _count;
public int Count => _count;
```

### Collections
```csharp
[SerializeField] private List<GameObject> _objects;
public List<GameObject> Objects => _objects;

[SerializeField] private Dictionary<string, GameObject> _dictionary;
public Dictionary<string, GameObject> Dictionary => _dictionary;
```

### Events with Parameters
```csharp
public event Action<int> OnValueChanged;

public void InvokeOnValueChanged(int value)
{
    OnValueChanged?.Invoke(value);
}
```

### Complex State Objects
```csharp
[SerializeField] private SomeState _state;
public SomeState State => _state;
```

## Checklist for New RuntimeData

- [ ] Use `ScriptablesProcessorInfrastructure` namespace
- [ ] Implement `IRuntimeDataScriptable` interface
- [ ] Use `[SerializeField]` for private fields
- [ ] Expose state through public properties
- [ ] Provide `Initialize()` method
- [ ] Define events for state changes
- [ ] Provide helper methods to invoke events
- [ ] Add XML comments to all public members
- [ ] Keep logic minimal (state only)
