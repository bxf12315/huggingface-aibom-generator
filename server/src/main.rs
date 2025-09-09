use actix_web::{App, HttpResponse, HttpServer, Result, middleware::Logger, web};
use lib::AIBOMGenerator;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GenerateRequest {
    model_id: String,
    #[serde(default)]
    verbose: bool,
}

#[derive(Serialize)]
struct GenerateResponse {
    success: bool,
    aibom: Option<serde_json::Value>,
    error: Option<String>,
}

async fn generate_aibom(req: web::Json<GenerateRequest>) -> Result<HttpResponse> {
    let model_id = req.model_id.clone();
    let verbose = req.verbose;

    // Validate model_id format
    if model_id.is_empty() {
        return Ok(HttpResponse::BadRequest().json(GenerateResponse {
            success: false,
            aibom: None,
            error: Some("model_id cannot be empty".to_string()),
        }));
    }

    println!(
        "📥 Received AIBOM generation request: model_id={}, verbose={}",
        model_id, verbose
    );

    if verbose {
        println!("🚀 Generating AIBOM for model '{}'...", model_id);
    }

    // Clone model_id for closure
    let model_id_for_closure = model_id.clone();

    // Use web::block to run synchronous code in thread pool
    let result = web::block(move || -> Result<lib::AIBOM, String> {
        let mut generator = AIBOMGenerator::new().map_err(|e| e.to_string())?;
        generator
            .generate_aibom(&model_id_for_closure)
            .map_err(|e| e.to_string())
    })
    .await;

    match result {
        Ok(Ok(aibom)) => {
            let aibom_json = serde_json::to_value(&aibom).unwrap();

            if verbose {
                println!("✅ AIBOM generation successful");
                println!("📊 Component count: {}", aibom.components.len());
                println!("🔗 Dependencies: {}", aibom.dependencies.len());
            } else {
                println!("✅ AIBOM generation successful: {}", model_id);
            }

            Ok(HttpResponse::Ok().json(GenerateResponse {
                success: true,
                aibom: Some(aibom_json),
                error: None,
            }))
        }
        Ok(Err(e)) => {
            eprintln!("❌ AIBOM generation failed: {}", e);
            Ok(HttpResponse::InternalServerError().json(GenerateResponse {
                success: false,
                aibom: None,
                error: Some(format!("Error generating AIBOM: {}", e)),
            }))
        }
        Err(e) => {
            eprintln!("❌ Thread pool execution failed: {}", e);
            Ok(HttpResponse::InternalServerError().json(GenerateResponse {
                success: false,
                aibom: None,
                error: Some(format!("Error executing task: {}", e)),
            }))
        }
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "aibom-generator-server"
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Starting AIBOM Generator Server on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/generate", web::post().to(generate_aibom))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
