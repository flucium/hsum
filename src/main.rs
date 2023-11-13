mod manifest;
use clap::{Parser, ValueEnum};
use digest::{generic_array::GenericArray, Digest, OutputSizeUser};
use md5::Md5;
use memmap2::MmapOptions;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512, Sha512_256};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use std::{
    
    fs::File,
    io::{stdin, stdout, Read, Write},
    os::fd::{AsRawFd, FromRawFd},
};

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

    #[clap(alias = "sha3", alias = "sha3-256", alias = "sha3_256")]
    Sha3_256,

    #[clap(alias = "sha3-384", alias = "sha3_384")]
    Sha3_384,

    #[clap(alias = "sha3-512", alias = "sha3_512")]
    Sha3_512,
}

/// Encode bytes to hex
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

/// Write bytes to stdout
fn write(bytes: impl AsRef<[u8]>) {
    let mut stdout = stdout().lock();

    if let Err(err) = stdout
        .write_all(bytes.as_ref())
        .and(stdout.write(b"\n").and(stdout.flush()))
    {
        panic!("Error: {}", err.to_string());
    }
}

fn compute<H: Digest>(
    mut hasher: H,
    mut file: &File,
) -> std::result::Result<GenericArray<u8, <H as OutputSizeUser>::OutputSize>, String> {
    let file_metadata = match file.metadata() {
        Err(err) => Err(err.to_string())?,
        Ok(metadata) => metadata,
    };

    if file_metadata.is_file() == false {
        // Todo!
        Err(String::default())?
    }

    let file_len = file_metadata.len();

    // It is adapted to the operation of BLAKE3 memmap2::MmapOptions as much as possible.
    // https://github.com/BLAKE3-team/BLAKE3
    // https://github.com/BLAKE3-team/BLAKE3/blob/master/LICENSE
    // https://github.com/BLAKE3-team/BLAKE3/blob/master/src/io.rs
    // https://github.com/BLAKE3-team/BLAKE3/blob/master/src/lib.rs
    // https://ja.wikipedia.org/wiki/65536
    if file_len == 0 || file_len > isize::MAX as u64 || file_len < 16 * 1024 {
        let mut buf = [0; 65536];

        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }

            hasher.update(&buf[..n]);

            buf = [0; 65536];
        }
    } else {
        hasher.update(
            unsafe { MmapOptions::new().len(file_len as usize).map(file) }
                .map_err(|err| String::from(err.to_string()))?,
        );
    }

    let hash = hasher.finalize();

    Ok(hash)
}

fn main() {
    let args = Args::parse();

    let stdin = unsafe { &std::fs::File::from_raw_fd(stdin().as_raw_fd()) };

    match args.hash_algorithm {
        HashAlgorithm::Md5 => match compute(Md5::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha1 => match compute(Sha1::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha256 => match compute(Sha256::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha384 => match compute(Sha384::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha512 => match compute(Sha512::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha512_256 => match compute(Sha512_256::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha3_256 => match compute(Sha3_256::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha3_384 => match compute(Sha3_384::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
        HashAlgorithm::Sha3_512 => match compute(Sha3_512::new(), stdin) {
            Err(err) => {
                panic!("Error: {}", err.to_string());
            }
            Ok(hash) => {
                let hex = encode_hex(hash, args.uppercase.unwrap_or(false));
                write(hex);
            }
        },
    };
}
