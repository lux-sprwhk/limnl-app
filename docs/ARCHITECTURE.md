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

**Schema** (8 tables total, managed via migrations):

**Table Organization**:
- **Core Entities** (4): `dreams`, `bugs`, `mind_dumps`, `cards`
- **Dream Analysis** (3): `dream_analyses`, `dream_analysis_cards`, `dream_creative_prompts`
- **Relationships** (1): `bug_cards`

**Migration History**:
- Migration 001: Core tables (dreams, bugs, mind_dumps, cards, dream analysis, relationships)
- Migration 002: Added dream metadata fields (is_recurring, last_occurrence_period, is_lucid)

```sql
-- Dream journal entries
CREATE TABLE dreams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_recorded TEXT NOT NULL,      -- When the dream was recorded
    date_occurred TEXT NOT NULL,      -- When the dream actually happened
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    emotions_tags TEXT,               -- Future: JSON array of emotion tags
    sleep_quality INTEGER,            -- 1-5 rating (optional)
    is_recurring INTEGER DEFAULT 0,   -- Whether this dream has occurred before (0=false, 1=true)
    last_occurrence_period TEXT,      -- When it last occurred (e.g., "last_week", "months_ago")
    is_lucid INTEGER DEFAULT 0,       -- Whether dreamer was aware they were dreaming
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- AI-powered dream analysis (cached results)
CREATE TABLE dream_analyses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dream_id INTEGER NOT NULL UNIQUE, -- One analysis per dream
    themes_patterns TEXT NOT NULL,    -- Themes and recurring patterns identified
    emotional_analysis TEXT NOT NULL, -- Emotional tone and feelings analysis
    narrative_summary TEXT NOT NULL,  -- Narrative arc and story structure
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dream_id) REFERENCES dreams(id) ON DELETE CASCADE
);

-- Symbol cards linked to dream analyses
CREATE TABLE dream_analysis_cards (
    dream_analysis_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    relevance_note TEXT,              -- Why this card is relevant to the dream
    created_at TEXT NOT NULL,
    PRIMARY KEY (dream_analysis_id, card_id),
    FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE,
    FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

-- AI-generated creative prompts based on dream analysis
CREATE TABLE dream_creative_prompts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dream_analysis_id INTEGER NOT NULL UNIQUE,
    image_prompts TEXT NOT NULL,      -- JSON array of image generation prompts
    music_prompts TEXT NOT NULL,      -- JSON array of music/soundscape ideas
    story_prompts TEXT NOT NULL,      -- JSON array of story extension prompts
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (dream_analysis_id) REFERENCES dream_analyses(id) ON DELETE CASCADE
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
CREATE INDEX idx_dream_analyses_dream_id ON dream_analyses(dream_id);
```

**Inspect Database**:

```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Query examples
SELECT * FROM dreams;
SELECT * FROM bugs WHERE status = 'active';
SELECT * FROM mind_dumps ORDER BY created_at DESC LIMIT 10;

# Check migration status
SELECT * FROM schema_version ORDER BY version;

# Card usage statistics
SELECT c.name, COUNT(bc.bug_id) as usage_count
  FROM cards c
  LEFT JOIN bug_cards bc ON c.id = bc.card_id
  GROUP BY c.id
  ORDER BY usage_count DESC;

# Dreams with analyses
SELECT d.title, da.themes_patterns
  FROM dreams d
  JOIN dream_analyses da ON d.id = da.dream_id
  ORDER BY d.date_occurred DESC;

# Symbol cards for a dream analysis
SELECT c.name, dac.relevance_note
  FROM dream_analysis_cards dac
  JOIN cards c ON dac.card_id = c.id
  WHERE dac.dream_analysis_id = 1;
```

## Database Migration System

Limnl uses a **lightweight, file-based SQL migration framework** for schema versioning.

### Architecture

**Components**:
- `src-tauri/migrations/*.sql` - Sequential SQL migration files
- `src-tauri/src/db/migrations.rs` - Migration runner
- `schema_version` table - Tracks applied migrations

**Migration Flow**:

