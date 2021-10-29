
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


pub fn transform_u32_u16to_array_of_u8(x1:u32) -> [u8;2] {
    let x = x1 as u16;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b3, b4]
}


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



#[cfg(test)]
mod tests {
    use crate::helper::{transform_u32_to_array_of_u8, u8_u322};

    #[test]
    fn test_converter(){
        assert_eq!(10, u8_u322(&transform_u32_to_array_of_u8(10)));
    }
}








