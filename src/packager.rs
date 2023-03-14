use bincode::{config::standard, encode_into_std_write};
use clap::Parser;
use responder::{Matcher, Package};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use yaml_rust::{Yaml, YamlLoader};

/// Generate a package usable by Responder Server
#[derive(Parser)]
struct Args {
    /// Path to Responder manifest
    manifest: String,

    /// Path to output file (for example package.bin)
    output: String,
}

fn main() {
    // Initialize package
    let mut package = Package {
        matchers: Vec::new(),
        responses: Vec::new(),
    };

    // Parse command line arguments.
    let args = Args::parse();

    // Read the manifest.
    let manifest_string = {
        let mut file = File::open(args.manifest.clone()).expect(&format!(
            "Could not open the manifest file from {}",
            args.manifest,
        ));
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("Could not read the manifest");
        buffer
    };

    // Base path relative to manifest to resolve response paths from.
    let base_path = args.manifest.clone();
    let base_path = Path::new(&base_path)
        .parent()
        .expect("Could not resolve folder containing manifest file");

    // Parse the manifest.
    let documents =
        YamlLoader::load_from_str(&manifest_string).expect("Could not load the manifest");
    let document = documents
        .get(0)
        .expect("Manifest must be the first document");
    let manifest_hash = document.as_hash().expect("The manifest must be an object");

    // Parse matchers.
    let matchers_yaml = manifest_hash
        .get(&Yaml::from_str("matchers"))
        .expect("The manifest must have 'matchers' property")
        .as_vec()
        .expect("Property 'matchers' must be an array");
    for matcher_yaml in matchers_yaml {
        let matcher_hash = matcher_yaml.as_hash().expect("Matcher must be an object");

        // Parse headers.
        let headers = {
            let mut headers: HashMap<String, Vec<String>> = HashMap::new();
            let headers_yaml = matcher_hash.get(&Yaml::from_str("headers"));
            if headers_yaml.is_some() {
                let headers_yaml = headers_yaml
                    .unwrap()
                    .as_hash()
                    .expect("Headers must be an object");
                for (key_yaml, values_yaml) in headers_yaml {
                    let key = key_yaml
                        .as_str()
                        .expect("Header property key must be a string")
                        .to_string();
                    let mut values: Vec<String> = Vec::new();
                    let values_yaml = values_yaml
                        .as_vec()
                        .expect("Header property value must be an array of possible values");
                    for value_yaml in values_yaml {
                        let value = value_yaml
                            .as_str()
                            .expect("Header value must be a string")
                            .to_string();
                        values.push(value)
                    }
                    headers.insert(key, values);
                }
            }
            headers
        };

        // Parse methods.
        let methods = {
            let mut methods: Vec<String> = Vec::new();
            let methods_yaml = matcher_hash.get(&Yaml::from_str("methods"));
            if methods_yaml.is_some() {
                let methods_yaml = methods_yaml
                    .unwrap()
                    .as_vec()
                    .expect("Methods must be an array");
                for method in methods_yaml {
                    let method = method
                        .as_str()
                        .expect("Method must be a string")
                        .to_string();
                    methods.push(method);
                }
            }
            methods
        };

        // Parse paths.
        let paths = {
            let mut paths: Vec<String> = Vec::new();
            let paths_yaml = matcher_hash.get(&Yaml::from_str("paths"));
            if paths_yaml.is_some() {
                let paths_yaml = paths_yaml
                    .unwrap()
                    .as_vec()
                    .expect("Paths must be an array");
                for path in paths_yaml {
                    let path = path.as_str().expect("Path must be a string").to_string();
                    paths.push(path);
                }
            }
            paths
        };

        // Add matcher to the package.
        package.matchers.push(Matcher {
            headers,
            methods,
            paths,
            response: package.responses.len() as u32,
        });

        // Parse response.
        let response_path = matcher_hash
            .get(&Yaml::from_str("response"))
            .expect("Matcher must have a response")
            .as_str()
            .expect("Response must be a string")
            .to_string();

        // Read response file.
        let mut response: Vec<u8> = Vec::new();
        let response_file_path = base_path.join(response_path);
        let mut response_file = File::open(&response_file_path).expect(&format!(
            "Count not open response file from {}",
            response_file_path.to_str().unwrap()
        ));
        response_file
            .read_to_end(&mut response)
            .expect("Could not read response file");
        package.responses.push(response);
    }

    let mut output_file = File::create(args.output).expect("Could not create output file");
    encode_into_std_write(package, &mut output_file, standard())
        .expect("Could not encode package into output file");
}
