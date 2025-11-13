# Development Guide

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

