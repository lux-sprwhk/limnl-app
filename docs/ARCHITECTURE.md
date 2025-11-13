# System Architecture

← [Back to README](../README.md)

> Overview of the Limnl application architecture, including data flow, database schema, and project structure.

## Overview

Limnl is built as a Tauri desktop application with a clear separation between:

- **Frontend**: SvelteKit-based UI with client-side routing
- **Backend**: Rust-based Tauri commands for database operations
- **Storage**: Local SQLite database with privacy-first design

## Data Flow

1. User interacts with Svelte component
2. Component calls API client function (`dreamsApi.createDream()`, etc.)
3. API client invokes Tauri command via `@tauri-apps/api`
4. Rust backend receives command, executes database operation
5. Result is serialized to JSON and returned to frontend
6. Component updates UI with new data

## Database

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