```
App Startup
    ↓
Database::new()
    ↓
run_migrations()
    ↓
Check schema_version table (create if missing)
    ↓
Get current version (0 if none applied)
    ↓
For each pending migration:
  - Execute SQL
  - Record version + timestamp
    ↓
Database ready
```

**Key Features**:
- **Automatic**: Runs on every app startup
- **Sequential**: Migrations numbered 001, 002, 003...
- **Idempotent**: Uses `IF NOT EXISTS` for safety
- **Forward-only**: No rollback needed (desktop app)
- **Tested**: Comprehensive test suite

**Version Tracking Table**:

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL  -- Unix timestamp
);
```

**Adding Migrations**:

1. Create `src-tauri/migrations/00X_feature.sql`
2. Register in `MIGRATIONS` array in `migrations.rs`
3. Test with `cargo test db::migrations::tests`
4. Deploy - auto-applies on next startup

See `docs/DEVELOPMENT.md#database-migrations` for detailed workflow.

## Dream Analysis System

> **Added in**: Recent development (post v0.1.0)
> **Status**: Fully implemented with database migration support

The dream analysis system provides **AI-powered insights** into dream content with symbol card associations and creative prompts.

### Feature Architecture

**Three-Tier Analysis**:
1. **Core Analysis** - Themes, emotions, narrative (stored in `dream_analyses`)
2. **Symbol Cards** - Relevant cards linked to the analysis (in `dream_analysis_cards`)
3. **Creative Prompts** - AI-generated prompts for art/music/stories (in `dream_creative_prompts`)

### Data Flow

**Generating Dream Analysis**:

```
User clicks "Analyze Dream" on dream detail page
    ↓
Frontend: dreamsApi.generateAnalysis({ dream_id, dream_title, dream_content, ... })
    ↓
Backend: generate_dream_analysis Tauri command
    ↓
LLM Client: Sends structured prompt to configured LLM provider
    ↓
LLM Response: { themes_patterns, emotional_analysis, narrative_summary, relevant_cards[] }
    ↓
Database: Creates dream_analysis record
    ↓
For each relevant card:
  - Look up card by name
  - Insert into dream_analysis_cards with relevance_note
    ↓
Return: DreamAnalysisWithCards { analysis, cards[] }
    ↓
Frontend: Display analysis sections + linked symbol cards
```

**Generating Creative Prompts** (requires existing analysis):

```
User clicks "Generate Creative Prompts" on dream detail page
    ↓
Frontend: dreamsApi.generateCreativePrompts({ dream_analysis_id, themes, emotions, narrative })
    ↓
Backend: generate_dream_creative_prompts Tauri command
    ↓
LLM Client: Sends analysis data to LLM
    ↓
LLM Response: { image_prompts[], music_prompts[], story_prompts[] }
    ↓
Database: Creates dream_creative_prompts record (JSON arrays as TEXT)
    ↓
Return: DreamCreativePrompts
    ↓
Frontend: Parse JSON arrays, display in categorized sections
```

### Database Relationships

```
dreams (1) ←→ (1) dream_analyses
                      ↓ (1)
                      ↓
                     (M) dream_analysis_cards ←→ (1) cards
                      ↓ (1)
                      ↓
                     (1) dream_creative_prompts
```

**Cascade Deletion**:
- Deleting a dream → deletes analysis → deletes analysis cards + creative prompts
- All relationships use `ON DELETE CASCADE`

### Caching Strategy

**Analysis Caching**:
- One analysis per dream (`dream_id` is UNIQUE in `dream_analyses`)
- Fetching analysis checks for existing record first
- Regenerating analysis replaces the old one (via transaction)

**Why Cache?**:
- LLM API calls are expensive/slow (cost: varies by provider)
- Analysis results are deterministic for same input
- Allows offline viewing after initial generation

**Performance Considerations**:

| Feature | Requires LLM | Cached | Offline Access | Avg. Latency |
|---------|--------------|--------|----------------|--------------|
| Dream Analysis | Yes | Yes | After generation | 5-15s (varies by provider/model) |
| Creative Prompts | Yes | Yes | After generation | 5-10s |
| Symbol Cards | No (pre-seeded) | N/A | Always | Instant |

