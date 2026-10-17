# Design

Parallaxe should feel like a **premium creative workshop** while staying out of the content's way. Images and 3D renders bring their own colors; the app's chrome stays restrained. No widespread neon, fake futurism, costly decorative animation or gratuitous "AI" effects.

## Experience principles

1. **Content first** — show resources before settings.
2. **Fast path** — few actions to inspect an asset, especially from the keyboard.
3. **Progressive disclosure** — advanced properties on demand, essentials always visible.
4. **Truthful UI** — show parser and codec limits, estimates and unavailable values.
5. **Safety by default** — never an unexpected move, deletion or overwrite.
6. **Modeless work** — long tasks run in the background without blocking browsing.
7. **Single source of truth** — tags and collections consistent everywhere.
8. **Native manners** — same interface grammar, OS-appropriate shortcuts and dialogs.

Three verbs structure the product: **Discover → Inspect → Reuse**. "Edit" joins the cycle after the MVP.

## Color tokens

Greys are strictly neutral: creators judge an asset's colors against them, so the chrome must not tint them. Components use semantic tokens only, never raw hex values. Pairs are checked against WCAG 2.2 (≥ 4.5:1 for text, ≥ 3:1 for essential graphics). Color never replaces a state label.

| Token | Dark | Light | Usage |
|---|---|---|---|
| `bg-canvas` | `#1B1B1B` | `#ECECEC` | Window, sidebar |
| `bg-surface` | `#232323` | `#F7F7F7` | Main work area |
| `bg-elevated` | `#2E2E2E` | `#E2E2E2` | Panes, menus, hover |
| `fg-primary` | `#EDEDED` | `#1A1A1A` | Main content |
| `fg-secondary` | `#9E9E9E` | `#5E5E5E` | Secondary information |
| `border-default` | `#3A3A3A` | `#C9C9C9` | Separators |
| `accent` | `#8A9DFF` | `#3452D6` | Selection, primary action |
| `fg-on-accent` | `#0E1330` | `#FFFFFF` | Text on accent |
| `success` | `#6FD39B` | `#116B3E` | Success |
| `warning` | `#F2C46B` | `#8A5A00` | Recoverable warning |
| `danger` | `#FF9A94` | `#A83232` | Risk, destruction |
| `focus-ring` | `#AEBBFF` | `#2142C9` | Keyboard focus |

## Typography and spacing

- **System fonts** (`-apple-system`, Segoe UI, Linux sans-serif) for the interface: native rendering at small sizes.
- **Bricolage Grotesque** for display headings only, bundled with the app (no network). Its optical size and width axes give large titles their character.
- Scale: 12, 13, **14 (base)**, 16, 20, 24, 30 px. Line height ≥ 1.4. Weights 400–700.
- **Tabular numbers** with localized grouping and explicit units (`42,680 triangles`, `29.4 MiB`).
- Long file names are truncated visually only; the full name stays in tooltips, copy and assistive technologies.
- Spacing: 4, 8, 12, 16, 24, 32, 48 px. Radius: 4, 6, 10, 14 px. Click targets ≥ 36 px, ideally 44 px.
- Motion: 100–180 ms, never required to understand an action; `prefers-reduced-motion` respected.
- Dark and light themes from day one, following the OS by default.

## Components

`AppButton`, `IconButton`, `TextInput`, `SearchField`, `FilterChip`, `TagChip`, `AssetTile`, `AssetListRow`, `MediaPreview`, `ModelViewer`, `MetadataField`, `ProgressIndicator`, `Toast`, `Dialog`, `EmptyState`, `ContextMenu`, `StatusPill`, `ErrorPanel`.

Every component has an accessible name and role, observable async states, keyboard support and tests. A `not_applicable` value renders differently from a valid `0`. `ErrorPanel` never shows a stack trace or a sensitive path by default.

## Wording

Use "Add folder", "Index", "Remove from library", "Open file", "Open folder", "Reconnect library", "Preview unavailable", "Not computable". Avoid "Upload your library", "Erase" for a virtual removal, or "Optimize" when nothing is measured.

## Accessibility

Every critical action is reachable by keyboard with visible focus. 3D controls have keyboard alternatives and never require a three-button mouse. 200% zoom keeps every essential action reachable. Screen readers: VoiceOver, Narrator, Orca.
