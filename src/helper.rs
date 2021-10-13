
pub fn binary2u8(vecc: Vec<bool>) -> Vec<u8>{
    let mut buff: Vec<u8> = Vec::new();


    let j: Vec<&[bool]> = vecc.chunks(8).collect();
    for x in j {
        buff.push(binary2dec_bed(x));
    }

    buff
}



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

pub fn vec_u16_u8(vecc: &Vec<u16>) -> Vec<u8>{
    let mut buff: Vec<u8> = Vec::new();



    for x in vecc.iter(){
        buff.extend(transform_u16_to_array_of_u8(x.clone()));
    }

    buff
}




/// Mean of a vector
pub fn mean_vec_u16(val: &Vec<u16>) -> u16{
    let su: u16= val.iter().sum();
    let j:u16  = (su as u16)/(val.len() as u16);
    j
}





pub fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

pub fn to_u32_2(slice: &[bool]) -> u32{
    slice.iter().rev().fold(0, |acc, &b| acc*2 + b as u32)
}

pub fn transform_u16_to_array_of_u8(x:u16) -> [u8;2] {
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b3, b4]
}



pub fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}
