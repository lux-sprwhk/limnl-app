/// Dream Analysis Migration Script
///
/// This script generates dream analysis for all dreams that don't have one yet.
/// It reads LLM configuration from environment variables.
///
/// Usage:
///   # Set environment variables for your LLM provider
///   export LLM_PROVIDER=ollama  # or openai, anthropic
///   export OLLAMA_URL=http://localhost:11434
///   export OLLAMA_MODEL=llama3.2
///
///   # Run the script
///   cargo run --bin migrate-dream-analysis
///
///   # Or with options
///   cargo run --bin migrate-dream-analysis -- --limit 5
///   cargo run --bin migrate-dream-analysis -- --dry-run

use std::env;
use lmnl_app_lib::db::{Database, models::CreateDreamAnalysisInput};
use lmnl_app_lib::llm::{LLMConfig, LLMProvider};

fn print_usage() {
    println!(
        r#"
Dream Analysis Migration Script

This script generates dream analysis for all dreams that don't have one yet.

Usage:
  cargo run --bin migrate-dream-analysis [OPTIONS]

Options:
  --dry-run         Show which dreams would be analyzed without actually running
  --limit N         Only process N dreams (useful for testing)
  --help            Show this help message

Environment Variables (LLM Configuration):
  LLM_PROVIDER            Provider to use: ollama, openai, or anthropic

  For Ollama:
    OLLAMA_URL           Ollama API URL (default: http://localhost:11434)
    OLLAMA_MODEL         Model name (default: llama3.2)

  For OpenAI:
    OPENAI_API_KEY       Your OpenAI API key
    OPENAI_MODEL         Model name (default: gpt-4o-mini)

  For Anthropic:
    ANTHROPIC_API_KEY    Your Anthropic API key
    ANTHROPIC_MODEL      Model name (default: claude-3-5-haiku-20241022)

Examples:
  # Dry run with Ollama
  export LLM_PROVIDER=ollama
  export OLLAMA_MODEL=llama3.2
  cargo run --bin migrate-dream-analysis -- --dry-run

  # Process first 5 dreams with OpenAI
  export LLM_PROVIDER=openai
  export OPENAI_API_KEY=sk-...
  cargo run --bin migrate-dream-analysis -- --limit 5

  # Process all dreams with Anthropic
  export LLM_PROVIDER=anthropic
  export ANTHROPIC_API_KEY=sk-ant-...
  cargo run --bin migrate-dream-analysis
"#
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut dry_run = false;
    let mut limit: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => dry_run = true,
            "--limit" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --limit requires a number argument");
                    std::process::exit(1);
                }
                limit = Some(args[i + 1].parse().expect("Invalid limit number"));
                i += 1;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    println!("Dream Analysis Migration Script");
    println!("================================\n");

    // Get LLM configuration from environment
    let provider_str = env::var("LLM_PROVIDER").unwrap_or_else(|_| {
        eprintln!("Error: LLM_PROVIDER environment variable not set");
        eprintln!("Please set it to: ollama, openai, or anthropic");
        eprintln!("\nExample:");
        eprintln!("  export LLM_PROVIDER=ollama");
        std::process::exit(1);
    });

    println!("LLM Provider: {}", provider_str);

    // Create LLMConfig struct
    let (_provider, config) = match provider_str.to_lowercase().as_str() {
        "ollama" => {
            let url = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
            let model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string());
            println!("Ollama URL: {}", url);
            println!("Ollama Model: {}", model);

            (LLMProvider::Ollama, LLMConfig {
                provider: LLMProvider::Ollama,
                ollama_url: url,
                ollama_model: model,
                openai_api_key: String::new(),
                openai_model: String::new(),
                anthropic_api_key: String::new(),
                anthropic_model: String::new(),
            })
        }
        "openai" => {
            let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
            let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
            println!("OpenAI Model: {}", model);

            (LLMProvider::OpenAI, LLMConfig {
                provider: LLMProvider::OpenAI,
                ollama_url: String::new(),
                ollama_model: String::new(),
                openai_api_key: api_key,
                openai_model: model,
                anthropic_api_key: String::new(),
                anthropic_model: String::new(),
            })
        }
        "anthropic" => {
            let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set");
            let model = env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-haiku-4-5".to_string());
            println!("Anthropic Model: {}", model);

            (LLMProvider::Anthropic, LLMConfig {
                provider: LLMProvider::Anthropic,
                ollama_url: String::new(),
                ollama_model: String::new(),
                openai_api_key: String::new(),
                openai_model: String::new(),
                anthropic_api_key: api_key,
                anthropic_model: model,
            })
        }
        _ => {
            eprintln!("Error: Invalid LLM_PROVIDER: {}", provider_str);
            eprintln!("Must be: ollama, openai, or anthropic");
            std::process::exit(1);
        }
    };
    println!();

    // Initialize database
    println!("Connecting to database...");
    let db = Database::new()?;
    println!("Database connected\n");

    // Get all dreams
    println!("Fetching all dreams...");
    let dreams = db.list_dreams(None, None)?;
    println!("Found {} total dreams\n", dreams.len());

    // Find dreams without analysis
    println!("Checking for existing analyses...");
    let mut dreams_needing_analysis = Vec::new();
    for dream in &dreams {
        if let Some(dream_id) = dream.id {
            match db.get_dream_analysis_with_cards(dream_id) {
                Ok(Some(_)) => {
                    // Has analysis, skip
                }
                Ok(None) | Err(_) => {
                    // No analysis or error, needs it
                    dreams_needing_analysis.push(dream.clone());
                }
            }
        }
    }

    println!(
        "Found {} dreams without analysis\n",
        dreams_needing_analysis.len()
    );

    if dreams_needing_analysis.is_empty() {
        println!("All dreams already have analysis! Nothing to do.");
        return Ok(());
    }

    // Apply limit if specified
    let dreams_to_process: Vec<_> = if let Some(lim) = limit {
        dreams_needing_analysis
            .into_iter()
            .take(lim)
            .collect()
    } else {
        dreams_needing_analysis
    };

    if let Some(lim) = limit {
        println!("Limiting to {} dreams\n", lim);
    }

    // Show what will be processed
    println!("Dreams to be analyzed:");
    println!("----------------------");
    for dream in &dreams_to_process {
        let preview: String = dream
            .content
            .chars()
            .take(60)
            .collect::<String>()
            .replace('\n', " ");
        println!(
            "  #{}: \"{}\" ({})",
            dream.id.unwrap_or(0),
            dream.title,
            dream.date_occurred.format("%Y-%m-%d")
        );
        println!("     {}{}", preview, if dream.content.len() > 60 { "..." } else { "" });
    }
    println!();

    // Dry run - exit here
    if dry_run {
        println!("[DRY RUN] Would analyze these dreams, but not actually doing it.");
        println!("Remove --dry-run flag to perform actual analysis.");
        return Ok(());
    }

    // Confirm before proceeding
    println!(
        "About to generate analysis for {} dreams.",
        dreams_to_process.len()
    );
    println!("This may take several minutes depending on your LLM provider.");
    println!("Press Enter to continue, or Ctrl+C to cancel...\n");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Process dreams
    println!("Starting analysis generation...\n");
    let mut success_count = 0;
    let mut error_count = 0;

    for (i, dream) in dreams_to_process.iter().enumerate() {
        let progress = format!("[{}/{}]", i + 1, dreams_to_process.len());
        let dream_id = dream.id.unwrap_or(0);

        println!("{} Analyzing dream #{}: \"{}\"", progress, dream_id, dream.title);

        match lmnl_app_lib::llm::client::generate_dream_analysis(
            &dream.title,
            &dream.content,
            dream.sleep_quality,
            &config,
        )
        .await
        {
            Ok(analysis_response) => {
                println!("  Saving analysis to database...");
                // Save to database using Database methods
                match db.create_dream_analysis(CreateDreamAnalysisInput {
                    dream_id,
                    themes_patterns: analysis_response.themes_patterns.clone(),
                    emotional_analysis: analysis_response.emotional_analysis.clone(),
                    narrative_summary: analysis_response.narrative_summary.clone(),
                }) {
                    Ok(analysis) => {
                        println!("  Analysis saved. Linking {} symbol cards...", analysis_response.symbol_cards.len());
                        // Link cards
                        for symbol in &analysis_response.symbol_cards {
                            println!("    Looking up card: {}", symbol.card_name);
                            if let Ok(Some(card)) = db.get_card_by_name(&symbol.card_name) {
                                println!("    Linking card {} to analysis", symbol.card_name);
                                let _ = db.link_card_to_dream_analysis(
                                    analysis.id.unwrap_or(0),
                                    card.id.unwrap_or(0),
                                    Some(symbol.relevance_note.clone()),
                                );
                            } else {
                                println!("    Card not found: {}", symbol.card_name);
                            }
                        }

                        success_count += 1;
                        println!("{} ✓ Success\n", progress);
                    }
                    Err(e) => {
                        error_count += 1;
                        eprintln!("{} ✗ Failed to save analysis: {}\n", progress, e);
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                eprintln!("{} ✗ Failed: {}\n", progress, e);
            }
        }

        // Small delay between requests to avoid rate limiting
        if i < dreams_to_process.len() - 1 {
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }

    // Summary
    println!("\n================================");
    println!("Migration Complete");
    println!("================================");
    println!("Total dreams processed: {}", dreams_to_process.len());
    println!("Successful: {}", success_count);
    println!("Failed: {}", error_count);

    if error_count > 0 {
        println!("\nSome dreams failed to analyze. Check errors above.");
        std::process::exit(1);
    } else {
        println!("\nAll dreams analyzed successfully!");
    }

    Ok(())
}