**Cost Implications**:
- **Ollama (local)**: Free, but requires local compute resources
- **OpenAI**: ~$0.01-0.05 per dream analysis (gpt-4o-mini)
- **Anthropic Claude**: ~$0.02-0.08 per dream analysis (claude-3-5-haiku)
- **Bulk Migration**: Analyzing 100 dreams could cost $1-8 depending on provider
- **Recommendation**: Use Ollama for bulk operations, cloud APIs for quality

### Symbol Card Selection

**How Cards Are Chosen**:
- LLM receives card deck context in system prompt
- Returns array of card names with relevance notes
- Backend validates card names exist in `cards` table
- Invalid card names are skipped (graceful degradation)

**Example LLM Response**:
```json
{
  "themes_patterns": "Recurring theme of flight representing freedom...",
  "emotional_analysis": "Joy mixed with anxiety about letting go...",
  "narrative_summary": "The dream follows a classic hero's journey arc...",
  "relevant_cards": [
    { "name": "The Wings", "relevance": "Represents the desire for freedom" },
    { "name": "The Mirror", "relevance": "Self-reflection on identity" }
  ]
}
```

### API Endpoints

**Dream Analysis** (6 new commands):

```rust
// Generate new analysis (or replace existing)
generate_dream_analysis(request: GenerateDreamAnalysisRequest) -> DreamAnalysisWithCards

// Fetch cached analysis with cards
get_dream_analysis_with_cards(dream_id: i64) -> Option<DreamAnalysisWithCards>

// Generate creative prompts (requires existing analysis)
generate_dream_creative_prompts(request: GenerateCreativePromptsRequest) -> DreamCreativePrompts

// Fetch cached creative prompts
get_dream_creative_prompts(dream_analysis_id: i64) -> Option<DreamCreativePrompts>
```

**Frontend API** (TypeScript):

```typescript
import { dreamsApi } from '$lib/api/dreams';

// Generate analysis
const analysis = await dreamsApi.generateAnalysis({
  dream_id: 123,
  dream_title: "Flying Over Mountains",
  dream_content: "I was soaring...",
  sleep_quality: 4,
  config: llmSettings.toConfig()
});

// Fetch existing analysis
const cached = await dreamsApi.getAnalysisWithCards(dreamId);

// Generate creative prompts
const prompts = await dreamsApi.generateCreativePrompts({
  dream_analysis_id: analysis.analysis.id,
  themes_patterns: analysis.analysis.themes_patterns,
  emotional_analysis: analysis.analysis.emotional_analysis,
  narrative_summary: analysis.analysis.narrative_summary,
  config: llmSettings.toConfig()
});

// Fetch cached prompts (returns parsed arrays)
const cachedPrompts = await dreamsApi.getCreativePrompts(analysisId);
// Returns: { image_prompts: string[], music_prompts: string[], story_prompts: string[] }
```

### Type Definitions

**Core Types** (`src/lib/types/dream.ts`):

```typescript
interface DreamAnalysis {
  id?: number;
  dream_id: number;
  themes_patterns: string;
  emotional_analysis: string;
  narrative_summary: string;
  created_at: string;
  updated_at: string;
}

interface DreamAnalysisCard {
  dream_analysis_id: number;
  card_id: number;
  card_name: string;
  relevance_note?: string;
  created_at: string;
}

interface DreamAnalysisWithCards {
  analysis: DreamAnalysis;
  cards: DreamAnalysisCard[];
}

interface DreamCreativePromptsData {
  image_prompts: string[];
  music_prompts: string[];
  story_prompts: string[];
}
```

### UI Integration

**Dream Detail Page** (`src/routes/dreams/[id]/+page.svelte`):

1. Loads dream on mount
2. Fetches existing analysis (if any)
3. Shows "Analyze Dream" button (if LLM configured and no analysis)
4. Displays analysis sections when available
5. Shows "Generate Creative Prompts" button (if analysis exists)
6. Displays creative prompts in categorized tabs/sections

