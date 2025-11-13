# Rust Binary Scripts

This directory contains standalone Rust binaries for development and maintenance tasks.

## Available Scripts

### migrate-dream-analysis

Generates dream analysis for all dreams that don't have analysis yet.

**Quick Usage:**

```bash
# From src-tauri directory
cd src-tauri

# Configure LLM
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2

# Run the script
cargo run --bin migrate-dream-analysis

# With options
cargo run --bin migrate-dream-analysis -- --dry-run
cargo run --bin migrate-dream-analysis -- --limit 5
cargo run --bin migrate-dream-analysis -- --help
```

**See also:**
- `MIGRATION.md` (project root) - Complete user documentation
- `scripts/README.md` - Technical notes

## Adding New Binaries

To add a new binary:

1. Create `src/bin/your-script.rs`
2. Import from the library: `use lmnl_app_lib::{...};`
3. Build with: `cargo build --bin your-script`
4. Run with: `cargo run --bin your-script`

Binaries automatically have access to all public modules from the main library (`lmnl_app_lib`).
