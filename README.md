# lmnl-app

A modern desktop application built with a powerful tech stack.

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
lmnl-app/
├── src/
│   ├── lib/
│   │   ├── components/
│   │   │   └── ui/          # Reusable UI components
│   │   └── utils/           # Utility functions
│   ├── routes/              # SvelteKit routes
│   └── app.css              # Global styles
├── src-tauri/               # Tauri Rust backend
├── styled-system/           # Generated Panda CSS files
└── static/                  # Static assets
```

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/) (recommended) or npm
- [Rust](https://www.rust-lang.org/) (for Tauri)

### Installation

```bash
# Install dependencies
pnpm install

# Generate Panda CSS
pnpm panda codegen
```

### Development

```bash
# Run in development mode (web only)
pnpm dev

# Run as Tauri desktop app
pnpm tauri:dev
```

The app will be available at `http://localhost:5173`

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

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Panda CSS Documentation](https://panda-css.com/docs)
- [bits-ui Documentation](https://bits-ui.com/)
- [Vitest Documentation](https://vitest.dev/)
