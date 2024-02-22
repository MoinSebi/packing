use crate::convert::helper::{byte_to_string, remove_prefix_filename};
use crate::core::core::PackCompact;
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
/// Get the meta data from the binary pack file (73 bytes)
/// Outputs sequence/Node, length, thresh, name
pub fn get_meta(buffer: &[u8]) -> (bool, bool, u8, u16, u16, u32, u32, String) {
    let cov = buffer[2];
    let bin = buffer[3];
    let method = buffer[4];
    let r = BigEndian::read_u16(&buffer[5..7]);
    let thresh = BigEndian::read_u16(&buffer[7..9]);
    let length = BigEndian::read_u32(&buffer[9..13]);
    let mut bytes = length * 2;
    if bin == 1 {
        bytes = length.div_ceil(8);
    }

    let name = byte_to_string(&buffer[13..77]);
    let name = name.trim_matches(char::from(0)).to_string();

    (cov == 1, bin == 1, method, r, thresh, bytes, length, name)
}

#[allow(dead_code)]
/// Reading multiple files at the same time
///
///
pub fn wrapper_bool(buffer: &Vec<u8>) -> Vec<PackCompact> {
    // total length 73 + len
    let (_kind, _bin, _method, _relative, _thresh, _length, bytes, _name) = get_meta(buffer);

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
pub fn wrapper_u16(buffer: &Vec<u8>) -> Vec<PackCompact> {
    // total length 73 + len
    let (_kind, _bin, _method, _relative, _thresh, _length, bytes, _name) = get_meta(buffer);
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
    let nodes = read_index(file_index);
    let mut p = PackCompact::wrapp(file_pc);
    p.node = nodes;
    p
}

pub fn read_input(matches: &clap::ArgMatches) -> PackCompact {
    let mut p: PackCompact = PackCompact::new();
    let mut no_file = false;
    // Determine Input format
    if matches.is_present("pack")
        | (matches.is_present("index") & matches.is_present("compressed pack"))
    {
        // READ "NORMAL" PACK FILE
        if matches.is_present("pack") {
            if Path::new(matches.value_of("pack").unwrap()).exists() {
                p = PackCompact::parse_pack(matches.value_of("pack").unwrap());
            } else {
                no_file = true;
            }
        }
        //READ COVERAGE AND META
        else if Path::new(matches.value_of("index").unwrap()).exists()
            & Path::new(matches.value_of("compressed pack").unwrap()).exists()
        {
            p = wrapper_compressed(
                matches.value_of("index").unwrap(),
                matches.value_of("compressed pack").unwrap(),
            );
        } else {
            no_file = true;
        }
    }
    if no_file {
        info!("There is no input file");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    }
    p
}

impl PackCompact {
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
                pc.node.push(no);
                pc.coverage.push(cov);
            }
        }
        pc.name = remove_prefix_filename(filename);
        info!(
            "{} entries have been truncated (have a coverage above 65,535).",
            count
        );
        pc
    }

    pub fn read_bin_coverage(buffer: &[u8]) -> Self {
        let (_kind, _bin, _method, _relative, _thresh, _length, _bytes, name) = get_meta(buffer);
        debug!("Name {}", name);
        let bv: BitVec<u8, Msb0> = BitVec::from_slice(&buffer[77..]);
        PackCompact {
            name,
            node: Vec::new(),
            node_coverage: Vec::new(),
            bin_coverage: bv,
            coverage: Vec::new(),
        }
    }

    pub fn wrapp(file_pc: &str) -> Self {
        let buff = unpack_zstd_to_byte(file_pc);
        let meta = get_meta(&buff);
        if meta.1 {
            
            Self::read_bin_coverage(&buff)
        } else {
            
            Self::read_u16(&buff)
        }
    }

    pub fn read_u16(buffer: &[u8]) -> Self {
        let (_kind, _bin, _method, _relative, _thresh, _length, _bytes, name) = get_meta(buffer);

        debug!("Name {}", name);
        let mut data = vec![0; buffer[77..].len() / 2];
        BigEndian::read_u16_into(&buffer[77..], &mut data);
        PackCompact {
            name,
            node: Vec::new(),
            node_coverage: Vec::new(),
            bin_coverage: BitVec::new(),
            coverage: data,
        }
    }
}
