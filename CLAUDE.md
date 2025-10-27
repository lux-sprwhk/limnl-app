# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Limnl** is a privacy-first desktop dream journal application built with **Tauri** (Rust backend) and **SvelteKit** (frontend), using **Panda CSS** for styling and **TypeScript** for type safety.

All dream data is stored locally in a SQLite database with no cloud sync or telemetry. The app provides full CRUD operations for dream entries, including search, sleep quality tracking, and rich text content.

## Key Technologies

- **Tauri 2.x**: Desktop application framework (Rust backend)
- **SvelteKit**: Web framework with file-based routing
- **SQLite**: Local database for dream storage (via `rusqlite`)
- **Panda CSS**: Build-time CSS-in-JS with type safety (generates `styled-system/` directory)
- **bits-ui**: Headless, accessible component primitives
- **sveltekit-superforms**: Form handling with Zod validation
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
  - `+page.svelte` - Home page
  - `+layout.svelte` - App layout
  - `settings/+page.svelte` - LLM and app settings
  - `dreams/` - Dream journal routes:
    - `+page.svelte` - List all dreams with search
    - `new/+page.svelte` - Create new dream (includes AI title generation)
    - `[id]/+page.svelte` - View dream details
    - `[id]/edit/+page.svelte` - Edit dream
- **API Layer**:
  - `src/lib/api/dreams.ts` - Tauri command wrapper functions
    - `dreamsApi.createDream()`, `getDreams()`, `getDream()`, `updateDream()`, `deleteDream()`, `searchDreams()`
  - `src/lib/api/llm.ts` - LLM provider abstraction
    - `llmApi.generateTitle()` - Generate dream title from content
- **Types**:
  - `src/lib/types/dream.ts` - Dream data interfaces
    - `Dream`, `CreateDreamInput`, `UpdateDreamInput`
  - `src/lib/types/llm.ts` - LLM configuration interfaces
    - `LLMConfig`, `LLMProvider`, `GenerateTitleRequest`, `GenerateTitleResponse`
- **Stores**: `src/lib/stores/` - Svelte state management
  - `llm-settings.ts` - LLM configuration store (persisted to localStorage)
- **Components**: `src/lib/components/` - Reusable UI components
  - `ui/` - Styled UI primitives (Button, etc.)
- **Utils**: `src/lib/utils/` - Utility functions
- **Static Assets**: `static/` - Public files

### Backend (Tauri/Rust)

- **Source**: `src-tauri/src/` - Rust application code
  - `lib.rs` - App initialization, registers Tauri commands
  - `main.rs` - Entry point
  - `commands.rs` - Tauri command handlers (invoked from frontend)
  - `db/` - Database layer:
    - `mod.rs` - Module exports
    - `models.rs` - Data structures (`Dream`, `CreateDreamInput`, `UpdateDreamInput`)
    - `connection.rs` - SQLite initialization and schema creation
    - `dreams.rs` - CRUD operations (create, get_all, get_by_id, update, delete, search)
- **Config**: `src-tauri/tauri.conf.json` - Tauri configuration
  - Window size: 800x600
  - Frontend served from `build/` directory
  - Dev server: `localhost:5173`
- **Dependencies**: `src-tauri/Cargo.toml`
  - `tauri`, `serde`, `serde_json` - Core framework
  - `rusqlite` - SQLite database driver
  - `chrono` - Date/time handling
  - `tauri-plugin-shell`, `tauri-plugin-updater` - Official plugins

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

### Database (SQLite)

**Location** (platform-specific):
- **Linux**: `~/.local/share/limnl/limnl-journal/dreams.db`
- **macOS**: `~/Library/Application Support/com.limnl.limnl-journal/dreams.db`
- **Windows**: `%APPDATA%\limnl\limnl-journal\dreams.db`

