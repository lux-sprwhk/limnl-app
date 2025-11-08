# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Limnl** is a privacy-first desktop application for self-reflection, dream journaling, problem tracking, and personal insights. Built with **Tauri 2.x** (Rust backend) and **SvelteKit** (frontend), using **Panda CSS** for styling and **TypeScript** for type safety.

All data is stored locally in a SQLite database with no cloud sync or telemetry. The app provides:

- **Dream Journal**: Full CRUD operations for dream entries with search, sleep quality tracking, and AI-powered title generation
- **Mind Dumps**: Quick stream-of-consciousness thought capture with word count tracking
- **Bug Tracking**: Personal issue/problem tracking with status workflow (active/resolved/archived), notes, and card associations
- **Card System**: Tarot-like divination deck with 1 and 3-card spreads, AI commentary, and bug relationship tracking
- **Authentication**: Optional PIN-based authentication for privacy
- **User Profiles**: Personalization with name, zodiac sign, and MBTI type
- **LLM Integration**: Flexible BYOLLM approach supporting Ollama (local), OpenAI, Anthropic Claude, or disabled
- **Database Backup**: Export database to any location with interactive file dialogs

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
  - `+page.svelte` - Home page with PIN authentication
  - `+layout.svelte` - App layout with sidebar navigation
  - `settings/+page.svelte` - LLM config, user profile, PIN settings, database backup
  - `help/+page.svelte` - Help and documentation
  - `dreams/` - Dream journal routes:
    - `+page.svelte` - List all dreams with search
    - `new/+page.svelte` - Create new dream with AI title generation
    - `[id]/+page.svelte` - View dream details
    - `[id]/edit/+page.svelte` - Edit dream
  - `bugs/` - Bug tracking routes:
    - `+page.svelte` - List bugs with status filtering (all/active/resolved/archived)
    - `create/+page.svelte` - Create new bug with AI features
    - `discover/+page.svelte` - Card discovery tool for bug insights
    - `[id]/+page.svelte` - Bug detail with notes, cards, conversation history
  - `mind-dumps/` - Mind dump routes:
    - `+page.svelte` - List mind dumps with search
    - `new/+page.svelte` - Create new mind dump
    - `[id]/+page.svelte` - View/edit mind dump
  - `cards/` - Card system routes:
    - `list/+page.svelte` - Browse all cards sorted by usage
    - `draw/+page.svelte` - Draw 1 or 3-card spreads with AI commentary
    - `[id]/+page.svelte` - Card detail page with related bugs

- **API Layer**: `src/lib/api/`
  - `dreams.ts` - Dream CRUD operations
    - `createDream()`, `getDream()`, `getDreams()`, `updateDream()`, `deleteDream()`, `searchDreams()`
  - `bugs.ts` - Bug tracking operations
    - `createBug()`, `getBug()`, `listBugs()`, `updateBug()`, `deleteBug()`
    - `optimizeBugDescription()`, `generateBugTitle()`
  - `cards.ts` - Card operations and relationships
    - `getCard()`, `getCardByName()`, `listCards()`, `listCardsByUsage()`
    - `linkCardToBug()`, `getBugCards()`, `unlinkCardFromBug()`, `clearBugCards()`, `getCardBugs()`
    - `commentOnCard()`, `commentOnMultipleCards()`, `chatWithHistory()`
  - `mind-dumps.ts` - Mind dump operations
    - `createMindDump()`, `getMindDump()`, `listMindDumps()`, `updateMindDump()`, `deleteMindDump()`, `searchMindDumps()`
  - `llm.ts` - LLM provider abstraction
    - `generateTitle()`, `optimizeDescription()`, `commentOnCard()`, `commentOnMultipleCards()`, `chat()`
  - `database.ts` - Database backup operations
    - `backupDatabase()`, `getDatabasePath()`, `backupDatabaseWithDialog()`

- **Types**: `src/lib/types/`
  - `dream.ts` - `Dream`, `CreateDreamInput`, `UpdateDreamInput`
  - `bug.ts` - `Bug`, `CreateBugInput`, `UpdateBugInput`, `BugNote`, `ConversationMessage`
  - `card.ts` - `Card`, `DbCard`, `CardWithCount`
  - `mind-dump.ts` - `MindDump`, `CreateMindDumpInput`, `UpdateMindDumpInput`
  - `llm.ts` - `LLMConfig`, `LLMProvider`, `GenerateTitleRequest`, `GenerateTitleResponse`, `CommentRequest`, `ChatRequest`
  - `auth.ts` - `PinConfig`, `AuthSession`
  - `user.ts` - `UserProfile`, `ZodiacSign`, `MBTIType`

