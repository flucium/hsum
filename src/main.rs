mod manifest;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512, Sha512_256};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::io::{stdin, stdout, BufReader, Read, Write};

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[clap(name = manifest::NAME, version = manifest::VERSION, author = manifest::AUTHOR, about = manifest::ABOUT)]
struct Args {
    #[clap(long("hash"), alias("algorithm"), default_value = "sha256")]
    hash_algorithm: HashAlgorithm,

    #[clap(long("uppercase"), short('u'), alias("upper"))]
    uppercase: Option<bool>,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum HashAlgorithm {
    #[clap(alias = "md5")]
    Md5,

    #[clap(alias = "sha1")]
    Sha1,

    #[clap(alias = "sha2", alias = "sha256")]
    Sha256,

    #[clap(alias = "sha384")]
    Sha384,

    #[clap(alias = "sha512")]
    Sha512,

    #[clap(alias = "sha512/256", alias = "sha512-256", alias = "sha512_256")]
    Sha512_256,

    #[clap(alias = "sha3-256", alias = "sha3_256")]
    Sha3_256,

    #[clap(alias = "sha3-384", alias = "sha3_384")]
    Sha3_384,

    #[clap(alias = "sha3-512", alias = "sha3_512")]
    Sha3_512,
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

fn read() -> Vec<u8> {
    let mut buffer = Vec::new();

    if let Err(err) = BufReader::new(stdin()).read_to_end(&mut buffer) {
        panic!("Error: {}", err.to_string());
    }

    buffer
}

fn write(bytes: impl AsRef<[u8]>) {
    if let Err(err) = stdout().write_all(bytes.as_ref()) {
        panic!("Error: {}", err.to_string());
    }
}

fn main() {
    let args = Args::parse();

    let bytes = read();

    let hash: Vec<u8> = match args.hash_algorithm {
        HashAlgorithm::Md5 => Md5::digest(&bytes).to_vec(),
        HashAlgorithm::Sha1 => Sha1::digest(&bytes).to_vec(),
        HashAlgorithm::Sha256 => Sha256::digest(&bytes).to_vec(),
        HashAlgorithm::Sha384 => Sha384::digest(&bytes).to_vec(),
        HashAlgorithm::Sha512 => Sha512::digest(&bytes).to_vec(),
        HashAlgorithm::Sha512_256 => Sha512_256::digest(&bytes).to_vec(),
        HashAlgorithm::Sha3_256 => Sha3_256::digest(&bytes).to_vec(),
        HashAlgorithm::Sha3_384 => Sha3_384::digest(&bytes).to_vec(),
        HashAlgorithm::Sha3_512 => Sha3_512::digest(&bytes).to_vec(),
    };

    let hex = encode_hex(hash, args.uppercase.unwrap_or(false));

    write(hex);
}
