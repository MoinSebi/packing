use std::io::{Write, Read};

/// bool vector to u8 vector
pub fn binary2u8(vecc: &Vec<bool>) -> Vec<u8>{
    let mut buff: Vec<u8> = Vec::new();


    let j: Vec<&[bool]> = vecc.chunks(8).collect();
    for x in j {
        buff.push(binary2dec_bed(x));
    }

    buff
}

/// Bool verctor (max size 8) to u8
pub fn binary2dec_bed(vecc: &[bool]) -> u8{
    let mut result: u8 = 0;
    let mut count = 0;
    let t: u8 = 2;
    for x in vecc.iter().rev(){

        result += (t.pow(count as u32)) * (*x as u8);
        count += 1;
    }
    result
}


/// u16 vector to u8 vector
pub fn vec_u16_u8(vecc: &Vec<u16>) -> Vec<u8>{
    let mut buff: Vec<u8> = Vec::new();
    for x in vecc.iter(){
        buff.extend(transform_u16_to_array_of_u8(x.clone()));
    }

    buff
}


/// u16 vector to u8 vector
pub fn vec_u16_u82(vecc: &Vec<u32>) -> Vec<u8>{
    let mut buff: Vec<u8> = Vec::new();
    for x in vecc.iter(){
        buff.extend(transform_u32_u16to_array_of_u8(x.clone()));
    }

    buff
}

/// Same function than "transform_u16_to_array_of_u8"
pub fn transform_u32_u16to_array_of_u8(x1:u32) -> [u8;2] {
    let x = x1 as u16;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b3, b4]
}


/// Transform u16 to 2 u8
pub fn transform_u16_to_array_of_u8(x:u16) -> [u8;2] {
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b3, b4]
}



/// Mean of a vector
pub fn mean_vec_u16(val: &Vec<u16>) -> u16{
    let su: u16= val.iter().sum();
    let j:u16  = (su as u16)/(val.len() as u16);
    j
}



/// u32 -> 4xu8
pub fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    let array = [b1, b2, b3, b4];
    return array
}



pub fn byte_to_bitvec(buf: &u8) -> Vec<bool>{
    let mut h: Vec<bool> = Vec::new();
    let mut n = buf.clone();
    while n > 0{
        h.push((n%2)  == 1);
        n = n/2
    }
    for _x in 0..(8-h.len()){
        h.insert(0, false);
    }
    h
}


/// Byte to string
///
/// Alternativ to  std::str::from_utf8
/// https://doc.rust-lang.org/std/str/fn.from_utf8.html
pub fn byte_to_string(input: &[u8]) -> String {
    let mut o = "".to_string();
    for x in input.iter(){

        o.push(x.clone() as char);

    }
    return o
}




// pub fn read_be_u32(input: & mut &[u8]) -> u32 {
//     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
//     *input = rest;
//     u32::from_be_bytes(int_bytes.try_into().unwrap())
// }
//
// pub fn read_be_u16(input: &mut &[u8]) -> u16 {
//     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
//     *input = rest;
//     u16::from_be_bytes(int_bytes.try_into().unwrap())
// }

pub fn u8_u16(vector: &[u8]) -> u16{
    let number = ((vector[0] as u16) << 8) | vector[1] as u16;
    number
}

pub fn u8_u322(vector: &[u8]) -> u32{
    let number = ((vector[0] as u32) << 24) |((vector[1] as u32) << 16) |((vector[2] as u32) << 8) | vector[3] as u32;
    number
}






/// Coverts two bytes to a 16
pub fn byte2u16(vector: &[u8]) -> u16{
    let number = ((vector[0] as u16) << 8) | vector[1] as u16;
    number
}


pub fn zstd_encode(v: &Vec<u8>) -> Vec<u8>{
    let mut e = zstd::Encoder::new(Vec::new(), 0).unwrap();
    e.write_all(v).expect("Not working");
    let com = e.finish().unwrap();
    return com
}

pub fn zstd_decode(bytes: Vec<u8>) -> Vec<u8> {
    let mut gz = zstd::Decoder::new(&bytes[..]).unwrap();
    let mut k: Vec<u8> = Vec::new();
    println!("{:?}", gz.read(& mut k));
    k
}


#[cfg(test)]
mod helper {
    use crate::helper::{transform_u32_to_array_of_u8, u8_u322, transform_u16_to_array_of_u8, u8_u16, mean_vec_u16, binary2u8, byte_to_bitvec, byte_to_string};

    #[test]
    fn test_converter(){
        assert_eq!(10, u8_u322(&transform_u32_to_array_of_u8(10)));
    }

    #[test]
    fn test_u16(){
        assert_eq!(10, u8_u16(&transform_u16_to_array_of_u8(10)));
    }

    #[test]
    fn mean_test(){
        assert_eq!(10, mean_vec_u16(&vec![11,10,0,9,20]))
    }

    #[test]
    fn bit_vec(){
        assert_eq!(vec![224,224], binary2u8(&vec![true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, false]))
    }

    #[test]
    fn u32_u8(){
        assert_eq!(vec![59, 154, 202, 0], transform_u32_to_array_of_u8(1000000000));
    }

    #[test]
    fn u8_bit(){
        assert_eq!(byte_to_bitvec(&3), vec![false, false, false, false, false, false, true, true])
    }

    #[test]
    fn u8_string(){
        assert_eq!(byte_to_string(&vec![116, 101, 115,116]), "test".to_string());
    }



}