**Schema**:
```sql
CREATE TABLE dreams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_recorded TEXT NOT NULL,      -- When the dream was recorded
    date_occurred TEXT NOT NULL,      -- When the dream happened
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    emotions_tags TEXT,               -- Future: JSON array of emotions
    sleep_quality INTEGER,            -- 1-5 rating
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Inspect Database**:
```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Example queries
SELECT * FROM dreams;
SELECT COUNT(*) FROM dreams;
SELECT title, date_occurred FROM dreams WHERE content LIKE '%flying%';
```

### LLM Integration (BYOLLM - Bring Your Own LLM)

**Overview**: Limnl supports optional LLM features like automatic dream title generation. Users can configure their preferred LLM provider in settings.

**Supported Providers**:
- **Ollama** (local, privacy-first, free) - Runs LLM models locally on user's machine
- **OpenAI** - Requires API key (gpt-4o-mini recommended for cost)
- **Anthropic Claude** - Requires API key (claude-3-5-haiku-20241022 recommended)
- **Disabled** - All LLM features disabled (manual input only)

**Configuration Storage**:
- Stored in browser localStorage (key: `limnl-llm-config`)
- API keys never leave the device
- Settings persist across sessions

**Current Features**:
- **Title Generation**: Auto-generate dream titles from content in dream creation form
  - Click "Generate" button next to title field
  - Requires dream content to be written first
  - Falls back to manual input if LLM unavailable

**Implementation**:
```typescript
// Using the LLM API
import { llmApi } from '$lib/api/llm';

const response = await llmApi.generateTitle({
  content: 'I was flying over mountains...'
});
console.log(response.title); // "Flying Over Mountains"

// Checking if LLM is configured
import { llmSettings } from '$lib/stores/llm-settings';
if (llmSettings.isConfigured) {
  // LLM features available
}
```

**Provider-Specific Notes**:
- **Ollama**: User must install Ollama separately and have models pulled (e.g., `ollama pull llama3.2`)
- **Cloud APIs**: User provides their own API key via Settings page
- **Privacy**: Ollama keeps all data local; cloud providers will receive dream content for processing

### Data Flow

1. User interacts with Svelte component (e.g., dream form)
2. Component calls API client function: `dreamsApi.createDream(input)`
3. API client invokes Tauri command: `invoke('create_dream', { input })`
4. Rust backend receives command in `commands.rs`
5. Command handler calls database function: `db::dreams::create()`
6. Database operation executes SQLite query via `rusqlite`
7. Result is serialized to JSON and returned to frontend
8. Component receives result and updates UI

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
- **Database**: Automatically created on first run at platform-specific location
- **Privacy-First**: All data stored locally, no cloud sync, no telemetry
- **Tauri Commands**: Defined in `src-tauri/src/commands.rs`, registered in `lib.rs`
- **Web-Only Mode**: `pnpm dev` won't have database access (Tauri commands unavailable)
- **Full App Mode**: `pnpm tauri:dev` required for complete functionality

## Tauri Commands Reference

Available commands (invoked from frontend via `@tauri-apps/api`):

```rust
// Create a new dream entry
create_dream(input: CreateDreamInput) -> Result<Dream>

// Get all dreams (sorted by date_occurred DESC)
get_dreams() -> Result<Vec<Dream>>

// Get a single dream by ID
get_dream(id: i64) -> Result<Dream>

// Update an existing dream
update_dream(id: i64, input: UpdateDreamInput) -> Result<Dream>

// Delete a dream
delete_dream(id: i64) -> Result<()>

// Search dreams by query (searches title and content)
search_dreams(query: String) -> Result<Vec<Dream>>
```

Example frontend usage:
```typescript
import { dreamsApi } from '$lib/api/dreams';

// Create dream
const dream = await dreamsApi.createDream({
  date_occurred: '2025-01-15',
  title: 'Flying Dream',
  content: 'I was soaring above mountains...',
  sleep_quality: 4
});

// Search dreams
const results = await dreamsApi.searchDreams('flying');
```
