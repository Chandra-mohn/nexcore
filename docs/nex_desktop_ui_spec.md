# NexStudio -- Technical Specification

**Status**: DECIDED
**Date**: 2026-03-22
**Product**: NexStudio -- "The COBOL modernization workbench"
**Repo**: nex-studio
**Purpose**: Define the desktop UI technology stack and architecture for NexStudio

---

## Decision Summary

**NexStudio** is a standalone desktop application using **Tauri v2** (Rust backend
shell) with **Svelte 5** + **shadcn-svelte** frontend, styled to closely reproduce
the JetBrains RustRover IDE experience.

---

## Decision Rationale

### Why Standalone (not IDE plugin)?
- NexStudio is a product, not a plugin in someone else's product
- NexMod's Architecture Canvas and Design Surface require full UX control
- Nexflow already has a VS Code plugin -- no duplication needed
- Standalone enables the PM/executive web dashboard (same Svelte skills)

### Why Tauri (not Electron, not pure Rust UI)?
- Rust backend: direct FFI to cobol2rust crates, no serialization overhead
- System WebView: smaller binary than Electron (~10 MB vs ~150 MB)
- Native worker pool via Tokio for background parsing/transpiling
- Memory-mapped file I/O for monster files (1M+ lines)
- Cross-platform: Windows, macOS, Linux
- VS Code existence proof: WebView + native backend works at IDE scale

### Why Svelte 5 (not React, not SolidJS)?
- Performance-first: Svelte 5 runes provide fine-grained reactivity, no virtual DOM
- Smallest bundle (~2 KB base) -- lighter than React (~42 KB) and SolidJS (~7 KB)
- Svelte Flow (35K stars, by xyflow) for node-based graph visualization
- svelte-dnd-action (2K stars) for drag-and-drop
- shadcn-svelte (8.5K stars) provides the closest visual match to JetBrains IDEs
- Larger ecosystem than SolidJS (~2x community, more component options)
- SvelteKit reuse for web dashboard (same frontend skills)

### Why shadcn-svelte (not Skeleton, not Flowbite)?
- Copy-paste ownership: full control over every component, essential for IDE-class UX
- Design language closest to JetBrains Darcula (clean borders, compact spacing, dark theme)
- Built on Bits UI (headless, accessible primitives) + Tailwind CSS
- shadcn-svelte-extras provides Tree View component (critical for file browser)
- CSS variables for theming -- trivially tunable to match Darcula palette

---

## Technology Stack

```
LAYER           TECHNOLOGY              VERSION     PURPOSE
-----           ----------              -------     -------
Shell           Tauri                   v2.x        Native window, menus, file dialogs, IPC
Frontend        Svelte                  v5          UI framework (runes reactivity)
Language        TypeScript              v5.x        Type safety
Components      shadcn-svelte           v1.x        Tabs, dialogs, menus, tooltips, badges
                shadcn-svelte-extras    latest      Tree view, extended components
Headless        Bits UI                 latest      Accessible primitives (foundation for shadcn)
Layout          PaneForge               v1.x        Resizable panel system (IDE layout)
Code Editor     Monaco Editor           latest      Code viewing, DSL editing, diff viewer
Graphs          Svelte Flow             latest      Lineage diagrams, pipeline visualization
                D3.js                   v7          Cluster maps, custom charts
DnD             svelte-dnd-action       latest      Tab reordering, panel dragging
Toasts          svelte-sonner           v1.x        Notifications
Styling         Tailwind CSS            v4          Utility classes, dark theme
Icons           Lucide                  latest      UI icons (shadcn default)
                Custom file icons       --          COBOL, Rust, Java, TOML, DSL file types
Backend         cobol2rust crates       --          Parser, transpiler, rustify, DSL emitters
Async           Tokio                   v1.x        Background workers, file I/O
IPC             Tauri Commands          v2          Frontend <-> Rust bridge
```

---

## UI Layout Specification

Target: Reproduce JetBrains RustRover layout as closely as possible.

