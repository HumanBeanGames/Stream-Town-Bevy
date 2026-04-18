# TODO List

## High Priority

- [ ] Convert coroutines in processors to use Process() function instead
- [ ] Remove OnEnable and OnDisable methods from processors
- [ ] RuntimeDatas DO NOT get installers; processors instantiate and install them on creation
- [ ] Add TODO markers in processor files for each remaining `OnEnable`/`OnDisable` and coroutine usage (`IEnumerator`/`StartCoroutine`)
- [ ] Excluded processors with serialized scene/UI refs should include standardized architecture TODO note and MAY need migration to non-processor pattern

## Medium Priority

- [ ] Consider splitting GameEventProcessor
- [ ] Investigate EventProcessor; not sure this is the correct approach for it
- [ ] Check for processors over 200 lines; they may be prime for cleaning or splitting

## Excluded Processor Files (serialized scene/UI refs)

- `Assets/Scripts/UserInterface/Menus/MainMenuProcessor.cs`
- `Assets/Scripts/UserInterface/Menus/LoadingProcessor.cs`
