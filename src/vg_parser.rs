use crate::core::{PackCompact};
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::helper::mean_vec_u16;



/// Reading full file - but smarter
///
pub fn parse_smart(filename: &str) -> PackCompact {
    let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
    let reader = BufReader::new(file);
    let mut pc: PackCompact = PackCompact::new();
    for (i, line) in reader.lines().enumerate(){
        let l = line.unwrap();
        if i != 0{
            let line_split: Vec<&str> = l.split("\t").collect();
            let no: u32 = line_split[1].parse().unwrap();
            let cov: u32 = line_split[3].parse().unwrap();
            pc.node.push(no);
            pc.coverage.push(cov);

        }
    }
    pc
}


#[allow(dead_code)]
/// Node only parser
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
            //let h3: u16 = line_split[3].parse().unwrap();      // Coverage
            let h3: u16;
            match line_split[3].parse::<u16>() {
                Ok(n) => h3 = n ,
                Err(_e) => h3 = u16::MAX,
            };
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
    let mm = mean_vec_u16(&o);
    if mm >= thresh{
        test1.push(true);
    } else {
        test1.push(false);
    }
    (name, test1)
}


#[allow(dead_code)]
/// Node only parser -> u16
/// Return a boolean vector for all nodes
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
    let mm = mean_vec_u16(&o);
    test1.push(mm);
    (name, test1)
}

#[cfg(test)]
mod parser {
    use crate::vg_parser::{parse_smart, parse_node_thresh, parse_node_mean};
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn read_smart() {
        let k = parse_smart("testing/9986.100k.txt");
        let k2:HashSet<u32> = HashSet::from_iter(k.node.iter().cloned());
        assert_eq!(k.coverage[2527], 1);
        assert_eq!(k.node[2527], 182);
    }

    #[test]
    fn read_node_thresh() {
        let k = parse_node_thresh("testing/9986.100k.txt", 10);
        //assert_eq!(k.1.len(), 99999);
        assert_eq!(k.1.len(), 7404);
        assert_eq!(k.1[0], false);
        assert_eq!(k.1[96], false);
    }

    #[test]
    fn read_node() {
        let k = parse_node_mean("testing/9986.100k.txt");
        assert_eq!(k.1.len(), 7404);
        assert_eq!(k.1[182], 1);
    }
}
