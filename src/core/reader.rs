use crate::normalize::convert_helper::Method;

use crate::core::core::{DataType, PackCompact};
use crate::normalize::helper::{byte_to_string, remove_prefix_filename};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use clap::ArgMatches;
use log::{debug, info, warn};
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use zstd::Decoder;

/// Helper function for zstd decoder
/// https://docs.rs/zstd/0.1.9/zstd/struct.Decoder.html
pub fn zstd_decode(bytes: Vec<u8>) -> Vec<u8> {
    let mut gz = zstd::Decoder::new(&bytes[..]).unwrap();
    let mut k: Vec<u8> = Vec::new();
    gz.read_to_end(&mut k).expect("Decoding does not work");
    k
}

/// Read a file byte by byte (whole file)
///
/// Check if file is available and readable. Returns buffer
pub fn unpack_zstd_to_byte(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap_or_else(|_| panic!("No file found {}", filename));
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    // THIS IS A FUCKING JOKE
    file.read_exact(&mut buffer).expect("buffer overflow");

    zstd_decode(buffer)
}

#[allow(dead_code)]
/// Reading multiple files at the same time
///
///
pub fn wrapper_reader(buffer: &[u8]) -> Vec<PackCompact> {
    // total length 85 + len
    let (_kind, _include_all, _bin, _method, _relative, _std, _thresh, bytes, _length, _name) =
        PackCompact::get_meta(buffer);

    let chunks = buffer.chunks((bytes + 86) as usize);
    let mut result: Vec<PackCompact> = Vec::new();
    info!("Number of samples: {}", chunks.len());

    for chunk in chunks.into_iter() {
        if _bin == DataType::TypeU16 {
            result.push(PackCompact::read_u16(chunk));
        } else if _bin == DataType::TypeBit {
            result.push(PackCompact::read_bin_coverage(chunk));
        } else {
            result.push(PackCompact::read_f32(chunk))
        }
    }
    result
}

/// Reads the index file
///
/// u32 only
pub fn read_index(filename: &str) -> Vec<u32> {
    let buf = unpack_zstd_to_byte(filename);
    let mut vec_nodes: Vec<u32> = vec![0; buf.len() / 4];

    BigEndian::read_u32_into(&buf, &mut vec_nodes);

    vec_nodes
}

/// Wrapper for meta + coverage combination
/// https://stackoverflow.com/questions/29445026/converting-number-primitives-i32-f64-etc-to-byte-representations
pub fn wrapper_compressed(file_index: &str, file_pc: &str) -> PackCompact {
    let mut p = PackCompact::read_wrapper(file_pc);
    p.node_index = read_index(file_index);
    p.print_meta();
    p
}

/// Get the input arguments
pub fn get_input_args(args: &ArgMatches, input: &str) -> String {
    let a: String = args.value_of(input).unwrap_or("").parse().unwrap();
    a
}

pub fn read_input2(pack_file: &str, index_file: &str, pc_file: &str) -> (PackCompact, bool) {
    let p: PackCompact;

    if !pack_file.is_empty() {
        p = PackCompact::parse_pack(pack_file);
        (p, true)
    } else if !index_file.is_empty() && !pc_file.is_empty() {
        if Path::new(index_file).exists() && Path::new(pc_file).exists() {
            p = wrapper_compressed(index_file, pc_file);
            (p, true)
        } else {
            warn!("Index or PC file does not exist");
            panic!("[-h, --help] for help information");
        }
    } else if !pc_file.is_empty() {
        if Path::new(pc_file).exists() {
            p = PackCompact::read_wrapper(pc_file);
            (p, false)
        } else {
            warn!("PC file does not exist");
            panic!("[-h, --help] for help information");
        }
    } else {
        warn!("No input file");
        panic!("[-h, --help] for help information");
    }
}

impl PackCompact {
    /// Get the meta data from the binary pack file (73 bytes)
    /// Outputs sequence/Node, length, thresh, name
    pub fn get_meta(
        buffer: &[u8],
    ) -> (
        bool,
        bool,
        DataType,
        Method,
        f32,
        f32,
        f32,
        u32,
        u32,
        String,
    ) {
        let is_sequence = buffer[2];
        let include_all = buffer[3];
        let bin = DataType::from_u8(buffer[4]);
        let method = buffer[5];
        let r = BigEndian::read_f32(&buffer[6..10]);
        let aaa = BigEndian::read_f32(&buffer[10..14]);
        let thresh = BigEndian::read_f32(&buffer[14..18]);
        let length = BigEndian::read_u32(&buffer[18..22]);

        let mut bytes = length * 2;
        if bin == DataType::TypeBit {
            bytes = length.div_ceil(8);
        } else if bin == DataType::TypeF32 {
            bytes *= 2;
        }

        let mut name = byte_to_string(&buffer[22..86]);
        name = name.trim_matches(char::from(0)).to_string();
        name = name.trim_end().to_string();

        (
            is_sequence == 1,
            include_all == 1,
            bin,
            Method::from_u8(method),
            r,
            aaa,
            thresh,
            bytes,
            length,
            name,
        )
    }

