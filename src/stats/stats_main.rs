use crate::convert::helper::{mean_vec_u16_u16, median_vec_u16_16, remove_zero_new};
use crate::core::core::PackCompact;
use crate::core::reader::read_input;

use clap::ArgMatches;
use log::info;

pub fn stats_main(matches: &ArgMatches) {
    info!("Stats main");
    let (mut pc, index_present) = read_input(matches);
    stats_wrapper(&mut pc, index_present);
}

pub fn stats_wrapper(pc: &mut PackCompact, _index_present: bool) {
    println!("Name: {}", pc.name);
    if pc.is_binary {
        println!(
            "Number presence entries {}",
            pc.bin_coverage.iter().filter(|x| **x).count()
        );
        println!(
            "Number absence entries {}",
            pc.bin_coverage.iter().filter(|x| !**x).count()
        );
    } else if pc.coverage.is_empty() {
        println!(
            "Average (with zeros) {}",
            mean_vec_u16_u16(&pc.node_coverage)
        );
        println!(
            "Median (with zeros) {}",
            median_vec_u16_16(&pc.node_coverage)
        );
        let wo = remove_zero_new(&pc.node_coverage);
        println!("Average (without zeros) {}", mean_vec_u16_u16(&wo));
        println!("Median (without zeros) {}", median_vec_u16_16(&wo));
    } else {
        println!("Average (with zeros) {}", mean_vec_u16_u16(&pc.coverage));
        println!("Median (with zeros) {}", median_vec_u16_16(&pc.coverage));
        let wo = remove_zero_new(&pc.coverage);
        println!("Average (without zeros) {}", mean_vec_u16_u16(&wo));
        println!("Median (without zeros) {}", median_vec_u16_16(&wo));
    }
}
