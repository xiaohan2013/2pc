// extern crate protoc_rust;

// fn main() {
//     // let out_dir_env = env.var_os("OUT_DIR");
//     protobuf_codegen_pure::Codegen::new()
//         .out_dir("src/protos")
//         .inputs(&["src/protos/person.proto"])
//         .include("src/protos")
//         .run()
//         .expect("Codegen failed.");

//     // protobuf_codegen::Codegen::new()
//     //     .protoc()
//     //     // .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
//     //     .cargo_out_dir("protos")
//     //     .input("src/protos/person.proto")
//     //     .include("src/protos")
//     //     .run_from_script();

//     // protoc_rust
//     // protoc_rust::Codegen::new()
//     //     .out_dir("src/protos")
//     //     .inputs(&["src/protos/person.proto"])
//     //     .include("src/protos")
//     //     .run()
//     //     .expect("Running protoc failed.");
// }


// fn main()->Result<(),Box<dyn std::error::Error>>{
//     // compiling protos using path on build time
//     tonic_build::compile_protos("src/protos/message.proto")?;
//     Ok(())
// }

fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/protos")
        .compile(&["src/protos/message.proto"], &["src/protos"])
        .unwrap()
}