### Data Migration Tool

**Purpose**: Backfill analyses for existing dreams

**Binary**: `src-tauri/src/bin/migrate-dream-analysis.rs`

**Workflow**:
1. Fetches all dreams without analysis
2. For each dream:
   - Generates analysis via LLM
   - Links symbol cards
   - Logs success/failure
3. Rate limits to avoid API throttling
4. Provides dry-run mode

**Usage**:
```bash
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2
cd src-tauri
cargo run --bin migrate-dream-analysis -- --limit 10
```

See `docs/DEVELOPMENT.md#data-migration-tool` for details.

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
│   │   │   ├── connection.rs    # Database initialization & migrations
│   │   │   ├── migrations.rs    # Migration runner with version tracking
│   │   │   ├── dreams.rs        # Dream CRUD operations
│   │   │   ├── bugs.rs          # Bug CRUD operations
│   │   │   ├── cards.rs         # Card operations & relationships
│   │   │   └── mind_dumps.rs    # Mind dump CRUD operations
│   │   ├── llm/
│   │   │   ├── mod.rs           # LLM module exports
│   │   │   ├── client.rs        # HTTP client for LLM APIs
│   │   │   └── providers.rs     # Ollama, OpenAI, Anthropic implementations
│   │   ├── bin/
│   │   │   └── migrate-dream-analysis.rs  # Data migration tool for backfilling
│   │   ├── commands.rs          # Tauri command handlers (40 commands)
│   │   ├── lib.rs               # App initialization
│   │   └── main.rs              # Entry point
│   ├── migrations/
│   │   ├── 001_initial.sql      # Initial schema (8 tables)
│   │   ├── 002_example.sql.example  # Template for new migrations
│   │   └── README.md            # Migration documentation
│   └── Cargo.toml               # Rust dependencies
├── styled-system/               # Generated Panda CSS files (DO NOT EDIT)
├── static/                      # Static assets
└── cards.json                   # Card deck definitions
```

## Tauri Commands Reference

Complete list of all **40 Tauri commands** available for frontend invocation.

### Dream Commands (6)

**CRUD Operations**:
```rust
create_dream(input: CreateDreamInput) -> Dream
get_dream(id: i64) -> Option<Dream>
list_dreams(limit: Option<i64>, offset: Option<i64>) -> Vec<Dream>
update_dream(input: UpdateDreamInput) -> Option<Dream>
delete_dream(id: i64) -> bool
search_dreams(query: String) -> Vec<Dream>
```

### Dream LLM Commands (2)

**Title & Description**:
```rust
generate_dream_title(request: GenerateTitleRequest) -> GenerateTitleResponse
optimize_dream_description(request: OptimizeDescriptionRequest) -> OptimizeDescriptionResponse
```

### Dream Analysis Commands (4)

**AI-Powered Analysis**:
```rust
generate_dream_analysis(request: GenerateDreamAnalysisRequest) -> DreamAnalysisWithCards
get_dream_analysis_with_cards(dream_id: i64) -> Option<DreamAnalysisWithCards>
generate_dream_creative_prompts(request: GenerateCreativePromptsRequest) -> DreamCreativePrompts
get_dream_creative_prompts(dream_analysis_id: i64) -> Option<DreamCreativePrompts>
```

### Bug Commands (5)

**CRUD Operations**:
```rust
create_bug(input: CreateBugInput) -> Bug
get_bug(id: i64) -> Option<Bug>
list_bugs() -> Vec<Bug>
update_bug(input: UpdateBugInput) -> Option<Bug>
delete_bug(id: i64) -> bool
```

### Bug LLM Commands (2)

**Title & Description**:
```rust
generate_bug_title(request: GenerateTitleRequest) -> GenerateTitleResponse
optimize_bug_description(request: OptimizeDescriptionRequest) -> OptimizeDescriptionResponse
```

### Mind Dump Commands (6)

**CRUD Operations**:
```rust
create_mind_dump(input: CreateMindDumpInput) -> MindDump
get_mind_dump(id: i64) -> Option<MindDump>
list_mind_dumps() -> Vec<MindDump>
update_mind_dump(input: UpdateMindDumpInput) -> Option<MindDump>
delete_mind_dump(id: i64) -> bool
search_mind_dumps(query: String) -> Vec<MindDump>
```

### Card Commands (4)

**Card Retrieval**:
```rust
get_card(id: i64) -> Option<Card>
get_card_by_name(name: String) -> Option<DbCard>
list_cards() -> Vec<Card>
list_cards_by_usage() -> Vec<CardWithCount>
```

### Card Relationship Commands (6)

**Bug-Card Associations**:
```rust
create_bug_with_cards(input: CreateBugInput, card_ids: Vec<i64>) -> Bug
link_card_to_bug(bug_id: i64, card_id: i64, position: Option<i32>) -> ()
get_bug_cards(bug_id: i64) -> Vec<BugCard>
unlink_card_from_bug(bug_id: i64, card_id: i64) -> ()
clear_bug_cards(bug_id: i64) -> ()
get_card_bugs(card_id: i64) -> Vec<Bug>
```

### Card LLM Commands (3)

**AI Commentary & Chat**:
```rust
comment_on_card(request: CommentOnCardRequest) -> CardCommentaryResponse
comment_on_multiple_cards(request: CommentOnMultipleCardsRequest) -> CardCommentaryResponse
chat_with_history(request: ChatRequest) -> ChatResponse
```

### Database Commands (2)

**Backup & Utilities**:
```rust
backup_database(destination: String) -> ()
get_database_path() -> String
```

### Usage Example

**TypeScript/Frontend**:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Dream, CreateDreamInput } from '$lib/types/dream';

// Direct invocation
const dream = await invoke<Dream>('create_dream', {
  input: {
    date_occurred: '2025-01-15',
    title: 'Flying Dream',
    content: 'I was soaring...',
    sleep_quality: 4
  }
});

// Via API wrapper (recommended)
import { dreamsApi } from '$lib/api/dreams';
const dreams = await dreamsApi.list(10, 0); // limit=10, offset=0
const analysis = await dreamsApi.generateAnalysis({ ... });
```

