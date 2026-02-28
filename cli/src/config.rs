use anyhow::Result;
use serde::{Deserialize, Serialize};


/// Configuration for QB-COM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub compiler: CompilerConfig,
    pub runtime: RuntimeConfig,
    pub display: DisplayConfig,
    pub sound: SoundConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub optimization_level: u8,
    pub target: String,
    pub emit_llvm_ir: bool,
    pub emit_bytecode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub memory_limit_mb: usize,
    pub stack_limit: usize,
    pub enable_graphics: bool,
    pub enable_sound: bool,
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub screen_mode: u8,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub vsync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    pub enabled: bool,
    pub sample_rate: u32,
    pub buffer_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            compiler: CompilerConfig {
                optimization_level: 2,
                target: "native".to_string(),
                emit_llvm_ir: false,
                emit_bytecode: false,
            },
            runtime: RuntimeConfig {
                memory_limit_mb: 16, // 16MB like old DOS
                stack_limit: 1024,
                enable_graphics: true,
                enable_sound: true,
                strict_mode: false,
            },
            display: DisplayConfig {
                screen_mode: 0,
                width: 640,
                height: 480,
                scale: 2.0,
                vsync: true,
            },
            sound: SoundConfig {
                enabled: true,
                sample_rate: 44100,
                buffer_size: 512,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from config file
        if let Some(config_dir) = directories::ProjectDirs::from("com", "qbc", "QB-COM") {
            let config_path = config_dir.config_dir().join("config.toml");
            if config_path.exists() {
                let content = std::fs::read_to_string(&config_path)?;
                let config: Config = toml::from_str(&content)?;
                return Ok(config);
            }
        }
        Ok(Self::default())
    }

    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "qbc", "QB-COM") {
            std::fs::create_dir_all(config_dir.config_dir())?;
            let config_path = config_dir.config_dir().join("config.toml");
            let content = toml::to_string_pretty(self)?;
            std::fs::write(&config_path, content)?;
        }
        Ok(())
    }
}