- **Stores** (Svelte 5 runes): `src/lib/stores/`
  - `llm-settings.svelte.ts` - LLM configuration (persisted to localStorage as `limnl-llm-config`)
  - `auth.svelte.ts` - Authentication state, PIN management, session tracking
    - Methods: `authenticate()`, `setupPin()`, `togglePinRequirement()`, `logout()`
  - `user-profile.svelte.ts` - User profile management (persisted to localStorage as `limnl-user-profile`)
    - Methods: `updateProfile()`, `resetProfile()`
    - Fields: name, zodiacSign, mbtiType

- **Components**: `src/lib/components/`
  - `ui/` - Styled UI primitives (Button, Select, Loader)
  - `Navbar.svelte` - Sidebar navigation with active route highlighting
  - `BugNotes.svelte` - Notes editor for bug entries with add/delete functionality
  - `auth/PinInput.svelte` - PIN entry component with masked input

- **Utils**: `src/lib/utils/` - Utility functions
- **Static Assets**: `static/` - Public files

### Backend (Tauri/Rust)

- **Source**: `src-tauri/src/` - Rust application code
  - `lib.rs` - App initialization, registers all 34 Tauri commands
  - `main.rs` - Entry point
  - `commands.rs` - Tauri command handlers (invoked from frontend)
  - `db/` - Database layer:
    - `mod.rs` - Module exports
    - `models.rs` - Data structures for all entities
      - Dreams: `Dream`, `CreateDreamInput`, `UpdateDreamInput`
      - Bugs: `Bug`, `CreateBugInput`, `UpdateBugInput`
      - Mind Dumps: `MindDump`, `CreateMindDumpInput`, `UpdateMindDumpInput`
      - Cards: `Card`, `DbCard`, `CardWithCount`, `BugCard`
    - `connection.rs` - SQLite initialization, schema creation, card seeding from `cards.json`
    - `dreams.rs` - Dream CRUD operations (create, get_all, get_by_id, update, delete, search)
    - `bugs.rs` - Bug CRUD operations with status management
    - `mind_dumps.rs` - Mind dump CRUD operations with word count
    - `cards.rs` - Card operations and bug-card relationship management
  - `llm/` - LLM integration layer:
    - `mod.rs` - Module exports
    - `client.rs` - HTTP client for LLM API calls
    - `providers.rs` - Provider implementations (Ollama, OpenAI, Anthropic)
    - Request/response types for various LLM operations

- **Config**: `src-tauri/tauri.conf.json` - Tauri configuration
  - Window size: 800x600
  - Frontend served from `build/` directory
  - Dev server: `localhost:5173`
  - Plugins: shell, dialog, updater

- **Dependencies**: `src-tauri/Cargo.toml`
  - `tauri 2.9.1`, `serde`, `serde_json` - Core framework
  - `rusqlite 0.32` - SQLite database driver (bundled)
  - `chrono 0.4` - Date/time handling
  - `tokio 1.x` - Async runtime
  - `reqwest 0.12` - HTTP client for LLM APIs
  - `directories 5.0` - Platform-specific paths
  - `thiserror 1.0` - Error handling
  - Plugins: `tauri-plugin-shell`, `tauri-plugin-dialog`, `tauri-plugin-updater`

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

**Schema** (5 tables):

