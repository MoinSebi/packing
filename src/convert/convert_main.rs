use crate::convert::convert_helper::{Method, OutputType};
use crate::convert::helper::{
    get_real_threshold, make_header, normalize_u16_u16, vec2binary, vec_u16_to_u8,
};
use crate::core::reader::read_input;
use crate::core::writer::{writer_compress, writer_compress_zlib};
use clap::ArgMatches;
use log::{debug, info, warn};

use std::process;

pub fn convert_main(matches: &ArgMatches) {
    let (mut pc, index_present) = read_input(matches);

    let bin = matches.is_present("binary");
    let normalize = matches.is_present("normalize");
    let absolute_thresh: u16 = matches
        .value_of("absolute threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let relative_thresh: u16 = matches
        .value_of("relative threshold")
        .unwrap_or("100")
        .parse()
        .unwrap();
    let method_string = matches.value_of("method").unwrap_or("nothing");
    let method = Method::from_str(method_string);
    let include_all = matches.is_present("non-covered");

    info!("Method: {}", method.to_string());
    info!("Absolute threshold: {}", absolute_thresh);
    info!("Relative threshold: {}", relative_thresh);
    info!("Include all: {}", include_all);
    info!("Binary: {}", bin);
    info!("Normalize: {}", normalize);

    if !index_present {
        warn!("There is no index file.");
        process::exit(0x0100);
    }

    if !pc.bin_coverage.is_empty() {
        warn!("You loaded a presence/absence file. You are not able to further convert it.");
        process::exit(0x0100);
    }

    let real_thresh;

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    if pc.coverage.is_empty() {
        info!("There is a problem with the input files. Run 'packing info' on your file.");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    } else {
        info!("File is {}", pc.name)
    }

    // Checking the output base (sequence, nodes) or pack file
    let mut out_type = OutputType::Sequence;
    if matches.is_present("type") {
        out_type = OutputType::from_str(matches.value_of("type").unwrap());
    }

    info!("Output type: {}", out_type.to_string());

    // Write the pack
    if out_type == OutputType::Pack {
        debug!("Writing pack file");
        pc.write_pack(matches.value_of("out").unwrap());
        process::exit(0x0100);
    }

    // Absolute threshold is adjusted is made with thresh
    if absolute_thresh == 0 {
        real_thresh = get_real_threshold(&mut pc, out_type, include_all, relative_thresh, method);
    } else {
        real_thresh = absolute_thresh;
    }
    info!("Real threshold: {}", real_thresh);

    // The vector we work with
    let mut output: Vec<u16>;
    if out_type == OutputType::Node {
        pc.calc_node_cov();
        output = pc.node_coverage;
    } else {
        output = pc.coverage;
    }

    if normalize {
        output = normalize_u16_u16(output, &real_thresh);
    }

    let number_entries = output.len();
    let buffer: Vec<u8>;
    if bin {
        buffer = vec2binary(output, &real_thresh);
    } else {
        buffer = vec_u16_to_u8(&output);
    }
    let mut bb = make_header(
        out_type,
        bin,
        method,
        relative_thresh,
        &absolute_thresh,
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);
    if matches.is_present("non-compressed") {
        writer_compress(&bb, matches.value_of("out").unwrap());
    } else {
        writer_compress_zlib(&bb, matches.value_of("out").unwrap());
    }
}
