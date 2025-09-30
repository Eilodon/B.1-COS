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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
