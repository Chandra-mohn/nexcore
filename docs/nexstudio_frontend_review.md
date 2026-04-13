# NexStudio Frontend Code Review

## Svelte 5 / TypeScript / WCAG / Tauri Best Practices

**Date**: 2026-03-27
**Codebase**: /Users/chandramohn/workspace/nexstudio/src/
**Size**: 10,052 lines across 38 files (30 components, 2 stores, 3 editor utils)
**Stack**: Svelte 5 (runes) + TypeScript strict + Tailwind v4 + Monaco + Tauri v2
**Scope**: Report only (no changes made)

---

## Executive Summary

| Category | Score | Notes |
|---|---|---|
| Svelte 5 Patterns | GOOD | $derived used correctly in most places |
| TypeScript | NEEDS WORK | invoke<any[]> throughout, missing response types |
| Accessibility | POOR | No ARIA roles, no keyboard nav, no focus traps |
| Performance | MIXED | Virtual scroll in DataViewer (good), DOM bloat in logs (bad) |
| Component Size | NEEDS WORK | 3 components over 600 lines |
| Testing | FAIL | Zero tests, no test runner configured |
| Linting | FAIL | No ESLint, no Prettier, no Biome |

---

## P0: Performance Issues

### Log viewer DOM bloat (MigrationPanel.svelte:355)

```svelte
{#each logLines as line}
  <div class="...">{line}</div>
{/each}
```

With 1000s of migration log lines, this renders every line as a DOM node.
DataViewer has virtual scrolling -- log viewer does not.
**Fix**: Virtual scroll or cap visible lines with "show more" button.

### setInterval timer leak (MigrationPanel.svelte:105)

Elapsed timer created with `setInterval` but cleanup only happens on
migration complete -- not on component unmount. If user navigates away
mid-migration, the timer leaks.
**Fix**: Clean up in Svelte 5 `$effect` return or explicit `onDestroy`.

### Map-to-array on every render (SearchPanel.svelte:125)

```svelte
{#each [...groupedResults]}
```

Converts Map to array every render cycle. With 500 results, this
creates garbage on every reactive update.
**Fix**: Use `$derived` to pre-compute the array, or use a plain array.

### No invoke timeout protection

Every `invoke()` call to the Rust backend has no timeout. If the backend
hangs (e.g., parsing a massive copybook), the UI freezes with no feedback.
Affects: SearchPanel, ClusterMap, LineageGraph, CreateProjectWizard,
MigrationPanel, DataViewer.
**Fix**: Wrap in Promise.race with a reasonable timeout, or show a
cancellable loading indicator.

---

## P1: Component Size

Three components are oversized and should be split:

| Component | Lines | Recommendation |
|---|---|---|
| DataViewer.svelte | 1,092 | Split: Table, RawView, SchemaPanel, virtual scroll hook |
| CreateProjectWizard.svelte | 896 | Split: SourceConfig, TargetConfig, ModulesConfig, DataConfig |
| MigrationPanel.svelte | 625 | Split: ProgressBar, FileList, LogViewer |

These three files are 2,613 lines combined -- 26% of the frontend in 3 files.

---

## P1: TypeScript

### invoke<any[]> everywhere

Both stores and several components use untyped Tauri invoke responses:

```typescript
await invoke<any[]>("transpile_file", ...)    // pipeline.svelte.ts
await invoke<any[]>("run_rustify", ...)        // pipeline.svelte.ts
await invoke<any>("dv_auto_discover", ...)     // project.svelte.ts
```

**Fix**: Define TypeScript interfaces matching the Rust response types
(MigrationEvent, DiscoveryMatchResult, SchemaInfo, etc.) and use them
as generics: `invoke<MigrationEvent[]>(...)`.

### schemaInfo: any in DataViewer

```typescript
schemaInfo = $state<any>(null);  // line 44
```

SchemaInfo is a well-defined Rust struct -- should have a matching TS interface.

---

## P2: Accessibility (WCAG AA)

### No focus traps in modals

CreateProjectWizard and EditProjectWizard render as modals but have no
focus trap. Tab key escapes the modal into background content.
**Fix**: Use a focus trap library or manual focusTrap implementation.

### Missing ARIA roles on menus

TitleBar dropdown menus lack `role="menu"` and `role="menuitem"`.
Menu items are clickable divs with no keyboard navigation (arrow keys,
Enter to select, Escape to close).

### Missing ARIA on tab panels

IDELayout bottom tabs (Problems, Lineage, Clusters, Output) lack
`role="tablist"`, `role="tab"`, and `aria-selected`.

### No keyboard navigation in tree views

FileTree and DataExplorer support click but not arrow-key navigation.
Tree items lack `role="treeitem"` and `aria-expanded`.

### Missing log live region

MigrationPanel log output should have `role="log"` and `aria-live="polite"`
so screen readers announce new log entries.

### SVG visualizations inaccessible

ClusterMap (D3) and LineageGraph (Svelte Flow) have no text alternatives
or keyboard interaction. SVG containers lack `role="img"` and `aria-label`.

---

## P2: Svelte 5 Anti-Pattern

### $effect mutation in TitleBar (line 60-69)

```typescript
$effect(() => {
  const settingsItem = menus.File.find(m => m.label === "Project Settings...");
  if (settingsItem) settingsItem.disabled = !hasProject;
  // ... more mutations
});
```

Mutating object properties inside `$effect` is an anti-pattern in Svelte 5.
**Fix**: Use `$derived` to compute the menu with correct disabled states,
or restructure menu items as derived state.

---

## What's Good

- **Virtual scrolling in DataViewer** -- chunk-based fetching with LRU cache, handles
  large datasets well
- **$derived usage in IDELayout** -- textbook Svelte 5 pattern, all computed values
  derived correctly
- **Event cleanup in App.svelte** -- proper addEventListener/removeEventListener in
  onMount return
- **Store architecture** -- clean separation between project state and pipeline state
- **Debounced search** -- SearchPanel uses proper debounce timer
- **Dev build optimization** -- Cargo opt-level=2 for parser crates in dev builds
- **Darcula theme** -- well-implemented CSS custom properties, consistent aesthetic

---

## Tooling Gaps

| What | Status | Recommendation |
|---|---|---|
| ESLint | Not configured | Add eslint + eslint-plugin-svelte |
| Prettier | Not configured | Add prettier + prettier-plugin-svelte |
| Tests | None | Add vitest + @testing-library/svelte |
| Pre-commit hooks | None | Add lint-staged + husky |
| svelte-check | Configured but not in CI | Run in CI pipeline |

---

## Recommended Fix Order

1. **P0 Performance**: Fix timer leak, log DOM bloat, Map conversion
2. **P1 Split**: Break up DataViewer, CreateProjectWizard, MigrationPanel
3. **P1 Types**: Define TS interfaces for all invoke responses
4. **P2 A11y**: Focus traps, ARIA roles, keyboard nav (prioritize modals first)
5. **P2 Tooling**: ESLint + Prettier + vitest setup
6. **P2 Svelte**: Fix TitleBar $effect anti-pattern