```sql
-- Dream journal entries
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

-- Bug/problem tracking entries
CREATE TABLE bugs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,             -- 'active', 'resolved', or 'archived'
    cards_drawn TEXT,                 -- JSON array of card IDs drawn for this bug
    conversation_history TEXT,        -- JSON array of conversation messages
    notes TEXT,                       -- JSON array of note objects {id, content, timestamp}
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    resolved_at TEXT                  -- Timestamp when status changed to 'resolved'
);

-- Mind dump/quick thought entries
CREATE TABLE mind_dumps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,                       -- Optional title
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL,      -- Automatically calculated
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Card definitions (tarot-like deck)
CREATE TABLE cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,        -- Card name loaded from cards.json
    created_at TEXT NOT NULL
);

-- Many-to-many relationship: bugs ↔ cards
CREATE TABLE bug_cards (
    bug_id INTEGER NOT NULL,
    card_id INTEGER NOT NULL,
    position INTEGER,                 -- Order in which card was drawn (0, 1, 2)
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

**Card Initialization**:

- Cards are automatically seeded from `cards.json` on first database creation
- Card deck is fixed (read-only) - similar to a tarot deck
- Cards are referenced by their unique names

**Inspect Database**:

```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Example queries
SELECT * FROM dreams;
SELECT * FROM bugs WHERE status = 'active';
SELECT * FROM mind_dumps ORDER BY created_at DESC;

# Get card usage statistics
SELECT c.name, COUNT(bc.bug_id) as usage_count
FROM cards c
LEFT JOIN bug_cards bc ON c.id = bc.card_id
GROUP BY c.id
ORDER BY usage_count DESC;

# Get all cards associated with a bug
SELECT c.name, bc.position
FROM bug_cards bc
JOIN cards c ON bc.card_id = c.id
WHERE bc.bug_id = 1
ORDER BY bc.position;
```

### LLM Integration (BYOLLM - Bring Your Own LLM)

**Overview**: Limnl supports optional LLM features across dreams, bugs, and the card system. Users configure their preferred LLM provider in settings.

**Supported Providers**:

- **Ollama** (local, privacy-first, free) - Runs LLM models locally on user's machine
  - Recommended models: `llama3.2`, `mistral`, `phi3`
- **OpenAI** - Requires API key (gpt-4o-mini recommended for cost)
- **Anthropic Claude** - Requires API key (claude-3-5-haiku-20241022 recommended)
- **Disabled** - All LLM features disabled (manual input only)

**Configuration Storage**:

- Stored in browser localStorage (key: `limnl-llm-config`)
- API keys never leave the device (client-side only)
- Settings persist across sessions

**LLM Features**:

1. **Dream Title Generation**: Auto-generate titles from dream content
   - Available in: Dreams creation/edit pages
   - Endpoint: `generate_dream_title` (Rust command)

2. **Dream Description Optimization**: Refine and improve dream descriptions
   - Available in: Dreams edit page
   - Endpoint: `optimize_dream_description`

3. **Bug Title Generation**: Auto-generate titles from bug descriptions
   - Available in: Bug creation page
   - Endpoint: `generate_bug_title`

4. **Bug Description Optimization**: Refine and improve bug descriptions
   - Available in: Bug edit page
   - Endpoint: `optimize_bug_description`

5. **Card Commentary**: Get AI-powered insights on individual cards
   - Available in: Card detail pages, card drawing
   - Endpoint: `comment_on_card`
   - Includes user profile context (zodiac, MBTI)

6. **Multi-Card Commentary**: Get combined insights on multiple cards
   - Available in: 3-card spread drawing
   - Endpoint: `comment_on_multiple_cards`
   - Analyzes card combinations and relationships

7. **Chat with Context**: Multi-turn conversations with card and profile awareness
   - Available in: Bug discovery tool, card pages
   - Endpoint: `chat_with_history`
   - Maintains conversation history
   - Includes user profile (name, zodiac, MBTI) and card context

**Implementation**:

```typescript
// Generate dream title
import { llmApi } from '$lib/api/llm';
const response = await llmApi.generateTitle({
  content: 'I was flying over mountains...',
  entity_type: 'dream'
});

// Comment on a card
import { cardsApi } from '$lib/api/cards';
const comment = await cardsApi.commentOnCard({
  card_name: 'The Void',
  user_name: 'Alex',
  zodiac_sign: 'Scorpio',
  mbti_type: 'INTJ'
});

// Chat with card context
const chatResponse = await cardsApi.chatWithHistory({
  message: 'How does this card relate to my bug?',
  conversation_history: [...],
  card_names: ['The Mirror', 'The Gate'],
  user_profile: { name: 'Alex', zodiacSign: 'Scorpio', mbtiType: 'INTJ' }
});

// Check if LLM is configured
import { llmSettings } from '$lib/stores/llm-settings';
if (llmSettings.isConfigured) {
  // LLM features available
}
```

**Provider-Specific Notes**:

- **Ollama**: User must install Ollama separately and pull models (e.g., `ollama pull llama3.2`)
  - Default endpoint: `http://localhost:11434`