**Command Organization**:
- Total: **40 commands**
- Dreams: 12 commands (6 CRUD + 2 LLM + 4 analysis)
- Bugs: 13 commands (5 CRUD + 2 LLM + 6 card relationships)
- Mind Dumps: 6 commands (CRUD only)
- Cards: 7 commands (4 retrieval + 3 LLM)
- Database: 2 commands

All commands are defined in `src-tauri/src/commands.rs` and registered in `src-tauri/src/lib.rs`.

---

## Architecture Summary

**System Metrics**:
- **Total Tauri Commands**: 40
- **Database Tables**: 8 (managed via migrations)
- **Migration Version**: 2 (002_add_dream_metadata.sql)
- **Frontend Routes**: 15+ pages (SvelteKit file-based routing)
- **LLM Providers Supported**: 4 (Ollama, OpenAI, Anthropic, Disabled)
- **Storage**: Local SQLite (no cloud sync)
- **Platform**: Desktop (Linux, macOS, Windows via Tauri)

**Key Features**:
- ✅ Dream journal with full-text search
- ✅ AI-powered dream analysis (themes, emotions, narrative)
- ✅ Symbol card system (tarot-like deck)
- ✅ Creative prompt generation (image/music/story)
- ✅ Bug/problem tracking with card associations
- ✅ Mind dumps (stream-of-consciousness journaling)
- ✅ PIN-based authentication (optional)
- ✅ User profiles (name, zodiac, MBTI)
- ✅ Database backup/export
- ✅ Schema versioning with migrations

**Technology Stack**:
- **Backend**: Rust + Tauri 2.x + SQLite (rusqlite)
- **Frontend**: SvelteKit + TypeScript + Panda CSS + bits-ui
- **LLM Integration**: HTTP client with provider abstraction
- **Testing**: Vitest (frontend) + Rust built-in tests (backend)
- **Package Manager**: pnpm (required)
