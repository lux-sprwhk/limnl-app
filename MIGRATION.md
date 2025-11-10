# Dream Analysis Migration Guide

## Overview

This guide explains how to generate dream analysis for dreams that were created before the dream analysis feature was added using the Rust migration script.

## Quick Start

```bash
# 1. Set environment variables for your LLM provider
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2

# 2. Run the migration script
cd src-tauri
cargo run --bin migrate-dream-analysis

# Or with options
cargo run --bin migrate-dream-analysis -- --dry-run     # Preview only
cargo run --bin migrate-dream-analysis -- --limit 5     # Test with 5 dreams
```

## Prerequisites

### LLM Configuration

Before running the migration, you must configure an LLM provider via environment variables:

#### Option 1: Ollama (Local, Free, Recommended)

```bash
export LLM_PROVIDER=ollama
export OLLAMA_URL=http://localhost:11434  # Optional, this is the default
export OLLAMA_MODEL=llama3.2              # Or mistral, phi3, etc.
```

**Prerequisites**: Install Ollama and pull a model:
```bash
# Install Ollama from https://ollama.ai
ollama pull llama3.2
```

#### Option 2: OpenAI

```bash
export LLM_PROVIDER=openai
export OPENAI_API_KEY=sk-...           # Your API key
export OPENAI_MODEL=gpt-4o-mini       # Optional, this is the default
```

**Cost**: ~$0.0002 per dream (~$0.02 for 100 dreams)

#### Option 3: Anthropic Claude

```bash
export LLM_PROVIDER=anthropic
export ANTHROPIC_API_KEY=sk-ant-...                    # Your API key
export ANTHROPIC_MODEL=claude-3-5-haiku-20241022      # Optional, this is the default
```

**Cost**: ~$0.0005 per dream (~$0.05 for 100 dreams)

## Script Options

```bash
cargo run --bin migrate-dream-analysis -- [OPTIONS]
```

### Available Options

- `--dry-run`: Preview which dreams would be analyzed without actually running analysis
- `--limit N`: Only process N dreams (useful for testing)
- `--help`: Show help message with all options

### Examples

```bash
# Preview what would be migrated
cargo run --bin migrate-dream-analysis -- --dry-run

# Test with first 5 dreams
cargo run --bin migrate-dream-analysis -- --limit 5

# Process all dreams (requires confirmation)
cargo run --bin migrate-dream-analysis
```

## Migration Process

When you run the script, it will:

1. **Validate Configuration**: Check that LLM provider environment variables are set
2. **Connect to Database**: Open the SQLite database at the platform-specific location
3. **Fetch Dreams**: Get all dreams from the database
4. **Identify Missing Analyses**: Check which dreams don't have analysis yet
5. **Show Preview**: List all dreams that will be processed
6. **Confirm**: Wait for you to press Enter (unless `--dry-run`)
7. **Process Each Dream**:
   - Send dream content to LLM
   - Receive analysis with themes, emotions, narrative, and symbol cards
   - Save analysis to `dream_analyses` table
   - Link symbol cards in `dream_analysis_cards` table
   - Show progress with success/failure status
8. **Report Summary**: Display final statistics (successful, failed, total time)

### Output Example

```
Dream Analysis Migration Script
================================

LLM Provider: ollama
Ollama URL: http://localhost:11434
Ollama Model: llama3.2

Database: /home/user/.local/share/limnl/limnl-journal/dreams.db

Fetching all dreams...
Found 42 total dreams

Checking for existing analyses...
Found 15 dreams without analysis

Dreams to be analyzed:
----------------------
  #8: "Flying Over Mountains" (2025-01-10)
     I was soaring above snow-capped peaks, feeling free and lightw...
  #12: "Lost in a Library" (2025-01-15)
     Endless shelves of books, searching for something I couldn't r...
  ...

About to generate analysis for 15 dreams.
This may take several minutes depending on your LLM provider.
Press Enter to continue, or Ctrl+C to cancel...

Starting analysis generation...

[1/15] Analyzing dream #8: "Flying Over Mountains"
[1/15] ✓ Success

[2/15] Analyzing dream #12: "Lost in a Library"
[2/15] ✓ Success

...

================================
Migration Complete
================================
Total dreams processed: 15
Successful: 15
Failed: 0

All dreams analyzed successfully!
```

## Best Practices

### Testing First

Always test with a small batch before running the full migration:

```bash
# 1. Dry run to see what would happen
cargo run --bin migrate-dream-analysis -- --dry-run

# 2. Process 5 test dreams
cargo run --bin migrate-dream-analysis -- --limit 5

# 3. Check the results in the app

# 4. Run full migration if satisfied
cargo run --bin migrate-dream-analysis
```

### Handling Large Migrations

For 100+ dreams:

- **Use Ollama** for faster, free, local processing
- The script includes 1-second delays between requests to avoid rate limiting
- Time estimates:
  - Ollama: 5-15 minutes for 100 dreams (depends on hardware)
  - OpenAI/Anthropic: 3-10 minutes for 100 dreams

