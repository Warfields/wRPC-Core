extern crate protoc_rust;
use protoc_rust::Customize;

fn main() {

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["protos/RPC_Module.proto", "protos/result.proto", 
            "protos/packager.proto", "protos/request.proto"],
        includes: &["protos"],
        customize: Customize {
        ..Default::default()
        },
    }).expect("protoc");
                
}