```
+--+------------------+----------------------------------+------------------+--+
|  |                  |          EDITOR TABS              |                  |  |
|  |   LEFT PANEL     +----------------------------------+   RIGHT PANEL    |  |
|T |                  |                                  |                  |T |
|O |   Project tree   |       CODE EDITOR                |   Cargo/Crate    |O |
|O |   File browser   |       (Monaco)                   |   tree           |O |
|L |   Data explorer  |       - Line numbers             |   Data views     |L |
|  |                  |       - Syntax highlighting      |   Properties     |  |
|B |                  |       - Gutter decorations        |                  |B |
|A |                  |       - Minimap                   |                  |A |
|R |                  |       - Virtual scrolling         |                  |R |
|  |                  |                                  |                  |  |
|  +------------------+----------------------------------+------------------+  |
|  |                    BOTTOM PANEL                                        |  |
|  |   Problems | Output | Terminal | Lineage | Search                     |  |
|  |   - Tab bar with count badges                                         |  |
|  |   - Resizable (drag border up/down)                                   |  |
|  +-----------------------------------------------------------------------+  |
|  |   STATUS BAR                                                          |  |
|  |   breadcrumb | file info | encoding | git branch | progress           |  |
|  +-----------------------------------------------------------------------+  |
+------------------------------------------------------------------------------+
```

### Panel System (PaneForge)

```
Root PaneGroup (horizontal)
  |-- Left Panel (collapsible, default 250px)
  |     |-- Tabs: Project, Data, Lineage
  |     +-- Content varies by tab
  |
  |-- Center PaneGroup (vertical, fills remaining)
  |     |-- Editor Area (grows)
  |     |     |-- Tab bar (closeable, reorderable, overflow scroll)
  |     |     +-- Monaco Editor instance(s)
  |     |
  |     +-- Bottom Panel (collapsible, default 200px)
  |           |-- Tabs: Problems, Output, Terminal, Search
  |           +-- Content varies by tab
  |
  +-- Right Panel (collapsible, default 250px, hidden by default)
        |-- Tabs: Structure, Cargo, Properties
        +-- Content varies by tab
```

### Tool Bar (Left/Right Icon Strips)

```
LEFT TOOL BAR (vertical icon strip):
  - Project (file tree toggle)
  - Structure (code structure toggle)
  - Data (data explorer toggle)
  - Lineage (graph toggle)
  - Git (version control toggle)
  - Search (global search)

RIGHT TOOL BAR (vertical icon strip):
  - Cargo/Build (build panel toggle)
  - Properties (file properties toggle)
  - Notifications (notification panel toggle)

Each icon toggles its corresponding panel open/closed.
```

---

## Component Mapping

### shadcn-svelte Components Used

| UI Element | shadcn Component | Customization |
|---|---|---|
| File/crate tree | Tree View (extras) | File type icons, expand/collapse, selection |
| Editor tabs | Tabs | Close button, modified indicator, reorder via DnD |
| Bottom panel tabs | Tabs + Badge | Count badges (e.g., "37 problems") |
| Right-click menus | Context Menu | File ops, editor ops, tree ops |
| Toolbar dropdowns | Dropdown Menu | File, Edit, View, Navigate, Code, etc. |
| Search/command | Command | Ctrl+Shift+P command palette |
| Dialogs | Dialog | Settings, migration config, confirmations |
| Problem list | Table | Sortable, filterable, with severity icons |
| Tooltips | Tooltip | Toolbar icon descriptions |
| Breadcrumbs | Breadcrumb | File path navigation |
| Scroll areas | Scroll Area | Custom scrollbars matching theme |
| Badges | Badge | Tab counts, status indicators |
| Inputs | Input | Search fields, config forms |
| Buttons | Button | Toolbar actions, dialog actions |
| Separators | Separator | Panel dividers, menu dividers |
| Toggle | Toggle | Settings switches |

### Custom Components (built on shadcn primitives)

| Component | Foundation | Description |
|---|---|---|
| ToolBar (icon strip) | Tailwind flex column | Vertical icon strip (left/right edges) |
| StatusBar | Tailwind flex row | Bottom status: breadcrumb, encoding, git |
| EditorTabBar | shadcn Tabs + DnD | Closeable, reorderable, overflow, modified dot |
| FileTypeIcon | Lucide + custom SVGs | Icons for .cbl, .rs, .java, .toml, .schema, etc. |
| ProblemRow | shadcn Table row | Severity icon + message + file + line |
| SplitEditor | PaneForge + Monaco | Side-by-side COBOL vs Rust/Java |
| ProgressPanel | shadcn Progress + custom | Migration/analysis progress with log streaming |

