/**
 * Dream Analysis Migration Script
 *
 * This script generates dream analysis for all dreams that don't have one yet.
 * It uses the configured LLM provider from settings to analyze each dream.
 *
 * Usage:
 *   pnpm tsx scripts/migrate-dream-analysis.ts [--dry-run] [--limit N]
 *
 * Options:
 *   --dry-run    Show which dreams would be analyzed without actually running analysis
 *   --limit N    Only process N dreams (useful for testing)
 *   --help       Show this help message
 */

import { invoke } from '@tauri-apps/api/core';

interface Dream {
  id: number;
  title: string;
  content: string;
  date_occurred: string;
  sleep_quality: number | null;
}

interface DreamAnalysisWithCards {
  analysis: {
    id: number;
    dream_id: number;
    themes_patterns: string;
    emotional_analysis: string;
    narrative_summary: string;
    created_at: string;
    updated_at: string;
  };
  cards: Array<{
    card_id: number;
    card_name: string;
    relevance_note: string | null;
  }>;
}

interface LLMConfig {
  provider: 'ollama' | 'openai' | 'anthropic' | 'disabled';
  ollama_url?: string;
  ollama_model?: string;
  openai_api_key?: string;
  openai_model?: string;
  anthropic_api_key?: string;
  anthropic_model?: string;
}

async function getLLMConfig(): Promise<LLMConfig | null> {
  if (typeof window === 'undefined') {
    console.error('This script must run in a Tauri environment');
    return null;
  }

  const configStr = localStorage.getItem('limnl-llm-config');
  if (!configStr) {
    console.error('No LLM configuration found in localStorage');
    console.error('Please configure LLM settings in the app before running this script');
    return null;
  }

  try {
    const config = JSON.parse(configStr) as LLMConfig;

    if (config.provider === 'disabled') {
      console.error('LLM is disabled. Please enable an LLM provider in settings');
      return null;
    }

    return config;
  } catch (e) {
    console.error('Failed to parse LLM config:', e);
    return null;
  }
}

async function getAllDreams(): Promise<Dream[]> {
  try {
    return await invoke<Dream[]>('list_dreams');
  } catch (e) {
    console.error('Failed to fetch dreams:', e);
    throw e;
  }
}

async function getDreamAnalysis(dreamId: number): Promise<DreamAnalysisWithCards | null> {
  try {
    return await invoke<DreamAnalysisWithCards>('get_dream_analysis_with_cards', { dreamId });
  } catch (e) {
    // If analysis doesn't exist, command returns an error
    return null;
  }
}

async function generateDreamAnalysis(dreamId: number, llmConfig: LLMConfig): Promise<void> {
  try {
    await invoke('generate_dream_analysis', {
      request: {
        dream_id: dreamId,
        llm_config: llmConfig
      }
    });
  } catch (e) {
    console.error(`Failed to generate analysis for dream ${dreamId}:`, e);
    throw e;
  }
}

function parseArgs() {
  const args = process.argv.slice(2);
  const options = {
    dryRun: false,
    limit: null as number | null,
    help: false
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === '--dry-run') {
      options.dryRun = true;
    } else if (arg === '--limit') {
      const nextArg = args[i + 1];
      if (!nextArg || isNaN(parseInt(nextArg))) {
        console.error('--limit requires a number argument');
        process.exit(1);
      }
      options.limit = parseInt(nextArg);
      i++; // Skip next arg
    } else if (arg === '--help' || arg === '-h') {
      options.help = true;
    } else {
      console.error(`Unknown option: ${arg}`);
      options.help = true;
    }
  }

  return options;
}

function showHelp() {
  console.log(`
Dream Analysis Migration Script

This script generates dream analysis for all dreams that don't have one yet.
It uses the configured LLM provider from settings to analyze each dream.

Usage:
  pnpm tsx scripts/migrate-dream-analysis.ts [--dry-run] [--limit N]

Options:
  --dry-run    Show which dreams would be analyzed without actually running analysis
  --limit N    Only process N dreams (useful for testing)
  --help       Show this help message

Examples:
  # Dry run to see what would be processed
  pnpm tsx scripts/migrate-dream-analysis.ts --dry-run

  # Process only the first 5 dreams (for testing)
  pnpm tsx scripts/migrate-dream-analysis.ts --limit 5

  # Process all dreams without analysis
  pnpm tsx scripts/migrate-dream-analysis.ts

Notes:
  - LLM must be configured in app settings before running this script
  - The script will skip dreams that already have analysis
  - Each dream analysis takes 5-15 seconds depending on your LLM provider
  - Progress is shown as dreams are processed
`);
}

