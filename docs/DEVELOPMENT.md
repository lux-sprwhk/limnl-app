# Development Guide

← [Back to README](../README.md)

## Table of Contents

- [Getting Started](#getting-started)
- [Available Scripts](#available-scripts)
- [Styling with Panda CSS](#styling-with-panda-css)
- [Components](#components)
- [Testing the Application](#testing-the-application)
- [Database Migrations](#database-migrations)
- [Troubleshooting](#troubleshooting)
- [Learn More](#learn-more)

## Getting Started

### Prerequisites

1. **Node.js** (v18+) and **pnpm**

   ```bash
   # Install pnpm if needed
   npm install -g pnpm
   ```

2. **Rust** (for Tauri backend)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **System Dependencies** (Linux only)

   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

### Installation

```bash
# Install dependencies
pnpm install
```

### Development

```bash
# Run as desktop app (recommended)
pnpm tauri:dev

# Or run in web-only mode for frontend development
pnpm dev
```

**Note**: For the full experience with database access, use `pnpm tauri:dev`. The web-only mode (`pnpm dev`) won't have access to the Rust backend and SQLite database.

### Building

```bash
# Build for web
pnpm build

# Build Tauri desktop app
pnpm tauri:build
```

### Testing

```bash
# Run tests
pnpm test

# Run tests with UI
pnpm test:ui
```

### Code Quality

```bash
# Lint code
pnpm lint

# Format code
pnpm format

# Type check
pnpm check
```

## Available Scripts

- `pnpm dev` - Start development server
- `pnpm build` - Build for production
- `pnpm preview` - Preview production build
- `pnpm test` - Run tests
- `pnpm test:ui` - Run tests with UI
- `pnpm lint` - Lint and check formatting
- `pnpm format` - Format code
- `pnpm check` - Type check
- `pnpm tauri:dev` - Run Tauri dev server
- `pnpm tauri:build` - Build Tauri app

## Styling with Panda CSS

Panda CSS provides type-safe, build-time CSS. Example usage:

```svelte
<script>
  import { css } from '../styled-system/css';

  const buttonStyles = css({
    bg: 'blue.600',
    color: 'white',
    padding: '2',
    borderRadius: 'md',
    '&:hover': {
      bg: 'blue.700'
    }
  });
</script>

<button class={buttonStyles}>Click me</button>
```

## Components

### UI Components

Components are built using bits-ui primitives and styled with Panda CSS:

- `Button` - Customizable button with variants

Example:

```svelte
<Button variant="primary" onclick={handleClick}>
  Click me
</Button>
```

## Testing the Application

### Initial Setup

1. **Start the app**: `pnpm tauri:dev`

2. **First-time setup**:
   - **Set up PIN** (optional): Enter a 4+ digit PIN on first launch
   - **Configure user profile**: Settings → Enter name, select zodiac sign and MBTI type
   - **Configure LLM** (optional): Settings → Choose provider (Ollama, OpenAI, Claude, or Disabled)

### Testing Dreams

1. **Navigate to Dreams**: Click "Dreams" in sidebar
2. **Create a new dream**:
   - Click "New Dream" button
   - Fill in form (Date, Title, Content, Sleep Quality)
   - **Optional metadata** (added in v0.2):
     - Mark as recurring dream (checkbox)
     - If recurring, select when it last occurred (dropdown: today, yesterday, last week, etc.)
     - Mark as lucid dream (checkbox for dreams where you were aware you were dreaming)
   - Optionally click "Generate" next to title (requires LLM)
   - Click "Save Dream"
3. **Search dreams**: Use search box to filter by keywords
4. **View dream details**: Click on dream to see full content
5. **Generate AI Analysis** (requires LLM):
   - Open dream detail page
   - Click "Analyze Dream" button
   - View themes, patterns, emotional analysis, and narrative summary
   - See associated symbol cards automatically linked to the analysis
6. **Generate Creative Prompts** (requires LLM and analysis):
   - After generating analysis, click "Generate Creative Prompts"
   - View AI-generated prompts for:
     - Image generation (visual art inspired by the dream)
     - Music creation (soundscape/playlist ideas)
     - Story writing (narrative extensions of the dream)
7. **Edit/Delete**: Click on dream → Edit or Delete

### Testing Mind Dumps

1. **Navigate to Mind Dumps**: Click "Mind Dumps" in sidebar
2. **Create mind dump**: Click "New" → Write stream-of-consciousness content
3. **Word count**: Automatically tracked and displayed
4. **Search**: Filter across all mind dumps

### Testing Bug Tracking

1. **Navigate to Bugs**: Click "Bugs" in sidebar
2. **Create a bug**:
   - Click "Create Bug"
   - Enter title and description
   - Optionally generate title with LLM
   - Save
3. **Filter by status**: Use tabs (All / Active / Resolved / Archived)
4. **Add notes**: Open bug → Use notes section to add thoughts
5. **Card discovery**: Click "Discover" → Draw cards for insights
6. **Link cards to bug**: In bug detail, click "Draw Cards" or manually link
7. **Change status**: Mark as Resolved or Archived
8. **View related bugs**: Navigate to a card → See all bugs using that card

### Testing Card System

1. **Browse cards**: Navigate to Cards → List
2. **Draw cards**:
   - Click "Draw" in sidebar
   - Choose 1-card or 3-card spread
   - Get LLM commentary (if configured)
3. **View card details**: Click on any card to see details and related bugs
4. **Card usage stats**: Cards sorted by usage in bug tracking

### Testing Database Backup

1. **Navigate to Settings**
2. **Scroll to Database Backup section**
3. **Click "Backup Database"**
4. **Choose save location in file dialog**
5. **Verify backup file created** with date stamp

### Testing Authentication

1. **Set up PIN**: Settings → Security → Set up PIN
2. **Log out**: Close and reopen app
3. **Enter PIN**: Should prompt for PIN on startup
4. **Toggle PIN**: Settings → Toggle "Require PIN" off/on

## Database Migrations

Limnl uses a **file-based SQL migration system** that runs automatically on app startup.

### How Migrations Work

1. **Automatic Execution**: Migrations run when the app starts
2. **Version Tracking**: Stored in `schema_version` table
3. **Sequential Application**: Only new migrations are applied
4. **Idempotent**: Safe to run multiple times (uses `IF NOT EXISTS`)

### Migration Files

Located in `src-tauri/migrations/`:

```
src-tauri/migrations/
├── 001_initial.sql              # Base schema (8 tables)
├── 002_add_dream_metadata.sql   # Dream metadata fields (recurring, lucid)
├── 002_example.sql.example      # Template for new migrations
└── README.md                    # Comprehensive migration documentation
```

**Current Schema Version**: 2

**Migration History**:
- **Migration 001**: Core tables (dreams, bugs, mind_dumps, cards, dream analysis, relationships)
- **Migration 002**: Dream metadata (is_recurring, last_occurrence_period, is_lucid)

### Checking Migration Status

```bash
# View current schema version
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db "SELECT * FROM schema_version;"

# List all tables
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db ".tables"
```

### Adding a New Migration

1. Create numbered SQL file: `src-tauri/migrations/002_my_feature.sql`
2. Write SQL with `IF NOT EXISTS` for safety
3. Register in `src-tauri/src/db/migrations.rs`:
   ```rust
   const MIGRATIONS: &[&str] = &[
       include_str!("../../migrations/001_initial.sql"),
       include_str!("../../migrations/002_my_feature.sql"),
   ];
   ```
4. Test: `cargo test db::migrations::tests`
5. Next app start will auto-apply the migration

### Testing Migrations Locally

Before committing a migration, test it locally:

1. **Backup your database first**:
   ```bash
   # Linux
   cp ~/.local/share/limnl/limnl-journal/dreams.db ~/dreams-backup.db
   
   # macOS
   cp ~/Library/Application\ Support/com.limnl.limnl-journal/dreams.db ~/dreams-backup.db
   
   # Windows
   copy %APPDATA%\limnl\limnl-journal\dreams.db %USERPROFILE%\dreams-backup.db
   ```

2. **Test the migration**:
   ```bash
   # Run the app - migration will execute on startup
   pnpm tauri:dev
   ```

3. **Verify schema**:
   ```bash
   # Linux
   sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db ".schema dreams"
   
   # Check migration was recorded
   sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db "SELECT * FROM schema_version;"
   ```

4. **Verify existing data is preserved**:
   ```bash
   sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db "SELECT COUNT(*) FROM dreams;"
   ```

5. **Test idempotency** (migration should be safe to run twice):
   - Close and reopen the app
   - Migration should not error or duplicate data

### Migration Rollback

Since Limnl is a desktop app with local data:

- **Migrations are forward-only** (no automatic rollback)
- **Users can restore from backup** if needed:
  ```bash
  # Restore from backup
  cp ~/dreams-backup.db ~/.local/share/limnl/limnl-journal/dreams.db
  ```
- **Test thoroughly before release** - migrations are applied automatically on app startup
- **Version tracking** prevents re-running migrations, but manual schema changes could cause issues

### Data Migration Tool

For backfilling existing data (e.g., generating analyses for old dreams):

```bash
# Set LLM configuration
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2

# Run data migration (in src-tauri directory)
cargo run --bin migrate-dream-analysis

# Dry run (preview only)
cargo run --bin migrate-dream-analysis -- --dry-run

# Limit to first N dreams
cargo run --bin migrate-dream-analysis -- --limit 5
```

**Important Notes**:
- This is separate from schema migrations and requires LLM configuration
- **Cost Warning**: Analyzing large dream journals with cloud APIs can be expensive
  - 100 dreams ≈ $1-8 depending on provider (OpenAI/Anthropic)
  - Use Ollama (free, local) for bulk operations if cost is a concern
  - Use `--dry-run` first to see how many dreams will be processed
  - Use `--limit` to test with a small batch before running on entire journal

## Troubleshooting

### Frontend builds but Tauri doesn't start

- Ensure Rust is installed: `rustc --version`
- Check Tauri system dependencies are installed
- Run `pnpm tauri:dev` and check terminal for errors

### Database errors

- Check file permissions on data directory
- Ensure SQLite is working: `sqlite3 --version`
- Verify database location exists
- Check migration logs during app startup

### Migration errors

- Check `src-tauri/migrations/` directory exists
- Ensure SQL syntax is valid
- Verify `MIGRATIONS` array in `migrations.rs` is updated
- Run tests: `cargo test db::migrations::tests`
- Check terminal output for specific migration failures

### Type errors in frontend

- Run `pnpm check` to see TypeScript errors
- Ensure `@tauri-apps/api` is installed
- Restart TypeScript server in your editor

### LLM features not working

- Verify LLM provider is configured in Settings
- Check API keys are correct (for OpenAI/Anthropic)
- For Ollama: ensure service is running (`ollama serve`)
- Check browser console for error messages

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Panda CSS Documentation](https://panda-css.com/docs)
- [bits-ui Documentation](https://bits-ui.com/)
- [Vitest Documentation](https://vitest.dev/)

