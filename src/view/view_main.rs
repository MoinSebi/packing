use clap::ArgMatches;
use std::collections::HashSet;
use std::fs::File;

use log::info;
use std::io::Write;

use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;

pub fn view_main(matches: &ArgMatches) {
    info!("View main");
    let (mut pc, index_present) = read_input(matches);
    let output = matches.value_of("output").unwrap_or("output.txt");

    view_wrapper(&mut pc, index_present, output);
}

pub fn view_wrapper(pc: &mut PackCompact, index_present: bool, outfile: &str) {
    let mut f = File::create(outfile).expect("Unable to create file");

    // If index is present
    if index_present {
        // complex
        if pc.is_sequence {
            if pc.is_binary == DataType::TypeBit {
                let mut node = 0;
                for x in 0..pc.coverage.len() {
                    if x == 0 {
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node_index[x], node, pc.coverage[x]
                        )
                        .expect("Can not write file");
                    } else if pc.node_index[x] == pc.node_index[x - 1] {
                        node += 1;
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node_index[x], node, pc.bin_coverage[x]
                        )
                        .expect("Can not write file");
                    } else {
                        node = 0;
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node_index[x], node, pc.coverage[x]
                        )
                        .expect("Can not write file");
                    }
                }
            } else if pc.is_binary == DataType::TypeU16 {
                pc.write_pack(outfile);
            }

            // is node
        } else {
            let nodes: HashSet<_> = pc.node_index.iter().collect();
            let mut nodes: Vec<_> = nodes.into_iter().collect();
            nodes.sort();
            if pc.is_binary == DataType::TypeBit {
                for (i, x) in nodes.iter().enumerate() {
                    writeln!(f, "{}\t{}", x, pc.coverage[i]).expect("Can not write file");
                }
            } else if pc.is_binary == DataType::TypeU16 {
                for (i, x) in nodes.iter().enumerate() {
                    writeln!(f, "{}\t{}", x, pc.bin_coverage[i]).expect("Can not write file");
                }
            } else {
                for (i, x) in nodes.iter().enumerate() {
                    writeln!(f, "{}\t{}", x, pc.normalized_coverage[i])
                        .expect("Can not write file");
                }
            }
        }
    } else if pc.is_sequence {
        if pc.is_binary == DataType::TypeBit {
            println!("Binary sequence");
            for x in pc.bin_coverage.iter() {
                writeln!(f, "{}", if *x { 1 } else { 0 }).expect("Can not write file");
            }
        } else if pc.is_binary == DataType::TypeU16 {
            for x in pc.coverage.iter() {
                writeln!(f, "{}", x).expect("Can not write file");
            }
        } else {
            for x in pc.normalized_coverage.iter() {
                writeln!(f, "{}", x).expect("Can not write file");
            }
        }
    } else {
        if pc.is_binary == DataType::TypeBit {
            for x in pc.bin_coverage.iter() {
                writeln!(f, "{}", if *x { 1 } else { 0 }).expect("Can not write file");
            }
        } else if pc.is_binary == DataType::TypeU16 {
            for x in pc.coverage.iter() {
                writeln!(f, "{}", x).expect("Can not write file");
            }
        } else {
            for x in pc.normalized_coverage.iter() {
                writeln!(f, "{}", x).expect("Can not write file");
            }
        }
    }
}
