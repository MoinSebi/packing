use clap::ArgMatches;
use std::collections::HashSet;
use std::fs::File;

use log::{info, warn};
use std::io::Write;

use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::{get_input_args, read_input2};

pub fn view_main(matches: &ArgMatches) {
    info!("View main");
    let input_pack = get_input_args(matches, "pack");
    let input_index = get_input_args(matches, "index");
    let input_pc = get_input_args(matches, "pack compressed");
    if input_pack.is_empty() && input_pc.is_empty() {
        warn!("No input file");
        panic!("[-h, --help] for help information");
    }
    if !input_pc.is_empty() && input_index.is_empty() {
        warn!("No index file");
        panic!("[-h, --help] for help information");
    }


    let (mut pc, index_present) = read_input2(&input_pack, &input_index, &input_pc);
    let output = matches.value_of("output").unwrap_or("output.txt");
    println!("{:?}", pc.node_index[0]);
    if !index_present {
        warn!("There is no index file.");
        panic!("Exiting");
    } else {
        view_wrapper(&mut pc, output);
    }
}

pub fn view_wrapper(pc: &mut PackCompact, outfile: &str) {
    let mut f = File::create(outfile).expect("Unable to create file");
    pc.print_meta();
    info!("View wrapper");

    if pc.is_sequence {
        if pc.data_type == DataType::TypeBit {
            let mut node = 0;
            writeln!(
                f,
                "{}\t{}\t{}\t{}",
                "seq.pos", "node.id", "node.offset", "coverage"
            ).expect("Can not write view file (header)");
            writeln!(
                f,
                "{}\t{}\t{}\t{}",
                0, pc.node_index[0], node, if {pc.bin_coverage[0] == true} {1} else {0}
            );
            for x in 1..pc.bin_coverage.len() {
                if pc.node_index[x] == pc.node_index[x - 1] {
                    node += 1;
                    writeln!(
                        f,
                        "{}\t{}\t{}\t{}",
                        x, pc.node_index[x], node, if {pc.bin_coverage[x] == true} {1} else {0}
                    )
                    .expect("Can not write file");
                } else {
                    node = 0;
                    writeln!(
                        f,
                        "{}\t{}\t{}\t{}",
                        x, pc.node_index[x], node, if {pc.bin_coverage[x] == true} {1} else {0}
                    )
                    .expect("Can not write file");
                }
            }
        } else if pc.data_type == DataType::TypeU16 {
            pc.write_pack(outfile);
        } else {
            let mut node = 0;
            writeln!(
                f,
                "{}\t{}\t{}\t{}",
                "seq.pos", "node.id", "node.offset", "coverage"
            ).expect("Can not write view file (header)");
            writeln!(
                f,
                "{}\t{}\t{}\t{}",
                0, pc.node_index[0], node, pc.normalized_coverage[0]
            ).expect("Can not write file");
            for x in 1..pc.normalized_coverage.len() {
                if pc.node_index[x] == pc.node_index[x - 1] {
                    node += 1;
                    writeln!(
                        f,
                        "{}\t{}\t{}\t{}",
                        x, pc.node_index[x], node, pc.normalized_coverage[x]
                    )
                        .expect("Can not write file");
                } else {
                    node = 0;
                    writeln!(
                        f,
                        "{}\t{}\t{}\t{}",
                        x, pc.node_index[x], node, pc.normalized_coverage[x]
                    )
                        .expect("Can not write file");
                }
            }
        }

        // is node
    } else {
        let nodes: HashSet<_> = pc.node_index.iter().collect();
        let mut nodes: Vec<_> = nodes.into_iter().collect();
        nodes.sort();
        writeln!(
            f,
            "{}\t{}",
            "node", "coverage"
        ).expect("Can not write view file (header)");
        if pc.data_type == DataType::TypeBit {
            for (i, x) in nodes.iter().enumerate() {
                writeln!(f, "{}\t{}", x, pc.coverage[i]).expect("Can not write file");
            }
        } else if pc.data_type == DataType::TypeU16 {
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

}
