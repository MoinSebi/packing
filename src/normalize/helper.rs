use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};

/// u16 vector to u8 vector
pub fn vec_u16_to_u8(input_vec: &Vec<u16>) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; input_vec.len() * 2];
    BigEndian::write_u16_into(input_vec, &mut buffer);

    buffer
}

/// u16 vector to u8 vector
pub fn vec_f32_to_u8(input_vec: &Vec<f32>) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; input_vec.len() * 4];
    BigEndian::write_f32_into(input_vec, &mut buffer);

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

pub fn mean<T>(data: &[T]) -> f64
where
    T: std::ops::Add<Output = T> + std::convert::From<u8> + Copy,
    f64: std::convert::From<T>,
{
    let sum: f64 = data.iter().map(|&x| f64::from(x)).sum();
    sum / (data.len() as f64)
}

pub fn median<T>(data: &mut [T]) -> f64
where
    T: PartialOrd + Copy,
    f64: From<T>,
{
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = data.len();
    if len % 2 == 0 {
        let mid = len / 2;

        (f64::from(data[mid - 1]) + f64::from(data[mid])) / 2.0
    } else {
        let mid = len / 2;
        f64::from(data[mid])
    }
}


pub fn percentile<T>(data: &[T], percentile: f64) -> f64
    where
        T: PartialOrd + Copy,
        f64: From<T>,
{

    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted_data.len() as f64;
    let index = percentile * (len - 1.0);

    let lower_index = index.floor() as usize;
    let upper_index = index.ceil() as usize;

    if lower_index == upper_index {
        f64::from(sorted_data[lower_index])
    } else {
        let lower_value = f64::from(sorted_data[lower_index]);
        let upper_value = f64::from(sorted_data[upper_index]);
        let fraction = index - lower_index as f64;
        lower_value + fraction * (upper_value - lower_value)
    }
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
pub fn normalize_u16_f32(input_vec: &Vec<u16>, absolute_thresh: &f32) -> Vec<u8> {
    let mut new_vec: Vec<f32> = Vec::new();
    for item in input_vec.iter() {
        new_vec.push((*item as f32) / *absolute_thresh)
    }
    vec_f32_to_u8(&new_vec)
}

/// Normalize/scale the vector by a value
pub fn normalize_f32_f32(input_vec: &Vec<f32>, absolute_thresh: &f32) -> Vec<u8> {
    let mut new_vec: Vec<f32> = Vec::new();
    for item in input_vec.iter() {
        new_vec.push(*item / *absolute_thresh)
    }
    vec_f32_to_u8(&new_vec)
}

/// Create binary vector
pub fn vec2binary(vecc: Vec<u16>, absolute_thresh: &f32) -> Vec<u8> {
    let mut bv: BitVec<u8, Msb0> = BitVec::new();
    for x in vecc.iter() {
        if (*x as f32) >= *absolute_thresh {
            bv.push(true)
        } else {
            bv.push(false)
        }
    }

    bv.into_vec()
}

pub fn vec2binary_f32(vecc: Vec<f32>, absolute_thresh: &f32) -> Vec<u8> {
    let mut bv: BitVec<u8, Msb0> = BitVec::new();
    for x in vecc.iter() {
        if *x >= *absolute_thresh {
            bv.push(true)
        } else {
            bv.push(false)
        }
    }

    bv.into_vec()
}

//-------------------------------------------------------------------------------------------------------
// Compression

/// Remove all zeros from a vector
pub fn remove_zero(vecc: &mut Vec<u16>) {
    vecc.retain(|&x| x != 0);
}

pub fn remove_zero_f32(vecc: &mut Vec<f32>) {
    vecc.retain(|&x| x != 0.0);
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

pub fn calculate_std_deviation(data: &[u16]) -> f64 {
    let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;

    let variance = data.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / data.len() as f64;

    variance.sqrt()
}

pub fn calculate_std_deviation_f32(data: &[f32]) -> f64 {
    let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;

    let variance = data.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / data.len() as f64;

    variance.sqrt()
}

// Standard variation function of u16 vec
