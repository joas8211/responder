use bincode::{config::standard, decode_from_std_read};
use responder::Package;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::exit,
};

fn main() {
    let args = env::args();
    if args.len() != 2 {
        eprintln!("First and only argument must be path to Responder bundle.");
        exit(1);
    }

    let package_path = args.last().unwrap();
    let mut package_file = File::open(package_path).expect("Could not open bundle file");
    let package: Package =
        decode_from_std_read(&mut package_file, standard()).expect("Could not decode bundle");

    #[cfg(target_family = "wasm")]
    let listener = wasmedge_wasi_socket::TcpListener::bind("0.0.0.0:8080", false)
        .expect("Could not bind TCP listener");
    #[cfg(not(target_family = "wasm"))]
    let listener =
        std::net::TcpListener::bind("0.0.0.0:8080").expect("Could not bind TCP listener");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let buf_reader = BufReader::new(&stream);
        let request: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if request.len() == 0 {
            continue;
        }

        let (start_line, headers) = request.split_first().unwrap();
        let start_line_parts: Vec<&str> = start_line.split(' ').collect();
        if start_line_parts.len() != 3 {
            continue;
        }

        let method = start_line_parts[0].to_string();
        let path = start_line_parts[1].to_string();
        let protocol = start_line_parts[2];
        if protocol != "HTTP/1.1" {
            continue;
        }

        'matching: for matcher in &package.matchers {
            if !matcher.methods.is_empty() && !matcher.methods.contains(&method) {
                continue;
            }

            if !matcher.paths.is_empty() && !matcher.paths.contains(&path) {
                continue;
            }

            'headers: for (header, values) in &matcher.headers {
                for line in headers {
                    let parts: Vec<&str> = line.splitn(2, ": ").collect();
                    if parts[0] != header {
                        // Not this header.
                        continue;
                    }

                    if parts[1].split(",").any(|v| values.contains(&v.to_string())) {
                        // Header is matching, check other headers.
                        continue 'headers;
                    } else {
                        // Header found but does not match, move to next matcher.
                        continue 'matching;
                    }
                }

                // Header not found, move to next matcher.
                continue 'matching;
            }

            // Match found, send response.
            let response = &package.responses[matcher.response as usize];
            stream.write(&response).ok();
            break;
        }
    }
}
