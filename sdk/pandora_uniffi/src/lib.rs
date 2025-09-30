#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unpredictable_function_pointer_comparisons)]
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

fn hello(name: String) -> String {
    format!("Hello from Rust, {}!", name)
}

uniffi::include_scaffolding!("pandora");