---

## Theming: JetBrains Darcula Match

```css
/* CSS variables to match RustRover Darcula theme */
:root {
  /* Backgrounds */
  --background:          hsl(225 6% 13%);    /* #1E1F22 - editor bg */
  --card:                hsl(225 5% 18%);    /* #2B2D30 - panel bg */
  --popover:             hsl(225 6% 20%);    /* popup/menu bg */

  /* Foreground */
  --foreground:          hsl(225 5% 75%);    /* #BCBEC4 - primary text */
  --muted-foreground:    hsl(225 5% 55%);    /* secondary text */

  /* Borders */
  --border:              hsl(225 5% 25%);    /* #3C3F41 - subtle borders */

  /* Selection/Accent */
  --accent:              hsl(215 60% 32%);   /* #214283 - selection */
  --primary:             hsl(215 70% 50%);   /* #4A86C8 - links, active */

  /* Status colors */
  --destructive:         hsl(0 70% 50%);     /* errors */
  --warning:             hsl(40 90% 55%);    /* warnings */
  --success:             hsl(120 40% 50%);   /* success/checkmarks */

  /* Radius */
  --radius:              0.25rem;            /* JetBrains uses minimal radius */
}
```

---

## Tauri Backend Architecture

```
tauri::App
  |
  |-- Commands (IPC endpoints called from Svelte frontend)
  |     |-- project::open_project(path) -> ProjectInfo
  |     |-- project::list_files(path, pattern) -> FileTree
  |     |-- file::read_file(path) -> FileContent
  |     |-- file::read_file_range(path, start, end) -> FileContent
  |     |-- parse::parse_cobol(path) -> ParseResult
  |     |-- parse::parse_cobol_batch(paths) -> Vec<ParseResult>
  |     |-- transpile::transpile_file(path, target) -> TranspileResult
  |     |-- transpile::transpile_batch(paths, target, on_progress) -> BatchResult
  |     |-- rustify::rustify_file(path, tiers) -> RustifyResult
  |     |-- dsl::emit_schema(path) -> SchemaResult
  |     |-- dsl::emit_rules(path) -> RulesResult
  |     |-- lineage::trace(program, direction) -> LineageGraph
  |     |-- cluster::analyze(paths) -> ClusterMap
  |     |-- search::search_codebase(query, options) -> SearchResults
  |
  |-- Events (Rust -> Svelte, streamed)
  |     |-- progress::batch_progress { done, total, current_file }
  |     |-- parse::diagnostics { file, errors, warnings }
  |     |-- file::watch_changed { path, kind }
  |
  |-- State (managed by Tauri)
  |     |-- ProjectState { root_path, file_index, parsed_cache }
  |     |-- WorkerPool { tokio::Runtime, job_queue }
  |     |-- FileWatcher { notify::Watcher }
  |
  |-- Plugins
        |-- tauri-plugin-dialog (file/folder picker)
        |-- tauri-plugin-fs (file system access)
        |-- tauri-plugin-shell (git commands)
        |-- tauri-plugin-store (user preferences)
        |-- tauri-plugin-updater (auto-update)
        |-- tauri-plugin-notification (OS notifications)
```

---

## Phase Plan

### Phase 1: NexStudio Foundation (NexIntel)

**Goal**: Standalone COBOL exploration tool. Proves the entire stack.
**Repo**: /Users/chandramohn/workspace/nexstudio

