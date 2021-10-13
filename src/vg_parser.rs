use crate::core::Pack;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::helper::mean_vec_u16;

/// Parsing a vg pack file COMPLETE
/// Output as pack
pub fn parse_all(filename: &str) -> Pack {
    // Reader basics
    let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
    let reader = BufReader::new(file);



    let name = filename;
    let s2:Vec<&str> = name.split("/").collect();

    let s = s2.last().unwrap().clone();
    //
    let mut p = Pack::new();
    p.name = s.to_string();



    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        // not the header
        if i != 0{
            let line_split: Vec<&str> = l.split("\t").collect();

            p.seq.push(line_split[0].parse().unwrap());
            p.node.push(line_split[1].parse().unwrap());
            p.offset.push(line_split[2].parse().unwrap());
            p.cov.push(line_split[3].parse().unwrap());
        }
        else {
            p.header = l;
        }
    }
    p
}


/// Node parser
/// Return a boolean vector for all nodes
pub fn parse_node_thresh(filename: &str, thresh: u16 ) -> (String, Vec<bool>){
    let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
    let reader = BufReader::new(file);


    //
    let name = filename.to_string();


    let mut test1: Vec<bool> = Vec::new();
    let mut j = 1;
    let mut o: Vec<u16> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        // not the header
        if i != 0{
            let line_split: Vec<&str> = l.split("\t").collect();


            let node: u32  = line_split[1].parse().unwrap();       // Node
            let _h2: u32  = line_split[0].parse().unwrap();      // Off set
            let h3: u16 = line_split[3].parse().unwrap();      // Coverage
            if node != j{
                let mm = mean_vec_u16(&o);
                if mm >= thresh{
                    test1.push(true);
                } else {
                    test1.push(false);
                }
                j = node;
                o = vec![h3];
            } else {
                o.push(h3)
            }


        }
    }
    (name, test1)
}



/// Node parser
/// Return a boolean vector for all nodes
///
///
pub fn parse_node_mean(filename: &str) -> (String, Vec<u16>){
    let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
    let reader = BufReader::new(file);


    //
    let name = filename.to_string();


    let mut test1: Vec<u16> = Vec::new();
    let mut j = 1;
    let mut o: Vec<u16> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        // not the header
        if i != 0{
            let line_split: Vec<&str> = l.split("\t").collect();


            let node: u32  = line_split[1].parse().unwrap();       // Node
            //let h2:u32  = line_split[0].parse().unwrap();      // Off set
            //let h3: u16 = line_split[3].parse().unwrap();      // Coverage
            let h3: u16;
            match line_split[3].parse::<u16>() {
                Ok(n) => h3 = n ,
                Err(_e) => h3 = u16::MAX,
            };
            if node != j{
                let mm = mean_vec_u16(&o);
                test1.push(mm);
                j = node;
                o = vec![h3];
            } else {
                o.push(h3)
            }


        }
    }
    (name, test1)
}
