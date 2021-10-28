
mod core;
mod vg_parser;
mod helper;
mod writer;
mod reader;

use clap::{App, Arg};
use crate::vg_parser::{parse_node_mean, parse_node_thresh, parse_smart};
use crate::writer::{write_file, writer_compress};
use crate::helper::{vec_u16_u8, binary2u8};
use std::env;
use getopts::Options;
use crate::core::PackCompact;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {


    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("packing")
        .arg(Arg::new("vg")
            .short('v')
            .long("vg")
            .about("vg file")
            .takes_value(true)
            .default_value("/home/svorbrugg_local/Rust/packing/9986.100k.txt"))
        .arg(Arg::new("type")
            .short('t')
            .long("type")
            .about("n|s (nodes or sequence)")
            .takes_value(true)
            .default_value("n"))
        .arg(Arg::new("threshhold")
            .short('d')
            .long("threshhold")
            .about("threshhold")
            .takes_value(true))
        .arg(Arg::new("out")
            .short('o')
            .long("out")
            .about("Output name")
            .takes_value(true)
            .default_value("pack"))
        .arg(Arg::new("coverage")
            .short('c')
            .long("coverage")
            .about("Take coverage not nodes"))
        .arg(Arg::new("compress")
            .short('s'))
        .arg(Arg::new("pb")
            .short('p')
            .takes_value(true)
        )


        .get_matches();

    /*
    Input is  vg pack file
    If you want the coverage, make result
    if ou give a threshold, then the outcome is bitwise
    esle it is in u16 -> two byte
    For calculation:
    - if coverage + no thres -> wc-l -1 x2
    - if coverage + thresh - wc -l / 8
    - if node = thresh = cut -f 2 uniq wc -l x2
    - if node  tresh same as above but + 2
     */



    // Collect the name

    let name: &str = matches.value_of("vg").unwrap();
    let s2:Vec<&str> = name.split("/").collect();
    let s = s2.last().unwrap().clone();

    println!("Packing tool");
    let mean_node_out:Vec<u8>;
    if matches.is_present("compress"){
        eprintln!("dsakdjaskld");
        let p =  parse_smart(matches.value_of("vg").unwrap());
        let buf = p.compress();
        let buf2 = p.compress2();
        let buf4 = p.compress4();
        writer_compress(&buf, "testing/test.compress");
        writer_compress(&buf2, "testing/test2.compress");
        writer_compress(&buf4, "testing/test4.compress");


    }

    if matches.is_present("pb"){
        let mut p = PackCompact::new();
        p.read_complete(matches.value_of("pb").unwrap());
    }




    if matches.is_present("vg"){
        if matches.is_present("coverage"){
            if matches.is_present("threshhold") {
                let thresh: u16 = matches.value_of("threshhold").unwrap().parse().unwrap();
                let p =  parse_smart(matches.value_of("vg").unwrap());
                mean_node_out = p.coverage2byte_thresh_bit(&thresh);
                write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
            } else {
                let p = parse_smart(matches.value_of("vg").unwrap());
                mean_node_out = p.coverage2byte();
                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            }
        }
        else { // this is for nodes
            if matches.is_present("threshhold") {
                let thresh: u16 = matches.value_of("threshhold").unwrap().parse().unwrap();
                let (name, mean_node) = parse_node_thresh(matches.value_of("vg").unwrap(), thresh);
                mean_node_out = binary2u8(&mean_node);
                write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
            } else {
                let (name, mean_node) = parse_node_mean(matches.value_of("vg").unwrap());
                mean_node_out = vec_u16_u8(&mean_node);
                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            }
        }
    }
}


