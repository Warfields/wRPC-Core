extern crate protoc_rust;
use protoc_rust::Customize;
use glob::glob;

fn main() {
    // Compile Main RPC Protos
    let protos = glob("protos/*.proto").expect("No Proto Files Found");

    for entry in protos {
        match entry{
            Ok(file) => {
                protoc_rust::run(protoc_rust::Args {
                    out_dir: "src/protos",
                    input: &[file.into_os_string().into_string().unwrap().as_str()],
                    includes: &["protos"],
                    customize: Customize {
                    ..Default::default()
                    },
                }).expect("protoc");
                }

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }

    // Compile Packager Specific Protos
    let protos = glob("protos/packgers/*.proto").expect("No Proto Files Found");

    for entry in protos {
        match entry{
            Ok(file) => {
                protoc_rust::run(protoc_rust::Args {
                    out_dir: "src/protos",
                    input: &[file.into_os_string().into_string().unwrap().as_str()],
                    includes: &["protos"],
                    customize: Customize {
                    ..Default::default()
                    },
                }).expect("protoc");
                }
                
            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }
}