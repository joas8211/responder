use bincode::{config::standard, encode_into_std_write};
use clap::Parser;
use responder::Package;
use std::{fs::File, process::exit};

/// Generate a package usable by Responder Server
#[derive(Parser)]
struct Args {
    /// Path to Responder manifest
    manifest: String,

    /// Path to output file (for example package.bin)
    output: String,
}

fn main() {
    // Parse command line arguments.
    let args = Args::parse();
    let package: Package;

    if args.manifest.ends_with(".yaml") {
        package = Package::from_manifest_yaml(args.manifest)
            .expect("Could not make a package from YAML manifest");
    } else if args.manifest.ends_with(".json") {
        package = Package::from_manifest_yaml(args.manifest)
            .expect("Could not make a package from JSON manifest");
    } else {
        eprintln!("Manifest must be either YAML and JSON file.");
        exit(1);
    }

    let mut output_file = File::create(args.output).expect("Could not create output file");
    encode_into_std_write(package, &mut output_file, standard())
        .expect("Could not encode package into output file");
}
