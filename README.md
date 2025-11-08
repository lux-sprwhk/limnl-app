# Limnl Space - Personal Insight & Dream Journal

A privacy-first desktop application for self-reflection, dream journaling, problem tracking, and personal insights. Built with Tauri and SvelteKit, combining powerful local-first data storage with optional AI-powered features.

## Features

### Core Features

- **Privacy-First Design**: All data stored locally in SQLite database - no cloud sync, no telemetry
- **PIN Authentication**: Optional PIN protection for your private data
- **User Profiles**: Personalize your experience with name, zodiac sign, and MBTI type
- **Database Backup**: Export your entire database with interactive file dialogs
- **Desktop Native**: Fast, secure desktop application powered by Tauri

### Dream Journal

- **Rich Dream Entries**: Record dreams with title, content, date, and sleep quality ratings
- **Full-Text Search**: Search across all your dreams by title or content
- **Complete CRUD Operations**: Create, read, update, and delete dream entries
- **Sleep Quality Tracking**: Rate your sleep quality (1-5 scale)
- **AI Title Generation**: Optional LLM-powered title suggestions from content

### Mind Dumps

- **Quick Thought Capture**: Stream-of-consciousness journaling for rapid ideation
- **Word Count Tracking**: Monitor your writing volume
- **Full-Text Search**: Find thoughts across all your mind dumps
- **Optional Titles**: Add titles or let content speak for itself

### Bug Tracking System

- **Issue Management**: Track personal bugs, problems, or areas for improvement
- **Status Workflow**: Active → Resolved → Archived lifecycle
- **Card Associations**: Link tarot-like cards to bugs for additional insights
- **Notes System**: Add persistent notes to bug entries
- **Conversation History**: Track discussions and thought processes
- **Card Discovery Tool**: Draw cards to explore problems from new angles
- **AI Features**: Generate titles and optimize descriptions with LLM

### Card System (Tarot-like Deck)

- **Fixed Deck**: Curated collection of insight cards for divination and reflection
- **Single & Three-Card Spreads**: Draw cards for guidance and perspective
- **Card Browser**: Explore all cards sorted by usage in bug tracking
- **Bug Relationships**: See which bugs are associated with each card
- **AI Commentary**: Get LLM-powered interpretations of individual or multiple cards
- **Chat with Context**: Have conversations with card context and user profile awareness

### LLM Integration (BYOLLM - Bring Your Own LLM)

- **Multiple Providers**: Ollama (local), OpenAI, Anthropic Claude, or Disabled
- **Privacy-Focused**: Local processing with Ollama or user-provided API keys
- **Title Generation**: Auto-generate titles for dreams and bugs
- **Content Optimization**: Refine and improve dream/bug descriptions
- **Card Commentary**: Receive insights on single or multiple cards
- **Profile-Aware Chat**: Conversations that understand your zodiac and MBTI type

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
│   │   │   ├── dreams.ts        # Dream journal API
│   │   │   ├── bugs.ts          # Bug tracking API
│   │   │   ├── cards.ts         # Card system API
│   │   │   ├── mind-dumps.ts    # Mind dumps API
│   │   │   ├── llm.ts           # LLM provider abstraction
│   │   │   └── database.ts      # Database backup API
│   │   ├── types/
│   │   │   ├── dream.ts         # Dream type definitions
│   │   │   ├── bug.ts           # Bug type definitions
│   │   │   ├── card.ts          # Card type definitions
│   │   │   ├── mind-dump.ts     # Mind dump types
│   │   │   ├── llm.ts           # LLM config types
│   │   │   ├── auth.ts          # Authentication types
│   │   │   └── user.ts          # User profile types
│   │   ├── components/
│   │   │   ├── ui/              # UI components (Button, Select, etc.)
│   │   │   ├── BugNotes.svelte  # Bug notes editor
│   │   │   ├── Navbar.svelte    # Main navigation
│   │   │   └── auth/
│   │   │       └── PinInput.svelte  # PIN authentication input
│   │   ├── stores/
│   │   │   ├── auth.svelte.ts         # Authentication state (Svelte 5 runes)
│   │   │   ├── user-profile.svelte.ts # User profile state
│   │   │   └── llm-settings.svelte.ts # LLM configuration state
│   │   └── utils/               # Utility functions
│   ├── routes/
│   │   ├── +page.svelte         # Home page (PIN entry)
│   │   ├── +layout.svelte       # App layout with navbar
│   │   ├── dreams/
│   │   │   ├── +page.svelte            # Dreams list with search
│   │   │   ├── new/+page.svelte        # Create new dream
│   │   │   └── [id]/
│   │   │       ├── +page.svelte        # Dream detail view
│   │   │       └── edit/+page.svelte   # Edit dream
│   │   ├── bugs/
│   │   │   ├── +page.svelte            # Bug list with status filtering
│   │   │   ├── create/+page.svelte     # Create new bug
│   │   │   ├── discover/+page.svelte   # Card discovery tool
│   │   │   └── [id]/+page.svelte       # Bug detail with notes & cards
│   │   ├── mind-dumps/
│   │   │   ├── +page.svelte            # Mind dumps list
│   │   │   ├── new/+page.svelte        # Create mind dump
│   │   │   └── [id]/+page.svelte       # View/edit mind dump
│   │   ├── cards/
│   │   │   ├── list/+page.svelte       # Card browser
│   │   │   ├── draw/+page.svelte       # Card drawing (1 or 3-card spreads)
│   │   │   └── [id]/+page.svelte       # Card detail with related bugs
│   │   ├── settings/+page.svelte       # Settings (LLM, profile, PIN, backup)
│   │   └── help/+page.svelte           # Help documentation
│   └── app.css                  # Global styles
├── src-tauri/
│   ├── src/
│   │   ├── db/
│   │   │   ├── mod.rs           # Database module exports
│   │   │   ├── models.rs        # Data structures (Dream, Bug, Card, etc.)
│   │   │   ├── connection.rs    # Database initialization & schema
│   │   │   ├── dreams.rs        # Dream CRUD operations
│   │   │   ├── bugs.rs          # Bug CRUD operations
│   │   │   ├── cards.rs         # Card operations & relationships
│   │   │   └── mind_dumps.rs    # Mind dump CRUD operations
│   │   ├── llm/
│   │   │   ├── mod.rs           # LLM module exports
│   │   │   ├── client.rs        # HTTP client for LLM APIs
│   │   │   └── providers.rs     # Ollama, OpenAI, Anthropic implementations
│   │   ├── commands.rs          # Tauri command handlers (34 commands)
│   │   ├── lib.rs               # App initialization
│   │   └── main.rs              # Entry point
│   └── Cargo.toml               # Rust dependencies
├── styled-system/               # Generated Panda CSS files (DO NOT EDIT)
├── static/                      # Static assets
└── cards.json                   # Card deck definitions
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

