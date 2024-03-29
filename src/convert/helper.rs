use crate::convert::convert_helper::{Method};
use crate::core::core::PackCompact;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use log::{info, warn};

/// u16 vector to u8 vector
pub fn vec_u16_to_u8(input_vec: &Vec<u16>) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; input_vec.len() * 2];
    BigEndian::write_u16_into(input_vec, &mut buffer);

    buffer
}

/// Calculate the average of a vector
///
/// - u16 in
/// - f64 out
pub fn mean_vec_u16_f64(val: &Vec<u16>) -> f64 {
    let sums: u64 = val.iter().fold(0, |mut sum, &val| {
        sum += val as u64;
        sum
    });

    (sums as f64) / (val.len() as f64)
}

/// Calculate the average of a vector
///
/// - u16 in
/// - u16 out
pub fn mean_vec_u16_u16(val: &Vec<u16>) -> u16 {
    let sums: u64 = val.iter().fold(0, |mut sum, &val| {
        sum += val as u64;
        sum
    });

    ((sums as f64) / (val.len() as f64)).round() as u16
}

/// Calculate the median of a vector
///
/// - u16 in
/// - u16 out
pub fn median_vec_u16_16(numbers: &Vec<u16>) -> f64 {
    let mut num = numbers.clone();
    num.sort();
    let mid = num.len() / 2;
    num[mid] as f64
}

/// u32 -> 4xu8
pub fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;

    [b1, b2, b3, b4]
}

/// Byte to string
///
/// Alternativ to  std::str::from_utf8
/// https://doc.rust-lang.org/std/str/fn.from_utf8.html
pub fn byte_to_string(input: &[u8]) -> String {
    let mut o = "".to_string();
    for x in input.iter() {
        o.push(*x as char);
    }
    o
}

/// Normalize/scale the vector by a value
pub fn normalize_u16_u16(input_vec: &Vec<u16>, absolute_thresh: &f64) -> Vec<u16> {
    if absolute_thresh == &0.0 {
        return input_vec.clone();
    }
    let mut new_vec: Vec<u16> = Vec::new();
    for item in input_vec.iter() {
        new_vec.push(((*item as f64) / (*absolute_thresh)).round() as u16)
    }
    new_vec
}

/// Create binary vector
pub fn vec2binary(vecc: Vec<u16>, absolute_thresh: &f64) -> Vec<u8> {
    let mut bv: BitVec<u8, Msb0> = BitVec::new();
    for x in vecc.iter() {
        if (*x as f64) >= *absolute_thresh {
            bv.push(true)
        } else {
            bv.push(false)
        }
    }

    bv.into_vec()
}

pub fn make_header(
    sequence_out: bool,
    is_binary: bool,
    method: Method,
    r: u16,
    thresh: &u16,
    length: u32,
    name: &str,
) -> Vec<u8> {
    let mut buff: Vec<u8> = vec![53, 56];

    // Is node?
    if sequence_out {
        buff.push(1);
    } else {
        buff.push(0);
    }

    // Is binary?
    if is_binary {
        buff.push(1);
    } else {
        buff.push(0);
    }

    match method {
        Method::Nothing => buff.push(0),
        Method::Mean => buff.push(1),
        Method::Median => buff.push(2),
        Method::Percentile => buff.push(3),
    }

    // Relative threshold
    let mut buff2 = vec![0; 2];
    BigEndian::write_u16(&mut buff2, r);
    buff.extend(buff2);
    // Real Threshold
    let mut buff2 = vec![0; 2];
    BigEndian::write_u16(&mut buff2, *thresh);
    buff.extend(buff2);

    // Length of the vector
    let mut buff2 = vec![0; 4];
    BigEndian::write_u32(&mut buff2, length);
    buff.extend(buff2);

    // Name
    let char_vec: Vec<char> = name.chars().collect();
    for c in char_vec.iter() {
        buff.push(*c as u8);
    }
    // Add space
    for _x in 0..(64 - char_vec.len()) {
        buff.push(32);
    }
    buff
}

//-------------------------------------------------------------------------------------------------------
// Compression

/// Remove all zeros from a vector
pub fn remove_zero(vecc: &mut Vec<u16>) {
    vecc.retain(|&x| x != 0);
}

pub fn remove_zero_new(vecc: &Vec<u16>) -> Vec<u16> {
    vecc.iter().cloned().filter(|x| *x != 0).collect::<Vec<_>>()
}

/// Get the name of the file
///
/// - Remove the "prefix" of a data path
pub fn remove_prefix_filename(filename: &str) -> String {
    let name: &str = filename;

    let s2: Vec<&str> = name.split('/').collect();
    return s2.last().unwrap().parse().unwrap();
}

/// Calculate the real threshold
///
/// Based on
/// - method
/// - relative threshold
/// - include_all
/// - node or sequence
pub fn get_real_threshold(
    pc: &mut PackCompact,
    include_all: bool,
    relative: u16,
    tt: Method,
) -> f64 {
    // "work_on" is the current data we do the normalizcation on
    let mut work_on: Vec<u16> = pc.coverage.clone();

    // relative is 0
    if relative == 0 {
        warn!("Relative threshold is 0");
        return 0.0;
    }

    if !include_all {
        remove_zero(&mut work_on)
    }

    let mut thresh: f64 = 0.0;
    if tt == Method::Percentile {
        work_on.sort();
        thresh = work_on
            [((work_on.len() as f64 - 1.0) * ((relative as f64) / 100_f64)).round() as usize]
            as f64;
        info!("{}% Percentile is {}", relative, thresh);
        info!("Working threshold is {}", thresh);
        return thresh;
    } else if tt == Method::Mean {
        thresh = mean_vec_u16_f64(&work_on);
        info!("Mean is {}", thresh);
        thresh *= (relative as f64) / 100_f64;
        info!("Working threshold is {}", thresh);
    } else if tt == Method::Median {
        thresh = median_vec_u16_16(&work_on);
        info!("Median is {}", thresh);
        thresh *= (relative as f64) / 100_f64;
        info!("Working threshold is {}", thresh);
    }
    thresh
}

// Standard variation function of u16 vec
