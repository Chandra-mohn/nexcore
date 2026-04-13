# NexStudio Query UI Spec

**Date**: 2026-03-27
**Status**: Design
**Depends on**: cobol-intel crate (DONE), NexStudio NS13+ baseline

---

## 1. Overview

NexQuery integration into NexStudio. The query UI follows the same pattern as
the Data Explorer -- launched from a menu, opens as editor tabs, multiple
instances supported. Saved queries appear in a new "Queries" explorer section.

**Principle**: UI is a thin presentation layer. All intelligence logic lives in
the cobol2rust CLI. NexStudio calls `cobol2rust query` and renders the JSON.

---

## 2. Explorer: Queries Section

New sixth section in the project explorer, after Data:

```
Explorer
  [Source]     -> {source.path}
  [Target]     -> {target.path}
  [Reports]    -> reports/
  [Data]       -> {data.path}
  [Queries]    -> {queries.path}   <-- NEW
```

### Behavior

- Maps to `nexintel/queries/` folder (configurable in nexstudio.json)
- Shows .nxq files as a flat list, sorted alphabetically
- Click a file -> opens query editor tab with that file loaded
- Right-click context menu: Run, Rename, Delete, Open in Editor
- Empty state: "No saved queries. Intel > New Query to create one."

### Project Config

```json
{
  "source": { "path": "app/cbl" },
  "target": { "path": "target" },
  "data": { "path": "data" },
  "queries": { "path": "nexintel/queries" },
  "intel": {
    "graph_path": "nexintel/graph.nxg",
    "license_path": "nexintel/.license"
  }
}
```

---

## 3. Intel Menu

New top-level menu between Run and Help:

```
Intel
  New Query              Cmd+Shift+Q
  --------
  Build Graph
  Rebuild Graph
  --------
  Graph Info
```

### New Query (Cmd+Shift+Q)

Opens a new untitled query editor tab. On first save, defaults to
`nexintel/queries/` directory. File extension: `.nxq`.

### Build Graph

Runs:
```
cobol2rust build-graph \
  -i {source.path} \
  -o {intel.graph_path} \
  -l $(cat {intel.license_path}) \
  --pattern "**/*.cbl"
```

Shows progress in status bar. On completion, updates Graph Info stats.

### Rebuild Graph

Same as Build Graph but skips the "graph already exists" check.
Auto-triggered after pipeline runs (transpile/rustify) if a .nxg file exists.

### Graph Info

Small modal dialog:
```
+----------------------------------+
| Code Intelligence Graph          |
|                                  |
| Nodes:      1,173                |
| Edges:        414                |
| Programs:      45                |
| Paragraphs:    25                |
| Fields:     1,001                |
| Copybooks:     65                |
|                                  |
| Built: 2026-03-27 14:30         |
| File:  nexintel/graph.nxg       |
| Size:  387 KB                    |
|                                  |
|              [OK]                |
+----------------------------------+
```

Reads node counts from `cobol2rust query -g ... -l ... -e "programs;"` etc.,
or from a cached stats file written during build.

---

## 4. Query Editor Tab

Each .nxq file (or new untitled query) opens as an editor tab. The tab has
two vertically split panes: editor (top) and results (bottom).

### 4.1 Editor Pane (Top)

Monaco editor instance with NexQuery language support:

- **Language ID**: `nexquery`
- **Syntax highlighting**:
  - Keywords (blue): trace, rank, similar, find-dead, deps, impact, compare, save, run
  - Node types (green): programs, paragraphs, fields, copybooks, files, tables, rules
  - Traversal verbs (purple): calling, called-by, using, used-by, etc.
  - Modifiers (orange): by, top, in, with, depth, level, order, scope, threshold, as
  - Comments (gray): -- ...
  - Strings (red): 'quoted'
  - Numbers (cyan): 42, 3.14
  - Operators: =, !=, >, <, ~, ~~, in, has, and, or, not
  - Special: each (italic)
  - Punctuation: ; [] ()

