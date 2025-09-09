use clap::Parser;
use lib::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aibom-generator")]
#[command(about = "Generate AI Bill of Materials (AIBOM) for machine learning models")]
#[command(version = "1.0.0")]
struct Args {
    /// The model ID to analyze (e.g., microsoft/DialoGPT-medium)
    #[arg(value_name = "MODEL_ID")]
    model_id: String,

    /// Output file path for the generated AIBOM
    #[arg(short, long, value_name = "FILE", default_value = "aibom.json")]
    output: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    if args.verbose {
        println!("Generating AIBOM for model: {}", args.model_id);
        println!("Output file: {}", args.output.display());
    }

    let mut generator = AIBOMGenerator::new()?;
    let aibom = generator.generate_aibom(&args.model_id)?;

    let output_content = serde_json::to_string_pretty(&aibom)?;

    if args.verbose {
        println!("Generated AIBOM:");
        println!("{}", output_content);
    }

    std::fs::write(&args.output, &output_content)?;

    println!("AIBOM saved to {}", args.output.display());

    Ok(())
}
