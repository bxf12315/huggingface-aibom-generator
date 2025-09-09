use lib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = AIBOMGenerator::new()?;
    let model_id = "microsoft/DialoGPT-medium";

    println!("Generating AIBOM for model: {}", model_id);

    let aibom = generator.generate_aibom(model_id)?;
    let json_output = serde_json::to_string_pretty(&aibom)?;
    println!("{}", json_output);

    std::fs::write("aibom.json", &json_output)?;
    println!("AIBOM saved to aibom.json");

    Ok(())
}