- **Cloud APIs**: User provides their own API key via Settings page
- **Privacy**: Ollama keeps all data local; cloud providers will receive content for processing
- **User Profiles**: Card commentary and chat include zodiac/MBTI context for personalized responses

### Data Flow

1. User interacts with Svelte component (e.g., dream form)
2. Component calls API client function: `dreamsApi.createDream(input)`
3. API client invokes Tauri command: `invoke('create_dream', { input })`
4. Rust backend receives command in `commands.rs`
5. Command handler calls database function: `db::dreams::create()`
6. Database operation executes SQLite query via `rusqlite`
7. Result is serialized to JSON and returned to frontend
8. Component receives result and updates UI

### Authentication System

**Overview**: Optional PIN-based authentication for privacy protection.

**Implementation**:

- **Frontend-only**: All auth logic in browser (no backend validation)
- **PIN Storage**: SHA-256 hash stored in localStorage (`limnl-pin-config`)
- **Session Management**: Session tracked in sessionStorage (`limnl-auth-session`)
- **Session Duration**: 1 hour timeout, auto-logout on app close

**Store**: `src/lib/stores/auth.svelte.ts`

```typescript
import { auth } from '$lib/stores/auth.svelte';

// Setup PIN (first-time or change)
await auth.setupPin('1234');

// Authenticate
const success = await auth.authenticate('1234');

// Toggle PIN requirement
auth.togglePinRequirement(false);

// Logout
auth.logout();

// Check state
if (auth.isAuthenticated) { /* user is logged in */ }
if (auth.requirePin) { /* PIN is required */ }
if (auth.hasPin) { /* PIN has been set */ }
```

**Component**: `src/lib/components/auth/PinInput.svelte`

- Masked numeric input (4+ digits)
- Shows/hides PIN toggle
- Used in home page for authentication

**Settings Integration**:

- Settings page allows toggling PIN requirement on/off
- Can change PIN from settings
- Security section shows PIN status

### User Profile System

**Overview**: User personalization for LLM-powered features.

**Fields**:

- `name`: User's name (optional)
- `zodiacSign`: One of 12 zodiac signs (Aries, Taurus, etc.)
- `mbtiType`: One of 16 MBTI types (INTJ, ENFP, etc.)

**Storage**: localStorage (`limnl-user-profile`)

**Store**: `src/lib/stores/user-profile.svelte.ts`

```typescript
import { userProfile } from '$lib/stores/user-profile.svelte';

// Update profile
userProfile.updateProfile({
  name: 'Alex',
  zodiacSign: 'Scorpio',
  mbtiType: 'INTJ'
});

// Access fields
console.log(userProfile.name);
console.log(userProfile.zodiacSign);
console.log(userProfile.mbtiType);

// Reset to defaults
userProfile.resetProfile();
```

**LLM Integration**:

- Profile data sent to LLM for personalized responses
- Used in card commentary and chat features
- Example: "As a Scorpio INTJ, The Mirror suggests..."

### Card System Architecture

**Overview**: Fixed tarot-like deck for divination and insight generation.

**Card Initialization**:

- Cards loaded from `cards.json` on first database creation
- Deck is read-only (users cannot add/edit/delete cards)
- Each card has a unique name (e.g., "The Void", "The Mirror", "The Gate")

**Card Usage**:

1. **Card Browser** (`/cards/list`): Browse all cards sorted by usage in bugs
2. **Card Drawing** (`/cards/draw`): Draw 1 or 3-card spreads for insights
3. **Card Detail** (`/cards/[id]`): View card info and all related bugs
4. **Bug Discovery** (`/bugs/discover`): Draw cards specifically for bug insights

**Bug-Card Relationships**:

- Many-to-many relationship via `bug_cards` join table
- Cards can be linked to multiple bugs
- Each card-bug link has a `position` (0, 1, 2 for 3-card spreads)
- Cards sorted by usage count in bug tracking

**LLM Features**:

- **Single Card Commentary**: Personalized insights based on user profile
- **Multi-Card Commentary**: Combined analysis of 2-3 cards
- **Chat with Context**: Multi-turn conversations with card context

**Example Flow**:

```typescript
// User draws 3 cards for a bug
const cards = ['The Void', 'The Mirror', 'The Gate'];

// Get LLM commentary on the spread
const commentary = await cardsApi.commentOnMultipleCards({
  card_names: cards,
  user_name: 'Alex',
  zodiac_sign: 'Scorpio',
  mbti_type: 'INTJ'
});

// Link cards to bug
for (let i = 0; i < cards.length; i++) {
  const card = await cardsApi.getCardByName(cards[i]);
  await cardsApi.linkCardToBug(bugId, card.id, i);
}

// Later: Get all bugs using "The Void"
const relatedBugs = await cardsApi.getCardBugs(voidCardId);
```

### Bug Tracking System

**Overview**: Personal issue/problem tracker with card associations and notes.

**Status Workflow**:

- `active`: Currently working on (default)
- `resolved`: Fixed/completed
- `archived`: No longer relevant

**Features**:

- **Notes**: Array of note objects with timestamps (managed by BugNotes component)
- **Card Associations**: Link 1-3 cards to bugs for insights via `bug_cards` table
- **Conversation History**: Track chat messages (JSON array)
- **Resolved Timestamp**: Auto-set when status changes to 'resolved'

**Bug Discovery Tool** (`/bugs/discover`):

- Draw cards for a specific bug
- Get LLM commentary on card spread
- Chat with AI about the bug using card context
- Maintains conversation history

**Example Workflow**:

```typescript
// Create bug
const bug = await bugsApi.createBug({
  title: 'Memory Leak',
  description: 'App consumes too much memory over time',
  status: 'active'
});

// Add notes via BugNotes component
// Notes are updated through bug update endpoint

// Draw cards for insights
const cards = await drawCardsForBug(bug.id);

// Get commentary
const commentary = await cardsApi.commentOnMultipleCards({
  card_names: cards.map(c => c.name),
  ...userProfile
});

// Mark as resolved
await bugsApi.updateBug(bug.id, {
  ...bug,
  status: 'resolved'
});
// resolved_at is automatically set

// Later: Archive
await bugsApi.updateBug(bug.id, {
  ...bug,
  status: 'archived'
});
```

### Mind Dumps Feature

**Overview**: Quick stream-of-consciousness journaling for rapid thought capture.

**Features**:

- **Optional Title**: Can be left blank for pure stream-of-consciousness
- **Word Count**: Automatically calculated and stored
- **Full-Text Search**: Search across title and content
- **Timestamp Tracking**: Created and updated timestamps

**Use Cases**:

- Morning pages / freewriting
- Brainstorming sessions
- Capturing fleeting thoughts
- Processing emotions quickly

**Example**:

