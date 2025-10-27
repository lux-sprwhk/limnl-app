# Limnl - Dream Journal

A privacy-first desktop dream journal application built with Tauri and SvelteKit. Record, explore, and understand your dreams with a beautiful, local-first interface.

## Features

- **Privacy-First Design**: All data stored locally in SQLite database - no cloud sync, no telemetry
- **Rich Dream Entries**: Record dreams with title, content, date, and sleep quality ratings
- **Full-Text Search**: Search across all your dreams by title or content
- **Complete CRUD Operations**: Create, read, update, and delete dream entries
- **Sleep Quality Tracking**: Rate and visualize your sleep quality (1-5 scale)
- **Modern UI**: Clean, responsive interface built with Panda CSS
- **Desktop Native**: Fast, secure desktop application powered by Tauri

## Tech Stack

- **[Tauri](https://tauri.app/)** - Build smaller, faster, and more secure desktop applications
- **[SvelteKit](https://kit.svelte.dev/)** - Fast, reactive web framework with excellent DX
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript
- **[Panda CSS](https://panda-css.com/)** - Build-time CSS-in-JS with type safety
- **[bits-ui](https://bits-ui.com/)** - Headless, accessible component primitives
- **[lucide-svelte](https://lucide.dev/)** - Beautiful & consistent icon library
- **[sveltekit-superforms](https://superforms.rocks/)** - Powerful form library for SvelteKit
- **[zod](https://zod.dev/)** - TypeScript-first schema validation
- **[Vitest](https://vitest.dev/)** - Fast unit testing framework
- **[ESLint](https://eslint.org/)** + **[Prettier](https://prettier.io/)** - Code quality & formatting

## Project Structure

```
limnl-app/
├── src/
│   ├── lib/
│   │   ├── api/
│   │   │   └── dreams.ts        # API client for Tauri commands
│   │   ├── types/
│   │   │   └── dream.ts         # TypeScript type definitions
│   │   ├── components/
│   │   │   └── ui/              # Reusable UI components (Button, etc.)
│   │   └── utils/               # Utility functions
│   ├── routes/
│   │   ├── +page.svelte         # Home page
│   │   ├── +layout.svelte       # App layout
│   │   └── dreams/
│   │       ├── +page.svelte            # Dreams list
│   │       ├── new/
│   │       │   └── +page.svelte        # Create new dream
│   │       └── [id]/
│   │           ├── +page.svelte        # Dream detail view
│   │           └── edit/
│   │               └── +page.svelte    # Edit dream
│   └── app.css                  # Global styles
├── src-tauri/
│   ├── src/
│   │   ├── db/
│   │   │   ├── mod.rs           # Module exports
│   │   │   ├── models.rs        # Data structures (Dream, CreateDreamInput, etc.)
│   │   │   ├── connection.rs    # Database initialization
│   │   │   └── dreams.rs        # CRUD operations
│   │   ├── commands.rs          # Tauri command handlers
│   │   ├── lib.rs               # App initialization
│   │   └── main.rs              # Entry point
│   └── Cargo.toml               # Rust dependencies
├── styled-system/               # Generated Panda CSS files (DO NOT EDIT)
└── static/                      # Static assets
```

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

**Note**: For the full dream journal experience with database access, use `pnpm tauri:dev`. The web-only mode (`pnpm dev`) won't have access to the Rust backend and SQLite database.

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

## Architecture

### Data Flow

1. User interacts with Svelte component
2. Component calls API client function (`dreamsApi.createDream()`, etc.)
3. API client invokes Tauri command via `@tauri-apps/api`
4. Rust backend receives command, executes database operation
5. Result is serialized to JSON and returned to frontend
6. Component updates UI with new data

### Database

**SQLite Database** stored locally in:
- **Linux**: `~/.local/share/limnl/limnl-journal/dreams.db`
- **macOS**: `~/Library/Application Support/com.limnl.limnl-journal/dreams.db`
- **Windows**: `%APPDATA%\limnl\limnl-journal\dreams.db`

**Schema**:
```sql
CREATE TABLE dreams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_recorded TEXT NOT NULL,      -- When the dream was recorded
    date_occurred TEXT NOT NULL,      -- When the dream actually happened
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    emotions_tags TEXT,               -- Future: JSON array of emotion tags
    sleep_quality INTEGER,            -- 1-5 rating
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Inspect Database**:
```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Query examples
SELECT * FROM dreams;
SELECT COUNT(*) FROM dreams;
SELECT title, date_occurred FROM dreams WHERE content LIKE '%keyword%';
```

## Testing the Dream Journal

### Manual Testing Workflow

1. **Start the app**: `pnpm tauri:dev`

2. **Navigate to Dreams**: Click "Open Dream Journal" on home page

3. **Create a new dream**:
   - Click "New Dream" button
   - Fill in the form:
     - Date: Today's date (or any past date)
     - Title: "Flying Over Mountains"
     - Content: "I was soaring above snow-capped peaks..."
     - Sleep Quality: Select 4
   - Click "Save Dream"

4. **View dream list**: Should see your new dream in a card with truncated content

5. **View dream details**: Click on a dream card to see full content and metadata

6. **Edit a dream**: Click "Edit" button, modify content, save changes

7. **Search functionality**: Create multiple dreams, use search box to filter by keywords

8. **Delete a dream**: Open dream detail page, click "Delete", confirm deletion

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

## Roadmap

### Phase 2: LLM Integration
- Symbol extraction from dream content
- Pattern recognition across dreams
- Automated insights generation
- Dream analysis and interpretation

### Phase 3: Tarot System
- Digital tarot deck
- Card reading spreads
- Reading interpretation journaling
- Integration with dream journal

### Future Enhancements
- **Emotion Tagging UI**: Visual tagging system for dream emotions
- **Symbol Highlighting**: Automatic recognition of recurring symbols
- **Timeline Visualization**: Visual timeline of dreams over time
- **Export Functionality**: PDF and Markdown export
- **Rich Text Editor**: Enhanced formatting options
- **Encryption**: Password-protect database with SQLCipher
- **Dark Mode**: Complete dark theme support
- **Keyboard Shortcuts**: Power user features

## Troubleshooting

### Frontend builds but Tauri doesn't start
- Ensure Rust is installed: `rustc --version`
- Check Tauri system dependencies are installed
- Run `pnpm tauri:dev` and check terminal for errors

### Database errors
- Check file permissions on data directory
- Ensure SQLite is working: `sqlite3 --version`
- Verify database location exists

### Type errors in frontend
- Run `pnpm check` to see TypeScript errors
- Ensure `@tauri-apps/api` is installed
- Restart TypeScript server in your editor

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Panda CSS Documentation](https://panda-css.com/docs)
- [bits-ui Documentation](https://bits-ui.com/)
- [Vitest Documentation](https://vitest.dev/)

## License

[Add your license here]

---

**Status**: ✅ Basic dream journal fully implemented and ready for testing!