```
NS1 -- Project Scaffolding (DONE)
  - Tauri v2 project scaffolding
  - Svelte 5 + Tailwind v4 setup
  - IDE shell: TitleBar, ToolBar, WorkArea, StatusBar
  - Darcula dark theme CSS variables
  - Tauri commands: greet, get_version
  - Frontend builds: 37 KB JS + 11 KB CSS

NS2 -- IDE Layout (Week 1-2)
  - Tauri v2 project scaffolding
  - Svelte 5 + shadcn-svelte + Tailwind v4 setup
  - PaneForge IDE layout (left, center, right, bottom panels)
  - Tool bar icon strips (left/right)
  - Status bar
  - Dark theme (Darcula match)
  - Tauri commands: open_project, list_files

NS3 -- File Browser (Week 2-3)
  - shadcn-extras Tree View for file browser
  - File type icons (COBOL, Rust, copybook, etc.)
  - File reading via Tauri commands (with range support for large files)

NS4 -- Code Editor (Week 3-4)
  - Monaco Editor Svelte 5 wrapper
  - Syntax highlighting for COBOL (custom Monarch tokenizer)
  - Editor tabs (open, close, switch)

NS5 -- Editor Polish (Week 4-5)
  - Modified indicator, tab reorder via DnD
  - Breadcrumb navigation
  - Range reading for large files
  - Minimap, scrollbar marks

NS6 -- Data Explorer (Week 5-6)
  - Copybook hierarchy tree view
  - Field list with types, sizes, levels
  - Usage cross-reference (where is this field used?)
  - TanStack Table (via table-core) for field data display
  - Search/filter within data structures

NS7 -- Lineage Graph (Week 7)
  - Svelte Flow integration for lineage graph
  - Node types: programs, files, copybooks, data items
  - Edge types: calls, reads, writes, copies
  - Zoom, pan, click-to-navigate

NS8 -- Cluster Visualization (Week 8)
  - D3 cluster map (program groupings)
  - Progressive disclosure (zoom levels)

NS9 -- Search + Command Palette (Week 9)
  - Global search (Ctrl+Shift+F) via Tauri command
  - Command palette (Ctrl+Shift+P) via Bits UI Command
  - Context menus (right-click on tree, editor, tabs)

NS10 -- Keyboard Shortcuts + Settings + Polish (Week 10)
  - Keyboard shortcuts system
  - Settings dialog (theme, editor preferences)
  - Error handling, loading states, empty states
  - Phase 1 integration testing
```

### Phase 2: NexStudio Migration (NexMig)

```
Milestone 2.1 -- Migration Configuration
  - Migration wizard dialog
  - Source/target selection
  - Options (Rust vs Java, rustify tiers, etc.)

Milestone 2.2 -- Migration Execution
  - Background batch transpilation via Tauri worker pool
  - Progress panel with streaming logs
  - File-by-file status (pending/running/done/failed)

Milestone 2.3 -- Code Review
  - Side-by-side diff view (COBOL left, Rust/Java right)
  - Monaco DiffEditor integration
  - Navigate between changes
  - Rustify before/after view

Milestone 2.4 -- Build Output
  - Project structure tree (generated Cargo.toml / pom.xml)
  - Build log viewer
  - Test runner output
```

### Phase 3: NexStudio Modernization (NexMod)

```
Milestone 3.1 -- DSL Editor
  - Monaco with Nexflow DSL grammar (custom language service)
  - .schema, .rules, .xform, .proc file editing
  - Validation and diagnostics

Milestone 3.2 -- Architecture Canvas
  - Svelte Flow-based design surface
  - Drag-and-drop components (services, data stores, APIs)
  - Connection drawing
  - AI-assisted suggestions

Milestone 3.3 -- Code Generation Preview
  - Generated code preview (read-only Monaco)
  - Target selection (Spark, Kafka, REST, etc.)
  - Diff against previous generation
```

---

## Alternatives Considered

| Option | Rejected Because |
|---|---|
| Electron | Bundles Chromium (~150 MB), slower, heavier memory |
| React | Virtual DOM overhead, larger bundle, performance not top priority |
| SolidJS | Smaller ecosystem, no Svelte Flow equivalent, fewer component options |
| Pure Rust UI (Iced/Slint/egui) | No Monaco equivalent, no graph viz, immature ecosystems |
| VS Code Extension | Not a product -- plugin in someone else's platform |
| IntelliJ Plugin | Java/Kotlin ecosystem, JNI bridge complexity |
| Dioxus Desktop | Still uses WebView (same as Tauri) but less mature |

---

## Open Items

1. **COBOL syntax highlighting for Monaco**: Need custom TextMate grammar or Monarch tokenizer
2. **TanStack Table Svelte 5 adapter**: No official adapter -- use table-core + custom glue
3. **File tree at 51K scale**: Test shadcn-extras Tree View with lazy loading/virtualization
4. **Tauri WebView on Linux**: Test WebKitGTK rendering consistency
5. **Auto-updater**: Tauri plugin exists but needs signing infrastructure
6. **License key system**: Phase 1 MVP uses local license file, no network
