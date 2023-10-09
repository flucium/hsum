use std::io::{stdin, stdout, Read, Write};

use clap::Parser;

use digest::Digest;

use sha2::{Sha256, Sha512, Sha512_256};

const NAME: &str = "HSum";

const VERSION: &str = "0.0.1";

const AUTHOR: &str = "flucium <flucium@flucium.net>";

const ABOUT: &str = "A simple cli tool get hash digest from stdin";

#[derive(Debug, Clone, Parser)]
#[clap(name = NAME, version = VERSION, author = AUTHOR, about = ABOUT)]
struct Args {
    #[clap(long("hash"), short('h'), alias("hash"), default_value("sha256"), 
    possible_values(&["sha256", "sha512","sha512_256"]))]
    hash_algorithm: String,

    #[clap(long("uppercase"), short('u'), alias("upper"))]
    upper_case: Option<bool>,
}

fn read() -> Vec<u8> {
    let mut buffer = Vec::new();

    if let Err(err) = stdin().read_to_end(&mut buffer) {
        panic!("Error: {}", err.to_string());
    }

    buffer
}

fn write(bytes: impl AsRef<[u8]>) {
    if let Err(err) = stdout().write_all(bytes.as_ref()) {
        panic!("Error: {}", err.to_string());
    }
}

fn encode_hex(bytes: impl AsRef<[u8]>, is_uppercase: bool) -> String {
    let bytes = bytes.as_ref();

    let mut buffer = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        buffer.push_str(&format!("{:02x}", byte));
    }

    if is_uppercase {
        buffer = buffer.to_uppercase();
    } else {
        buffer = buffer.to_lowercase();
    }

    buffer
}

fn hash_digest(algorithm: &str, bytes: &[u8]) -> Vec<u8> {
    let digest: Vec<u8> = match algorithm {
        "sha256" => Sha256::digest(&bytes).to_vec(),
        "sha512" => Sha512::digest(&bytes).to_vec(),
        "sha512_256" => Sha512_256::digest(&bytes).to_vec(),
        _ => {
            panic!("{}: invalid hash algorithm", algorithm);
        }
    };

    digest
}

fn main() {
    let bytes = read();

    let args = Args::parse();

    let digest = hash_digest(&args.hash_algorithm, &bytes);

    let hex = encode_hex(digest, args.upper_case.unwrap_or(false));

    write(hex);
}
