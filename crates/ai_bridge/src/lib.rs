use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::process::Command;

#[derive(Debug, Error)]
pub enum AiError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Script failed: {0}")]
    ScriptFailed(String),
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub image_path: String,
    pub output_dir: String,
    pub quality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub success: bool,
    pub mesh_path: Option<String>,
    pub error: Option<String>,
    pub cached: bool,
    pub device: Option<String>,
}

pub struct AiBridge {
    pub python_exe: PathBuf,
    pub script_path: PathBuf,
}

impl AiBridge {
    pub fn new(python_path: PathBuf, script_path: PathBuf) -> Self {
        Self { python_exe: python_path, script_path }
    }

    pub async fn generate(&self, req: GenerationRequest) -> Result<GenerationResponse, AiError> {
        tokio::fs::create_dir_all(&req.output_dir).await?;

        let output = Command::new(&self.python_exe)
            .arg(&self.script_path)
            .arg("--input")
            .arg(&req.image_path)
            .arg("--output-dir")
            .arg(&req.output_dir)
            .arg("--quality")
            .arg(&req.quality)
            .output()
            .await?;

        if !output.status.success() {
            return Err(AiError::ScriptFailed(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let mut resp: GenerationResponse = serde_json::from_str(&json_str)?;
        
        if !serde_json::to_value(&resp).ok().and_then(|v| v.get("cached").and_then(|c| c.as_bool())).unwrap_or(false) {
            resp.cached = false;
        }

        if !resp.success {
            return Err(AiError::GenerationFailed(resp.error.unwrap_or("Unknown".into())));
        }

        Ok(resp)
    }
}
