# Dream Analysis Migration Script

This directory contains documentation for the dream analysis migration script.

## Overview

The dream analysis migration script is a Rust binary located at `src-tauri/src/bin/migrate-dream-analysis.rs`. It generates dream analysis for all existing dreams that don't have one yet.

## Usage

See the main [MIGRATION.md](../MIGRATION.md) file in the project root for complete documentation.

### Quick Reference

```bash
# From project root
cd src-tauri

# Configure LLM via environment variables
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2

# Run the script
cargo run --bin migrate-dream-analysis

# With options
cargo run --bin migrate-dream-analysis -- --dry-run
cargo run --bin migrate-dream-analysis -- --limit 5
cargo run --bin migrate-dream-analysis -- --help
```

## Why Rust Instead of TypeScript?

The original plan included a TypeScript script (`migrate-dream-analysis.ts`), but it has limitations:

1. **Needs Tauri Runtime**: Can't run in Node.js, requires full Tauri environment
2. **Needs localStorage**: LLM config is stored in browser storage
3. **Complex Setup**: Would require headless Tauri window or IPC bridge

The Rust binary is simpler because it:

1. **Direct Database Access**: Uses `rusqlite` to read/write SQLite directly
2. **Environment Variables**: LLM config via `export` commands
3. **Standalone**: Runs independently without Tauri app
4. **Dev-Friendly**: Standard Cargo tooling, easy to debug

## Architecture

The script:

1. Reads LLM configuration from environment variables
2. Opens the SQLite database using the same path logic as the app
3. Uses the existing database modules (`src-tauri/src/db/*`)
4. Uses the existing LLM client (`src-tauri/src/llm/client.rs`)
5. Directly inserts analysis records into the database

## Files

- `src-tauri/src/bin/migrate-dream-analysis.rs` - The migration script
- `MIGRATION.md` - Complete user documentation
- `scripts/README.md` - This file (technical notes)
- `scripts/migrate-dream-analysis.ts` - Legacy/reference TypeScript version (not functional)

## Development Notes

### Testing the Script

```bash
cd src-tauri

# Compile only
cargo build --bin migrate-dream-analysis

# Run with debug output
RUST_LOG=debug cargo run --bin migrate-dream-analysis -- --dry-run

# Test with test database
DATABASE_PATH=/tmp/test.db cargo run --bin migrate-dream-analysis
```

### Adding New LLM Providers

To add a new provider:

1. Add variant to `limnl_app::llm::LLMConfig` enum
2. Implement in `limnl_app::llm::client::generate_dream_analysis()`
3. Add environment variable parsing in `migrate-dream-analysis.rs`
4. Update documentation

### Error Handling

The script continues processing even if individual dreams fail:

- Each dream is wrapped in a try/catch
- Errors are printed but don't stop the script
- Final summary shows success/failure counts
- Exit code 1 if any failures occurred

### Performance Considerations

- 1-second delay between requests to avoid rate limiting
- Single-threaded processing (could be parallelized for local models)
- Database writes are immediate (could batch for performance)
- No progress bar (could add with `indicatif` crate)

## Future Improvements

Potential enhancements:

1. **Parallel Processing**: Process multiple dreams concurrently (especially for Ollama)
2. **Progress Bar**: Visual progress indicator using `indicatif`
3. **Resume Support**: Save progress and resume from last successful dream
4. **Batch Mode**: Process in batches with configurable batch size
5. **Config File**: Support `.env` file or TOML config instead of env vars
6. **Verbose Mode**: `-v` flag for detailed logging
7. **Force Mode**: `--force` to regenerate existing analyses
8. **Filter Mode**: Only process dreams matching certain criteria (date range, etc.)

## Troubleshooting

### Compilation Errors

If the script fails to compile:

```bash
cd src-tauri
cargo clean
cargo build --bin migrate-dream-analysis
```

Common issues:
- Missing dependencies: Run `cargo update`
- Rust version: Requires Rust 1.70+ (check with `rustc --version`)

### Runtime Errors

**"Failed to open database"**
- Database doesn't exist yet (run the app once first)
- Permission issues (check file ownership)
- Wrong database path (check platform-specific location)

**"Failed to generate analysis"**
- LLM provider not running (Ollama)
- Network issues (cloud providers)
- Invalid API key
- Rate limiting

### Database Issues

To inspect the database:

```bash
# Linux
sqlite3 ~/.local/share/limnl/limnl-journal/dreams.db

# Check tables
.tables

# Check dream analyses
SELECT COUNT(*) FROM dream_analyses;

# See which dreams have analysis
SELECT d.id, d.title, da.id as analysis_id
FROM dreams d
LEFT JOIN dream_analyses da ON d.id = da.dream_id;
```

## Alternative Approaches Considered

### 1. TypeScript Script with Tauri

**Pros**: Same language as frontend, familiar tooling
**Cons**: Requires Tauri runtime, complex setup, no direct DB access

### 2. Web-Based Migration Page

**Pros**: User-friendly GUI, no CLI knowledge needed
**Cons**: Not dev-only, adds maintenance burden, requires app to be running

### 3. SQL Script

**Pros**: Simplest possible approach
**Cons**: Can't call LLM APIs, would require pre-generated analyses

### 4. Node.js Script with DB Driver

**Pros**: JavaScript, can use npm packages
**Cons**: Need to reimplement LLM client logic, separate dependency management

**Chosen Approach**: Rust binary (option 1) because it reuses existing code, requires minimal setup, and is truly dev-only.
