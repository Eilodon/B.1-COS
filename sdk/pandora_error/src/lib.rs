use thiserror::Error;

#[derive(Error, Debug)]
pub enum PandoraError {
    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Skill Execution Error: {skill_name} - {message}")]
    SkillExecution { skill_name: String, message: String },

    #[error("FFI Interface Error: {0}")]
    Ffi(String),

    #[error("Unknown error")]
    Unknown,
}