**Schema** (5 tables):

```sql
-- Dream journal entries
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

-- Bug/problem tracking entries
CREATE TABLE bugs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,             -- 'active', 'resolved', or 'archived'
    cards_drawn TEXT,                 -- JSON array of card IDs drawn for this bug
    conversation_history TEXT,        -- JSON array of conversation messages
    notes TEXT,                       -- JSON array of note objects
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    resolved_at TEXT                  -- Timestamp when status changed to 'resolved'
);

-- Mind dump/quick thought entries
CREATE TABLE mind_dumps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,                       -- Optional title
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Card definitions (tarot-like deck)
CREATE TABLE cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,        -- Card name (e.g., "The Void", "The Mirror")
    created_at TEXT NOT NULL
);

-- Many-to-many relationship: bugs ↔ cards
CREATE TABLE bug_cards (
    bug_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    position INTEGER,                 -- Order in which card was drawn for the bug
    created_at TEXT NOT NULL,
    PRIMARY KEY (bug_id, card_id),
    FOREIGN KEY (bug_id) REFERENCES bugs(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_bugs_status ON bugs(status);
CREATE INDEX idx_bugs_created_at ON bugs(created_at);
CREATE INDEX idx_dreams_date_occurred ON dreams(date_occurred);
CREATE INDEX idx_mind_dumps_created_at ON mind_dumps(created_at);
```

**Inspect Database**:

```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Query examples
SELECT * FROM dreams;
SELECT * FROM bugs WHERE status = 'active';
SELECT * FROM mind_dumps ORDER BY created_at DESC LIMIT 10;
SELECT c.name, COUNT(bc.bug_id) as usage_count
  FROM cards c
  LEFT JOIN bug_cards bc ON c.id = bc.card_id
  GROUP BY c.id
  ORDER BY usage_count DESC;
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
   - Optionally click "Generate" next to title (requires LLM)
   - Click "Save Dream"
3. **Search dreams**: Use search box to filter by keywords
4. **Edit/Delete**: Click on dream → Edit or Delete

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

### ✅ Completed Features (Phase 1-2)

- ✅ Dream journal with full CRUD operations
- ✅ Mind dumps for quick thought capture
- ✅ Bug tracking system with status workflow
- ✅ Card system (tarot-like deck) with spreads
- ✅ LLM integration (Ollama, OpenAI, Anthropic)
- ✅ AI-powered title generation and content optimization
- ✅ Card commentary and multi-card insights
- ✅ PIN authentication for privacy
- ✅ User profiles (name, zodiac, MBTI)
- ✅ Database backup functionality
- ✅ Card-bug relationship system
- ✅ Bug notes and conversation history

### Phase 3: Enhanced Insights & Analytics

- **Dream Pattern Analysis**: Detect recurring themes and symbols across dreams
- **Bug Trends**: Visualize bug resolution patterns over time
- **Card Statistics**: Deep analytics on card usage and combinations
- **Cross-Feature Insights**: Connect dreams, bugs, and mind dumps automatically
- **Timeline Visualization**: Visual timeline across all data types

### Phase 4: Enhanced UX & Export

- **Emotion Tagging UI**: Visual tagging system for dream emotions (database field exists)
- **Symbol Highlighting**: Automatic recognition of recurring symbols in dreams
- **Rich Text Editor**: Markdown or WYSIWYG editing for content
- **Export Functionality**: PDF and Markdown export for all data types
- **Batch Operations**: Bulk edit/delete/export across entries
- **Advanced Search**: Filter by date ranges, tags, sleep quality, status

### Phase 5: Security & Sync

- **Encryption**: Password-protect database with SQLCipher
- **Cloud Sync** (Optional): End-to-end encrypted sync for multi-device use
- **Data Import**: Import from other journal apps
- **Versioning**: Track changes to dreams and bugs over time

### Future Enhancements

- **Keyboard Shortcuts**: Power user features and quick navigation
- **Themes**: Multiple color schemes and customization
- **Mobile Companion**: Read-only mobile app for reviewing entries
- **Voice Recording**: Attach voice memos to dreams/bugs
- **Image Attachments**: Add sketches or photos to entries
- **Reminders**: Scheduled prompts for dream recording

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

**Status**: ✅ **v1.0 Feature-Complete** - Dream journal, mind dumps, bug tracking, card system, LLM integration, authentication, and database backup all implemented and ready for use!
