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

    #[error("Timeout while executing skill {skill_name} after {timeout_ms}ms")]
    Timeout { skill_name: String, timeout_ms: u64 },

    #[error("Circuit breaker open for skill {skill_name}")]
    CircuitOpen { skill_name: String },
}
