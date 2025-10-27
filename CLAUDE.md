# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A desktop application built with **Tauri** (Rust backend) and **SvelteKit** (frontend), using **Panda CSS** for styling and **TypeScript** for type safety.

## Key Technologies

- **Tauri 2.x**: Desktop application framework (Rust backend)
- **SvelteKit**: Web framework with file-based routing
- **Panda CSS**: Build-time CSS-in-JS with type safety (generates `styled-system/` directory)
- **bits-ui**: Headless, accessible component primitives
- **Vitest**: Unit testing with jsdom environment
- **pnpm**: Package manager (required)

## Essential Commands

### Development
```bash
# Web development (Vite dev server at localhost:5173)
pnpm dev

# Desktop development (runs Tauri app with hot reload)
pnpm tauri:dev

# Watch mode type checking
pnpm check:watch
```

### Testing
```bash
# Run all tests
pnpm test

# Run tests with UI
pnpm test:ui

# Tests are located in src/**/*.{test,spec}.{js,ts}
```

### Building
```bash
# Build web app (outputs to build/)
pnpm build

# Build desktop app (creates platform-specific installers)
pnpm tauri:build
```

### Code Quality
```bash
# Lint and check formatting
pnpm lint

# Auto-format code
pnpm format

# Type check Svelte and TypeScript
pnpm check
```

## Architecture

### Frontend (SvelteKit)

- **Routes**: `src/routes/` - File-based routing
  - `+page.svelte` - Page components
  - `+layout.svelte` - Layout wrappers
- **Components**: `src/lib/components/` - Reusable UI components
  - `ui/` - Styled UI primitives (built on bits-ui)
- **Utils**: `src/lib/utils/` - Utility functions
- **Static Assets**: `static/` - Public files

### Backend (Tauri/Rust)

- **Source**: `src-tauri/src/` - Rust application code
- **Config**: `src-tauri/tauri.conf.json` - Tauri configuration
  - Window size: 800x600
  - Frontend served from `build/` directory
  - Dev server: `localhost:5173`
- **Dependencies**: `src-tauri/Cargo.toml`
  - Uses `tauri-plugin-shell` and `tauri-plugin-updater`

### Styling (Panda CSS)

- **Config**: `panda.config.ts`
- **Generated**: `styled-system/` - Auto-generated CSS utilities (DO NOT EDIT)
- **Pattern**: Run `pnpm panda codegen` to regenerate after config changes
- **Usage**: Import from `styled-system/css` or `styled-system/patterns`

```typescript
import { css } from '../styled-system/css';

const styles = css({
  bg: 'blue.600',
  color: 'white',
  padding: '2'
});
```

### Component Architecture

Components use:
- **bits-ui** for accessible primitives (headless components)
- **Panda CSS** for styling with type-safe props
- **Svelte 5 runes** syntax (`$props`, `$state`, etc.)

Example component structure:
```svelte
<script lang="ts">
  import { Button as BitsButton } from 'bits-ui';
  import { css } from '../../../styled-system/css';

  interface Props {
    variant?: 'primary' | 'secondary' | 'outline';
    // ...
  }

  let { variant = 'primary' }: Props = $props();
</script>
```

## Adapter Configuration

- Uses `@sveltejs/adapter-static` for SPA output
- Fallback: `index.html` for client-side routing
- Output directory: `build/` (consumed by Tauri)

## Important Workflows

### After Panda Config Changes
```bash
pnpm panda codegen  # Regenerate styled-system/
```

### Before Commits
```bash
pnpm lint    # Check linting and formatting
pnpm check   # Type check
pnpm test    # Run tests
```

### Working with Forms
- Use `sveltekit-superforms` for form handling
- Use `zod` for schema validation

### Adding Icons
- Use `lucide-svelte` for consistent icons
- Import from `lucide-svelte` package

## Development Notes

- **Panda CSS** must run codegen before dev/build (handled by npm scripts)
- **SvelteKit sync** runs automatically via `prepare` script
- Tauri frontend expects static build output in `build/`
- Test files use `.test.ts` or `.spec.ts` suffix
- Global styles in `src/app.css`
