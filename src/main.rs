
mod core;
mod vg_parser;
mod helper;
mod writer;
mod reader;

use clap::{App, Arg};
use crate::vg_parser::{parse_smart};
use crate::writer::{write_file, writer_compress, write_pack};
use crate::helper::{vec_u16_u8, binary2u8};
use std::{ process};
use crate::core::PackCompact;
use std::path::Path;
use crate::reader::wrapper_meta;


fn main() {


    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("packing")
        .arg(Arg::new("pack")
            .short('p')
            .long("pack")
            .about("vg pack file")
            .takes_value(true))
        .arg(Arg::new("meta")
            .short('m')
            .takes_value(true))
        .arg(Arg::new("coverage")
            .short('c')
            .takes_value(true))
        .arg(Arg::new("binary pack")
            .short('b')
            .takes_value(true))
        .arg(Arg::new("sequence")
            .short('s')
            .long("sequence")
            .about("sequence [default: nodes]"))
        .arg(Arg::new("threshold")
            .short('t')
            .long("threshold")
            .about("threshold")
            .takes_value(true))
        .arg(Arg::new("out")
            .short('o')
            .long("out")
            .about("Output name")
            .takes_value(true)
            .default_value("pack"))
        .arg(Arg::new("output meta")
            .long("outmeta")
            .about("Write Metafile to this file")
            .takes_value(true))
        .arg(Arg::new("output coverage")
            .long("outcov")
            .about("Write Coverage to this file")
            .takes_value(true))
        .arg(Arg::new("output binary packing")
            .long("outbpack")
            .about("Write complete file to this file")
            .takes_value(true))
        .arg(Arg::new("output packing")
            .long("outpack")
            .about("Write complete file to this file")
            .takes_value(true))



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
    eprintln!("Packing tool");






    // Read input:
    let mut s = "";
    let mut p: PackCompact = PackCompact::new();
    let mut no_file = false;
    if matches.is_present("pack") | (matches.is_present("meta") & matches.is_present("coverage")) | (matches.is_present("binary pack")){
        if matches.is_present("pack"){
            if Path::new(matches.value_of("pack").unwrap()).exists(){
                p =  parse_smart(matches.value_of("pack").unwrap());
                let name: &str = matches.value_of("pack").unwrap();
                let s2:Vec<&str> = name.split("/").collect();
                s = s2.last().unwrap().clone();
            } else {
                no_file = true;
            }
        }
        else if matches.is_present("binary pack") {
            if Path::new(matches.value_of("binary pack").unwrap()).exists() {
                p = PackCompact::new();
                p.read_complete(matches.value_of("binary pack").unwrap());
                let name: &str = matches.value_of("binary pack").unwrap();
                let s2: Vec<&str> = name.split("/").collect();
                s = s2.last().unwrap().clone();
            } else {
                no_file = true;
            }
        }
        else {
            if Path::new(matches.value_of("coverage").unwrap()).exists() & Path::new(matches.value_of("meta").unwrap()).exists() {
                eprintln!("the file is");
                p = wrapper_meta(matches.value_of("meta").unwrap(), matches.value_of("coverage").unwrap());
                let name: &str = matches.value_of("coverage").unwrap();
                let s2: Vec<&str> = name.split("/").collect();
                s = s2.last().unwrap().clone();
            }else {
                no_file = true;
            }
        }
    }

    if no_file{
        eprintln!("There is no input file");
    }
    if p.coverage.len() == 0{
        eprintln!("There is a problem with the input files");

        process::exit(0x0100);
    } else {
        eprintln!("File is {}", s)
    }



    // Output
    if matches.is_present("output meta"){
        let buf = p.compress_only_node();
        writer_compress(&buf, matches.value_of("output meta").unwrap());

    }

    if matches.is_present("output coverage"){
        let buf = p.compress_only_coverage();
        writer_compress(&buf, matches.value_of("output coverage").unwrap());
    }

    if matches.is_present("output binary packing"){
        let buf = p.compress_all();
        writer_compress(&buf, matches.value_of("output binary packing").unwrap());
    }
    if matches.is_present("output packing"){
        write_pack(&p, matches.value_of("output packing").unwrap())
    }



    // Cat output
    let mean_node_out: Vec<u8>;
    if matches.is_present("coverage"){
        if matches.is_present("threshold"){
            let thresh: u16 = matches.value_of("threshold").unwrap().parse().unwrap();
            mean_node_out = p.coverage2byte_thresh_bit(&thresh);
            write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
        } else {
            mean_node_out = p.coverage2byte();
            write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
        }
    } else {
        if matches.is_present("threshold") {
            let thresh: u16 = matches.value_of("threshold").unwrap().parse().unwrap();
            mean_node_out = binary2u8(&p.node2byte_thresh(thresh));
            write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
        } else {
            mean_node_out = vec_u16_u8(&p.node2byte());
            write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
        }
    }
}


