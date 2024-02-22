use std::collections::HashSet;
use std::io::BufWriter;
use clap::ArgMatches;
use std::fs::File;
use std::io::{self, Write};
use crate::core::core::PackCompact;


use crate::core::reader::read_input;

pub fn view_main(matches: &ArgMatches){
    let (mut pc, index_present) = read_input(matches);

    let output = matches.value_of("output").unwrap_or("output.txt");

    view_wrapper(& mut pc, index_present, output);
}

pub fn view_wrapper(pc: &mut PackCompact, index_present: bool, outfile: &str) {
    let mut f = File::create(outfile).expect("Unable to create file");

    // If index is present
    if index_present {
        // complex
        if pc.is_sequence {
            if pc.is_binary {
                let mut node = 0;
                for x in 0..pc.coverage.len() {
                    if x == 0 {
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node[x], node, pc.coverage[x]
                        )
                            .expect("Can not write file");
                    } else if pc.node[x] == pc.node[x - 1] {
                        node += 1;
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node[x], node, pc.bin_coverage[x]
                        )
                            .expect("Can not write file");
                    } else {
                        node = 0;
                        writeln!(
                            f,
                            "{}\t{}\t{}\t{}",
                            x, pc.node[x], node, pc.coverage[x]
                        )
                            .expect("Can not write file");
                    }
                }
            } else {
                pc.write_pack("test")
            }

            // is node
        } else {
            let mut nodes: HashSet<_> = pc.node.iter().collect();
            let mut nodes: Vec<_> = nodes.into_iter().collect();
            nodes.sort();
            if !pc.is_binary {
                for (i, x) in nodes.iter().enumerate() {
                    writeln!(f, "{}\t{}", x, pc.node_coverage[i]);
                }
            } else {
                for (i, x) in nodes.iter().enumerate() {
                    writeln!(f, "{}\t{}", x, pc.bin_coverage[i]);
                }
            }
        }
    } else {
        if pc.is_sequence {
            if pc.is_binary{
                for  x in pc.bin_coverage.iter(){
                    writeln!(f, "{}",  if *x {1} else {0});
                    }
                }
             else {
                for x in pc.coverage.iter(){
                    writeln!(f, "{}", x);
                }
            }

        } else {
            if pc.is_binary{
                for x in pc.bin_coverage.iter(){
                    writeln!(f, "{}", if *x {1} else {0});
                }
            } else {
                for x in pc.node.iter() {
                    writeln!(f, "{}", x);
                }
            }
        }
    }


}
