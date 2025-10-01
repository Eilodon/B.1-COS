#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unpredictable_function_pointer_comparisons)]

fn hello(name: String) -> String {
    format!("Hello from Rust, {}!", name)
}

uniffi::include_scaffolding!("pandora");