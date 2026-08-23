## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

## visual regression ledgers

Two recurring visual regressions have persistent evidence ledgers:

- Character animation: `bevy-port/docs/visual-regressions/character-animation.md`
- Tree and foliage flicker: `bevy-port/docs/visual-regressions/tree-foliage-flicker.md`

Before changing either system, read its ledger completely. Do not retry an item marked failed unless a changed precondition or new measurement is recorded. Keep successful narrow fixes intact while isolating the unresolved symptom.

After every attempt, append the commit, one-variable hypothesis, fixed reproduction inputs, objective runtime measurements, capture path, and user-visible outcome. Automated tests, attachment logs, or still screenshots may support an attempt, but only a moving runtime check and user confirmation can close either visual regression.
