use crate::{Matcher, Package};
use std::io::Result;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

impl Package {
    pub fn from_manifest_json(manifest_path: String) -> Result<Package> {
        // Initialize package
        let mut package = Package {
            matchers: Vec::new(),
            responses: Vec::new(),
        };

        // Read the manifest.
        let manifest_string = {
            let mut file = File::open(manifest_path.clone()).expect(&format!(
                "Could not open the manifest file from {}",
                manifest_path,
            ));
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)
                .expect("Could not read the manifest");
            buffer
        };

        // Base path relative to manifest to resolve response paths from.
        let base_path = manifest_path;
        let base_path = Path::new(&base_path)
            .parent()
            .expect("Could not resolve folder containing manifest file");

        // Parse the manifest.
        let document = json::parse(&manifest_string).expect("Could not parse the manifest");

        // Parse matchers.
        for matcher in document["matchers"].members() {
            // Parse headers.
            let headers = {
                let mut headers: HashMap<String, Vec<String>> = HashMap::new();
                for (key, value) in matcher["headers"].entries() {
                    let mut values: Vec<String> = Vec::new();
                    for value in value.members() {
                        values.push(value.to_string())
                    }
                    headers.insert(String::from(key), values);
                }
                headers
            };

            // Parse methods.
            let methods = {
                let mut methods: Vec<String> = Vec::new();
                for method in matcher["methods"].members() {
                    methods.push(method.to_string());
                }
                methods
            };

            // Parse paths.
            let paths = {
                let mut paths: Vec<String> = Vec::new();
                for path in matcher["paths"].members() {
                    paths.push(path.to_string());
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
            let response_path = matcher["response"].to_string();

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

        Ok(package)
    }
}
