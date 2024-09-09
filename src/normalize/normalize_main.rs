use crate::core::reader::{get_input_args, read_input2};
use crate::core::writer::writer_compress_zlib;
use crate::normalize::convert_helper::Method;
use crate::normalize::helper::{normalize_f32_f32, normalize_u16_f32};
use clap::ArgMatches;
use log::{info, warn};

use crate::core::core::{DataType, PackCompact};

use std::process;

pub fn normalize_main(matches: &ArgMatches) {
    let input_pack = get_input_args(matches, "pack");
    let input_index = get_input_args(matches, "index");
    let input_pc = get_input_args(matches, "pc");

    let (mut pc, index_present) = read_input2(&input_pack, &input_index, &input_pc);

    // Check if the right data
    if pc.data_type == DataType::TypeBit {
        warn!("You loaded a presence/absence file. You are not able to further normalize it.");
        process::exit(0x0100);
    }

    // you need the index to normalize
    if !index_present {
        warn!("There is no index file.");
        process::exit(0x0100);
    }

    // If it is binary, you are not able to normalize anything
    if pc.coverage.is_empty() && pc.normalized_coverage.is_empty() {
        warn!("The data is empty");
        process::exit(0x0100);
    }

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    let absolute_thresh: u16 = matches
        .value_of("absolute-threshold")
        .unwrap_or("0")
        .parse()
        .expect("Failed to parse absolute threshold");
    let mut fraction: f32 = matches
        .value_of("fraction")
        .unwrap_or("0.0")
        .parse()
        .unwrap();
    if fraction < 0.0 {
        warn!("Fraction is negative");
        panic!("Exiting");
    }

    let method_string = matches.value_of("method").unwrap_or("nothing");
    let mut method = Method::from_str(method_string);
    let include_all = matches.is_present("keep-zeros");

    let mut want_sequence = !matches.is_present("node");

    let real_thresh: f32;
    // This is default
    if !matches.is_present("fraction")
        && method == Method::Nothing
        && !matches.is_present("absolute-threshold")
    {
        info!("No method or fraction given, using default");
        method = Method::Percentile;
        fraction = 0.1;
    }
    // Check is absolute threshold is given
    if absolute_thresh > 0 {
        method = Method::Absolute;
        fraction = 0.0;
        real_thresh = absolute_thresh as f32;
        if matches.is_present("node") && pc.is_sequence {
            pc.calc_node_cov();
        }
        // if not, give method and fraction
    } else if method == Method::Nothing && fraction != 0.0 {
        warn!("No method or fraction given");
        panic!("Exiting");
    } else if fraction == 0.0 {
        warn!("Relative threshold is 0");
        panic!("Exiting");
    } else if method == Method::Nothing {
        warn!("No method or fraction given");
        panic!("Exiting");
    } else {
        if matches.is_present("node") && pc.is_sequence {
            pc.calc_node_cov();
        }
        real_thresh = PackCompact::get_threshold(&pc, include_all, fraction, 0.0, method);
    }


    if !pc.is_sequence {
        want_sequence = false;
    }

    info!("New parameters");
    info!(
        "Feature: {}",
        if want_sequence { "sequence" } else { "node" }
    );
    info!("Method: {}", method.to_string());
    info!("Absolute threshold: {}", absolute_thresh);
    info!("Relative threshold: {}", fraction);
    info!("Include all: {}", include_all);
    info!("Real threshold: {}", real_thresh);

    // The vector we work with

    let number_entries;
    let mut buffer = Vec::new();

    if pc.normalized_coverage.is_empty() {
        buffer.extend(normalize_u16_f32(&pc.coverage, &real_thresh));
        number_entries = pc.coverage.len();
    } else {
        buffer.extend(normalize_f32_f32(&pc.normalized_coverage, &real_thresh));
        number_entries = pc.normalized_coverage.len();
    }

    info!("Number of entries: {}", number_entries);

    let mut bb = PackCompact::file_header(
        want_sequence,
        include_all,
        DataType::TypeF32,
        method,
        fraction,
        0.0,
        real_thresh,
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);

    writer_compress_zlib(&bb, matches.value_of("out").unwrap());
}