### Re-Running the Script

It's **safe to re-run** the migration multiple times:

- Only processes dreams without analysis
- Dreams with existing analysis are automatically skipped
- No duplicate analyses will be created

To regenerate analysis for a specific dream:
1. Delete the existing analysis from the app (dream detail page)
2. Re-run the migration script

## Time Estimates

Per dream:
- **Ollama**: 3-10 seconds (depends on your hardware and model)
- **OpenAI**: 2-5 seconds
- **Anthropic**: 2-5 seconds

Total time:
- **10 dreams**: ~1 minute
- **50 dreams**: ~5 minutes
- **100 dreams**: ~10 minutes
- **500 dreams**: ~50 minutes

## Cost Estimates (Cloud Providers)

**OpenAI** (gpt-4o-mini):
- ~$0.0002 per dream
- 100 dreams ≈ $0.02
- 1000 dreams ≈ $0.20

**Anthropic Claude** (claude-3-5-haiku-20241022):
- ~$0.0005 per dream
- 100 dreams ≈ $0.05
- 1000 dreams ≈ $0.50

**Ollama**: Free (runs locally on your computer)

## Troubleshooting

### "LLM_PROVIDER environment variable not set"

**Solution**: Set the environment variable before running:
```bash
export LLM_PROVIDER=ollama  # or openai, anthropic
cargo run --bin migrate-dream-analysis
```

### "OPENAI_API_KEY not set" or "ANTHROPIC_API_KEY not set"

**Solution**: Set your API key:
```bash
export OPENAI_API_KEY=sk-...
# or
export ANTHROPIC_API_KEY=sk-ant-...
```

### "Failed to connect" (Ollama)

**Possible causes**:
- Ollama is not running
- Wrong URL

**Solutions**:
```bash
# Start Ollama (it should be running)
ollama serve

# Or check if it's already running
curl http://localhost:11434/api/version

# If using a different port, set OLLAMA_URL
export OLLAMA_URL=http://localhost:11434
```

### "Failed to generate analysis"

**Possible causes**:
- Network issues (cloud providers)
- Rate limiting
- Invalid API key
- Model not available

**Solutions**:
- Check your network connection
- Verify API key is correct
- Wait a few minutes and retry
- Check that model exists (for Ollama: `ollama list`)

### Script Fails with Rust Error

**Solution**: Make sure you're in the correct directory:
```bash
cd src-tauri
cargo run --bin migrate-dream-analysis
```

If you get compilation errors, try:
```bash
cargo clean
cargo build
cargo run --bin migrate-dream-analysis
```

### Some Dreams Failed

The script continues even if some dreams fail. Check the error messages to identify the issue:

- Note failed dream IDs from the output
- Check those dreams manually (very long content, special characters, etc.)
- Re-run the migration (will only process dreams still missing analysis)
- Or generate analysis manually from dream detail pages in the app

## What Gets Analyzed?

For each dream, the analysis includes:

1. **Themes & Patterns**: Recurring symbols, motifs, and themes
2. **Emotional Analysis**: Emotional landscape and feelings
3. **Narrative Summary**: Overview of the dream's story and progression
4. **Symbol Cards**: 2-3 cards from the deck that relate to the dream's symbols

## Technical Details

### Database Location

The script accesses the database at platform-specific locations:

- **Linux**: `~/.local/share/limnl/limnl-journal/dreams.db`
- **macOS**: `~/Library/Application Support/com.limnl.limnl-journal/dreams.db`
- **Windows**: `%APPDATA%\limnl\limnl-journal\dreams.db`

### Database Operations

The script performs:

1. `INSERT INTO dream_analyses` - Creates analysis record
2. `INSERT INTO dream_analysis_cards` - Links cards to analysis
3. Uses SQLite transactions for data integrity

### Rate Limiting

The script includes a 1-second delay between each dream to avoid:
- Cloud API rate limits
- Overwhelming local LLM servers
- Network congestion

## Advanced Usage

### Custom Environment Variables

You can create a `.env` file or shell script with your configuration:

```bash
# llm-config.sh
export LLM_PROVIDER=ollama
export OLLAMA_MODEL=llama3.2
```

Then source it before running:
```bash
source llm-config.sh
cd src-tauri
cargo run --bin migrate-dream-analysis
```

### Integrating into Development Workflow

Add to your dev scripts:

```bash
# In your shell profile or Makefile
alias migrate-dreams="cd src-tauri && cargo run --bin migrate-dream-analysis"
```

Then just run:
```bash
migrate-dreams -- --dry-run
migrate-dreams -- --limit 10
```

## Support

If you encounter issues:

1. Check this documentation
2. Run with `--dry-run` to preview
3. Test with `--limit 5` first
4. Check the script output for specific error messages
5. Verify your LLM configuration with a test request
6. Check `scripts/README.md` for additional troubleshooting
