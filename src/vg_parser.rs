use crate::core::{PackCompact};
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::info;



/// Reading a VG pack file (tabular format).
///
/// Iterating over every line in the file and push node and coverage to the PackCompact field.
///
/// Input:
///     - filename: &str --> Name of the VG pack file
/// Output:
///     - PackCompact
///
///
pub fn parse_smart(filename: &str) -> PackCompact {
    let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
    let reader = BufReader::new(file);
    let mut pc: PackCompact = PackCompact::new();
    let mut count = 0;
        for (i, line) in reader.lines().enumerate(){
        let l = line.unwrap();
        if i != 0{
            let line_split: Vec<&str> = l.split("\t").collect();
            let no: u32 = line_split[1].parse().unwrap();
            let cov: u16;
            if let Ok(x) = line_split[3].parse::<u16>(){
                cov = x;
                count += 1;
            } else {
                cov = u16::MAX
            }
            pc.node.push(no);
            pc.coverage.push(cov);

        }
    }
    info!("{} entries were truncated.", count);
    pc
}


