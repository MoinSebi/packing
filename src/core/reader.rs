use crate::normalize::convert_helper::Method;

use crate::core::core::{DataType, PackCompact};
use crate::normalize::helper::{byte_to_string, remove_prefix_filename};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::{fs, process};

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
    let mut file = File::open(filename).expect("no file found");
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
pub fn wrapper_bool(buffer: &[u8]) -> Vec<PackCompact> {
    // total length 73 + len
    let (_kind, _bin, _method, _relative, _std, _thresh, bytes, _length, _name) =
        PackCompact::get_meta(buffer);

    let chunks = buffer.chunks((bytes + 77) as usize);
    let mut result: Vec<PackCompact> = Vec::new();
    info!("Number of samples: {}", chunks.len());

    for chunk in chunks.into_iter() {
        result.push(PackCompact::read_bin_coverage(chunk));
    }
    result
}

#[allow(dead_code)]
/// Parse multiple files
///
/// Iterate over each sample
pub fn wrapper_u16(buffer: &[u8]) -> Vec<PackCompact> {
    // total length 73 + len
    let (_kind, _bin, _method, _relative, _std, _thresh, bytes, _length, _name) =
        PackCompact::get_meta(buffer);
    let chunks = buffer.chunks((bytes + 77) as usize);

    info!("Number of samples: {}", chunks.len());
    let mut result: Vec<PackCompact> = Vec::new();
    for chunk in chunks.into_iter() {
        result.push(PackCompact::read_u16(chunk));
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

pub fn read_input(matches: &clap::ArgMatches) -> (PackCompact, bool) {
    let mut p: PackCompact = PackCompact::new();
    let mut no_file = false;
    let mut index_present = false;
    // Determine Input format

    if matches.is_present("pack")
        | (matches.is_present("index") & matches.is_present("pack compressed"))
        | matches.is_present("pack compressed")
    {
        // READ "NORMAL" PACK FILE
        if matches.is_present("pack") {
            if Path::new(matches.value_of("pack").unwrap()).exists() {
                p = PackCompact::parse_pack(matches.value_of("pack").unwrap());
                index_present = true;
            } else {
                no_file = true;
            }
        }
        //READ COVERAGE AND META
        else if matches.is_present("index") && matches.is_present("pack compressed") {
            if Path::new(matches.value_of("index").unwrap()).exists()
                & Path::new(matches.value_of("pack compressed").unwrap()).exists()
            {
                p = wrapper_compressed(
                    matches.value_of("index").unwrap(),
                    matches.value_of("pack compressed").unwrap(),
                );
                index_present = true;
            }
        } else if Path::new(matches.value_of("pack compressed").unwrap()).exists() {
            p = PackCompact::read_wrapper(matches.value_of("pack compressed").unwrap());
        } else {
            no_file = true;
        }
    } else {
        no_file = true;
    }
    if no_file {
        info!("There is no input file");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    }
    (p, index_present)
}

impl PackCompact {
    /// Get the meta data from the binary pack file (73 bytes)
    /// Outputs sequence/Node, length, thresh, name
    pub fn get_meta(buffer: &[u8]) -> (bool, DataType, Method, f32, f32, f32, u32, u32, String) {
        let is_sequence = buffer[2];
        let bin = DataType::from_u8(buffer[3]);
        let method = buffer[4];
        let r = BigEndian::read_f32(&buffer[5..9]);
        let aaa = BigEndian::read_f32(&buffer[9..13]);
        let thresh = BigEndian::read_f32(&buffer[13..17]);
        let length = BigEndian::read_u32(&buffer[17..21]);

        let mut bytes = length * 2;
        if bin == DataType::TypeBit {
            bytes = length.div_ceil(8);
        } else if bin == DataType::TypeF32 {
            bytes *= 2;
        }

        let mut name = byte_to_string(&buffer[21..85]);
        name = name.trim_matches(char::from(0)).to_string();
        name = name.trim_end().to_string();

        (
            is_sequence == 1,
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

    pub fn parse_pack(filename: &str) -> Self {
        let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
        let reader = BufReader::new(file);
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
        pc.print_meta();
        pc
    }

    /// Wrapper for PC reading.
    pub fn read_wrapper(file_pc: &str) -> Self {
        let buff = unpack_zstd_to_byte(file_pc);
        let meta = PackCompact::get_meta(&buff);
        if meta.1 == DataType::TypeBit {
            Self::read_bin_coverage(&buff)
        } else if DataType::TypeU16 == meta.1 {
            Self::read_u16(&buff)
        } else {
            Self::read_f32(&buff)
        }
    }

    pub fn read_bin_coverage(buffer: &[u8]) -> Self {
        let (_kind, _bin, _method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);
        debug!("Name1 {}", name);
        let mut bv: BitVec<u8, Msb0> = BitVec::from_slice(&buffer[85..]);
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

    pub fn read_u16(buffer: &[u8]) -> Self {
        let (_kind, _bin, method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);

        debug!("Name {}", name);
        let mut data = vec![0; buffer[85..].len() / 2];
        BigEndian::read_u16_into(&buffer[85..], &mut data);
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

    pub fn read_f32(buffer: &[u8]) -> Self {
        let (_kind, _bin, method, _relative, std, _thresh, _bytes, length, name) =
            PackCompact::get_meta(buffer);

        debug!("Name {}", name);
        let mut data = vec![0.0; buffer[85..].len() / 4];
        BigEndian::read_f32_into(&buffer[85..], &mut data);
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
