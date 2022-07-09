use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .cargo_out_dir("proto")
        .input("src/protos/example.proto")
        .include("src/protos")
        .run_from_script();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/protos/");
}