    /// # Parse pack file
    pub fn parse_pack(filename: &str) -> Self {

        let reader: Box<dyn Read> = if filename == "-" {
            // Read from standard input
            Box::new(io::stdin())
        } else {
            let path = Path::new(filename);
            if path.exists() == false {
                panic!("ERROR: FILE DOES NOT EXIST\n");
            }

            // Determine the reader based on the file extension
            if path.extension().and_then(|s| s.to_str()) == Some("zst") {
                // Open the compressed file and create a decoder
                let file = File::open(path).expect("ERROR: CAN NOT READ FILE\n");
                Box::new(Decoder::new(file).expect("ERROR: CAN NOT DECODE FILE\n"))
            } else {
                // Open the plain text file
                let file = File::open(path).expect("ERROR: CAN NOT READ FILE\n");
                Box::new(file)
            }
        };

        // Wrap the reader in a BufReader
        let mut reader = BufReader::new(reader);

        let mut pc: PackCompact = PackCompact::new();
        let mut count = 0;
        for (i, line) in reader.lines().enumerate() {
            let l = line.unwrap();
            if i != 0 {
                let line_split: Vec<&str> = l.split('\t').collect();
                let no: u32 = line_split[1].parse().unwrap();
                let cov: u16;
                if let Ok(x) = line_split[3].parse::<u16>() {
                    cov = x;
                } else {
                    cov = u16::MAX;
                    count += 1;
                }
                pc.node_index.push(no);
                pc.coverage.push(cov);
            }
        }
        pc.name = remove_prefix_filename(filename);
        pc.is_sequence = true;
        pc.length = pc.coverage.len() as u32;
        info!(
            "{} entries have been truncated (have a coverage above 65,535).",
            count
        );
        pc
    }

    /// # Wrapper for PC reading.
    pub fn read_wrapper(file_pc: &str) -> Self {
        let buff = unpack_zstd_to_byte(file_pc);
        let meta = PackCompact::get_meta(&buff);
        if meta.2 == DataType::TypeBit {
            Self::read_bin_coverage(&buff)
        } else if DataType::TypeU16 == meta.2 {
            Self::read_u16(&buff)
        } else {
            Self::read_f32(&buff)
        }
    }

    /// # Read the binary pack file
    pub fn read_bin_coverage(buffer: &[u8]) -> Self {
        let (_kind, _include_all, _bin, _method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);
        debug!("Name1 {}", name);
        let mut bv: BitVec<u8, Msb0> = BitVec::from_slice(&buffer[86..]);
        for _i in length as usize..bv.len() {
            bv.pop();
        }
        PackCompact {
            name,
            node_index: Vec::new(),
            normalized_coverage: Vec::new(),
            bin_coverage: bv,
            coverage: Vec::new(),
            is_sequence: _kind,
            data_type: _bin,
            method: _method,
            fraction: _relative,
            std,
            threshold: _thresh,
            length,
        }
    }

    /// # Read the u16 compressed pack
    pub fn read_u16(buffer: &[u8]) -> Self {
        let (_kind, _include_all, _bin, method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);

        debug!("Name {}", name);
        let mut data = vec![0; buffer[86..].len() / 2];
        BigEndian::read_u16_into(&buffer[86..], &mut data);
        PackCompact {
            name,
            node_index: Vec::new(),
            bin_coverage: BitVec::new(),
            normalized_coverage: Vec::new(),
            coverage: data,
            is_sequence: _kind,
            data_type: _bin,
            method,
            fraction: _relative,
            std,
            threshold: _thresh,
            length,
        }
    }

    /// # Read the f32 compressed pack
    pub fn read_f32(buffer: &[u8]) -> Self {
        let (_kind, _include_all, _bin, method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);

        debug!("Name {}", name);
        let mut data = vec![0.0; buffer[86..].len() / 4];
        BigEndian::read_f32_into(&buffer[86..], &mut data);
        PackCompact {
            name,
            node_index: Vec::new(),
            bin_coverage: BitVec::new(),
            normalized_coverage: data,
            coverage: Vec::new(),
            is_sequence: _kind,
            data_type: _bin,
            method,
            fraction: _relative,
            std,
            threshold: _thresh,
            length,
        }
    }
}
