fn main() {
    prost_build::compile_protos(&["proto/ontology.proto", "proto/core.proto"], &["proto/"])
        .unwrap();
}