- **Autocomplete** (future -- not v1):
  - Program names from .nxg graph
  - Copybook names, field names
  - Keyword completion

- **Run**: Ctrl+Enter executes the statement under the cursor (or the whole buffer)

### 4.2 Results Pane (Bottom)

Displays the JSON output from `cobol2rust query -e "..."`.

**JSON Tree View** (v1 -- only view):
- Pretty-printed JSON with syntax coloring
- Collapsible/expandable nodes (objects and arrays)
- Click to expand/collapse
- Copy button (copies JSON to clipboard)
- Read-only Monaco instance with JSON language, or a tree widget

**Table View** (future):
- For rank/list results, show as sortable table
- Toggle button: [JSON | Table]

**Graph View** (future):
- For trace/deps results, show as Svelte Flow graph
- Toggle button: [JSON | Table | Graph]

### 4.3 Toolbar (Between Editor and Results)

```
[Run (Ctrl+Enter)]  [Save (Cmd+S)]  |  Nodes: 1,173  Edges: 414  |  0.03s
```

- **Run**: execute the query
- **Save**: save the .nxq file
- Node/edge count from the loaded graph
- Execution time of the last query

---

## 5. License Key Management

Stored as a plain text file at `nexintel/.license` (one line, the key string).

- On first "Build Graph" or "New Query", if the file doesn't exist, prompt:
  "Enter your NexIntel license key" with a text input dialog
- Key is saved to `nexintel/.license`
- `.license` should be in .gitignore
- Future: proper license validation, expiry, tier gating

---

## 6. Auto-Rebuild

When the NexMig pipeline completes (transpile/rustify/DSL), if a .nxg file
already exists, automatically rebuild the graph:

```
Pipeline complete -> check nexintel/graph.nxg exists -> rebuild in background
```

Status bar shows: "Rebuilding code intelligence graph..."
On completion: "Graph rebuilt: 1,173 nodes, 414 edges"

If no .nxg exists, don't auto-build. The user must explicitly Build Graph first.

---

## 7. File Structure

```
project/
  nexstudio.json          -- project config (includes queries + intel paths)
  app/cbl/                -- COBOL source
  target/                 -- transpiled output
  nexintel/
    graph.nxg             -- encrypted code intelligence graph
    .license              -- license key (gitignored)
    queries/
      clearing-blast.nxq  -- saved queries
      dead-code-scan.nxq
      complexity-top20.nxq
```

---

## 8. Tauri Sidecar Commands

All intelligence operations go through CLI sidecar calls:

```typescript
// Build graph
await invoke('run_sidecar', {
  args: ['build-graph', '-i', sourcePath, '-o', graphPath, '-l', licenseKey]
});

// Execute query
const result = await invoke('run_sidecar', {
  args: ['query', '-g', graphPath, '-l', licenseKey, '-e', queryText]
});
// result is JSON string -> parse and render

// Graph stats (reuse query with simple expand)
const stats = await invoke('run_sidecar', {
  args: ['query', '-g', graphPath, '-l', licenseKey, '-e', 'programs;']
});
```

No new Tauri commands needed beyond the existing `run_sidecar` pattern.

---

## 9. Implementation Sessions

### NQ1: Monaco NexQuery Language Support
- Register `nexquery` language in Monaco
- Token provider for syntax highlighting
- File association: .nxq -> nexquery

### NQ2: Query Editor Tab Component
- QueryEditor.svelte: split pane (editor + results)
- Wire Ctrl+Enter to execute via sidecar
- JSON tree results display (read-only Monaco with JSON)
- Toolbar with Run, Save, stats

### NQ3: Explorer Queries Section + Intel Menu
- Add Queries section to explorer (file-backed, like Source)
- Add Intel menu with New Query, Build Graph, Graph Info
- Wire Build Graph to sidecar
- License key prompt dialog

### NQ4: Auto-Rebuild + Polish
- Auto-rebuild after pipeline completion
- Status bar integration
- Graph Info modal
- .license gitignore handling
- Tab title: filename or "Untitled Query"

### Total: ~4 sessions