```typescript
import { mindDumpsApi } from '$lib/api/mind-dumps';

// Create without title
const dump = await mindDumpsApi.createMindDump({
  content: 'Just woke up thinking about...',
  title: undefined
});

// Word count is calculated automatically
console.log(dump.word_count); // e.g., 47

// Search across all mind dumps
const results = await mindDumpsApi.searchMindDumps('thinking');
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
- **Database**: Automatically created on first run at platform-specific location
- **Privacy-First**: All data stored locally, no cloud sync, no telemetry
- **Tauri Commands**: Defined in `src-tauri/src/commands.rs`, registered in `lib.rs`
- **Web-Only Mode**: `pnpm dev` won't have database access (Tauri commands unavailable)
- **Full App Mode**: `pnpm tauri:dev` required for complete functionality

## Tauri Commands Reference

All 34 available commands (invoked from frontend via `@tauri-apps/api/core::invoke`):

### Dream Commands (6)

```rust
create_dream(input: CreateDreamInput) -> Result<Dream>
get_dream(id: i64) -> Result<Dream>
list_dreams() -> Result<Vec<Dream>>                          // Sorted by date_occurred DESC
update_dream(id: i64, input: UpdateDreamInput) -> Result<Dream>
delete_dream(id: i64) -> Result<()>
search_dreams(query: String) -> Result<Vec<Dream>>           // Searches title and content
```

### Dream LLM Commands (2)

```rust
generate_dream_title(request: GenerateTitleRequest) -> Result<GenerateTitleResponse>
optimize_dream_description(request: OptimizeDescriptionRequest) -> Result<OptimizeDescriptionResponse>
```

### Bug Commands (5)

```rust
create_bug(input: CreateBugInput) -> Result<Bug>
get_bug(id: i64) -> Result<Bug>
list_bugs() -> Result<Vec<Bug>>                              // Sorted by created_at DESC
update_bug(id: i64, input: UpdateBugInput) -> Result<Bug>
delete_bug(id: i64) -> Result<()>
```

### Bug LLM Commands (2)

```rust
generate_bug_title(request: GenerateTitleRequest) -> Result<GenerateTitleResponse>
optimize_bug_description(request: OptimizeDescriptionRequest) -> Result<OptimizeDescriptionResponse>
```

### Mind Dump Commands (6)

```rust
create_mind_dump(input: CreateMindDumpInput) -> Result<MindDump>
get_mind_dump(id: i64) -> Result<MindDump>
list_mind_dumps() -> Result<Vec<MindDump>>                   // Sorted by created_at DESC
update_mind_dump(id: i64, input: UpdateMindDumpInput) -> Result<MindDump>
delete_mind_dump(id: i64) -> Result<()>
search_mind_dumps(query: String) -> Result<Vec<MindDump>>    // Searches title and content
```

### Card Commands (4)

```rust
get_card(id: i64) -> Result<Card>
get_card_by_name(name: String) -> Result<DbCard>
list_cards() -> Result<Vec<Card>>                            // All cards alphabetically
list_cards_by_usage() -> Result<Vec<CardWithCount>>          // Sorted by usage in bugs DESC
```

### Card Relationship Commands (5)

```rust
link_card_to_bug(bug_id: i64, card_id: i64, position: Option<i32>) -> Result<()>
get_bug_cards(bug_id: i64) -> Result<Vec<BugCard>>          // Cards associated with a bug
unlink_card_from_bug(bug_id: i64, card_id: i64) -> Result<()>
clear_bug_cards(bug_id: i64) -> Result<()>                  // Remove all cards from a bug
get_card_bugs(card_id: i64) -> Result<Vec<Bug>>             // All bugs using this card
```

### Card LLM Commands (3)

```rust
comment_on_card(request: CommentOnCardRequest) -> Result<CommentResponse>
comment_on_multiple_cards(request: CommentOnMultipleCardsRequest) -> Result<CommentResponse>
chat_with_history(request: ChatRequest) -> Result<ChatResponse>
```

### Database Commands (2)

```rust
backup_database(destination: String) -> Result<()>           // Copy database to path
get_database_path() -> Result<String>                        // Get current database location
```

### Example Frontend Usage

**Dreams**:

```typescript
import { dreamsApi } from '$lib/api/dreams';

const dream = await dreamsApi.createDream({
  date_occurred: '2025-01-15',
  title: 'Flying Dream',
  content: 'I was soaring above mountains...',
  sleep_quality: 4
});

const results = await dreamsApi.searchDreams('flying');
```

**Bugs**:

```typescript
import { bugsApi } from '$lib/api/bugs';

const bug = await bugsApi.createBug({
  title: 'Memory Issue',
  description: 'App crashes when loading large files',
  status: 'active'
});

await bugsApi.updateBug(bug.id, {
  ...bug,
  status: 'resolved'
});
```

**Cards**:

```typescript
import { cardsApi } from '$lib/api/cards';

// Get card by name
const card = await cardsApi.getCardByName('The Void');

// Link card to bug
await cardsApi.linkCardToBug(bugId, cardId, 0);

// Get all bugs using a card
const bugs = await cardsApi.getCardBugs(cardId);

// Get card commentary
const commentary = await cardsApi.commentOnCard({
  card_name: 'The Mirror',
  user_name: 'Alex',
  zodiac_sign: 'Scorpio',
  mbti_type: 'INTJ'
});
```

**Mind Dumps**:

```typescript
import { mindDumpsApi } from '$lib/api/mind-dumps';

const mindDump = await mindDumpsApi.createMindDump({
  title: 'Morning Thoughts',
  content: 'Stream of consciousness...'
});

const results = await mindDumpsApi.searchMindDumps('thoughts');
```

**Database Backup**:

```typescript
import { databaseApi } from '$lib/api/database';

// Interactive backup with file dialog
await databaseApi.backupDatabaseWithDialog();

// Or backup to specific path
const dbPath = await databaseApi.getDatabasePath();
await databaseApi.backupDatabase('/path/to/backup/limnl-backup.db');
```