async function main() {
  const options = parseArgs();

  if (options.help) {
    showHelp();
    process.exit(0);
  }

  console.log('Dream Analysis Migration Script');
  console.log('================================\n');

  // Check LLM configuration
  console.log('Checking LLM configuration...');
  const llmConfig = await getLLMConfig();
  if (!llmConfig) {
    process.exit(1);
  }
  console.log(`LLM Provider: ${llmConfig.provider}\n`);

  // Fetch all dreams
  console.log('Fetching all dreams...');
  const dreams = await getAllDreams();
  console.log(`Found ${dreams.length} total dreams\n`);

  // Find dreams without analysis
  console.log('Checking for existing analyses...');
  const dreamsNeedingAnalysis: Dream[] = [];

  for (const dream of dreams) {
    const analysis = await getDreamAnalysis(dream.id);
    if (!analysis) {
      dreamsNeedingAnalysis.push(dream);
    }
  }

  console.log(`Found ${dreamsNeedingAnalysis.length} dreams without analysis\n`);

  if (dreamsNeedingAnalysis.length === 0) {
    console.log('All dreams already have analysis! Nothing to do.');
    process.exit(0);
  }

  // Apply limit if specified
  const dreamsToProcess = options.limit
    ? dreamsNeedingAnalysis.slice(0, options.limit)
    : dreamsNeedingAnalysis;

  if (options.limit && dreamsToProcess.length < dreamsNeedingAnalysis.length) {
    console.log(`Processing only ${dreamsToProcess.length} dreams due to --limit flag\n`);
  }

  // Show what will be processed
  console.log('Dreams to be analyzed:');
  console.log('----------------------');
  for (const dream of dreamsToProcess) {
    const preview = dream.content.substring(0, 60).replace(/\n/g, ' ');
    console.log(`  #${dream.id}: "${dream.title}" (${dream.date_occurred})`);
    console.log(`     ${preview}${dream.content.length > 60 ? '...' : ''}`);
  }
  console.log('');

  // Dry run - exit here
  if (options.dryRun) {
    console.log('[DRY RUN] Would analyze these dreams, but not actually doing it.');
    console.log('Remove --dry-run flag to perform actual analysis.');
    process.exit(0);
  }

  // Confirm before proceeding
  console.log(`About to generate analysis for ${dreamsToProcess.length} dreams.`);
  console.log('This may take several minutes depending on your LLM provider.');
  console.log('Press Ctrl+C to cancel, or any key to continue...\n');

  // Wait for user input (if in interactive terminal)
  if (process.stdin.isTTY) {
    process.stdin.setRawMode(true);
    await new Promise(resolve => {
      process.stdin.once('data', () => {
        process.stdin.setRawMode(false);
        resolve(null);
      });
    });
  }

  // Process dreams
  console.log('Starting analysis generation...\n');
  let successCount = 0;
  let errorCount = 0;

  for (let i = 0; i < dreamsToProcess.length; i++) {
    const dream = dreamsToProcess[i];
    const progress = `[${i + 1}/${dreamsToProcess.length}]`;

    console.log(`${progress} Analyzing dream #${dream.id}: "${dream.title}"`);

    try {
      await generateDreamAnalysis(dream.id, llmConfig);
      successCount++;
      console.log(`${progress} ✓ Success\n`);
    } catch (e) {
      errorCount++;
      console.error(`${progress} ✗ Failed: ${e}\n`);
    }

    // Small delay between requests to avoid rate limiting
    if (i < dreamsToProcess.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }

  // Summary
  console.log('\n================================');
  console.log('Migration Complete');
  console.log('================================');
  console.log(`Total dreams processed: ${dreamsToProcess.length}`);
  console.log(`Successful: ${successCount}`);
  console.log(`Failed: ${errorCount}`);

  if (errorCount > 0) {
    console.log('\nSome dreams failed to analyze. Check errors above.');
    process.exit(1);
  } else {
    console.log('\nAll dreams analyzed successfully!');
    process.exit(0);
  }
}

// Run the script
main().catch(e => {
  console.error('Fatal error:', e);
  process.exit(1);
});
