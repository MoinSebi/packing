
mod core;
mod vg_parser;
mod helper;
mod writer;
mod reader;
mod index;
mod info;


use clap::{App, Arg };
use crate::vg_parser::{parse_smart};
use crate::writer::{write_file, write_pack, writer_compress_zlib};
use crate::helper::{vec_u16_u8, binary2u8, vec_f32_u82};
use std::{ process};
use crate::core::PackCompact;
use std::path::Path;
use chrono::Local;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use crate::reader::wrapper_meta;
use std::io::Write;

fn main() {


    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("packing")
        .subcommand(App::new("info")
            .about("Information about the compressed index")
            .version("0.1.0")
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .required(true)
                .about("Information about the binary index file")
                .takes_value(true))
            .arg(Arg::new("exact")
                .short('e')
                .long("exact")
                .about("Check if length are the same"))
            .arg(Arg::new("all")
                .short('a')
                .long("all")
                .about("Check all entries (for concatenated index)")))
        .subcommand(App::new("index")
            .about("Index a graph (gfa format)")
            .version("0.1.0")
            .arg(Arg::new("gfa")
                .short('g')
                .long("gfa")
                .about("gfa for index")
                .takes_value(true))
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("pack format after alignment")
                .takes_value(true))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file")
                .takes_value(true)
                .required(true)))

        .subcommand(App::new("convert")
            .about("Convert VG PACK format for a compact index structure (partially reversible)")
            // Input
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

            // Type
            .arg(Arg::new("sequence")
                .short('s')
                .long("sequence")
                .about("sequence [default: nodes]"))


            // Modification
            .arg(Arg::new("threshold")
                .short('t')
                .long("threshold")
                .about("Threshold after normalizing (in %)")
                .takes_value(true))
            .arg(Arg::new("absolute threshold")
                .long("absolute threshold")
                .about("absolute threshold")
                .takes_value(true))
            // If you normalize, pls use me
            .arg(Arg::new("normalize")
                .short('n')
                .long("normalize")
                .about("Normalize everything")
                .takes_value(true))




            //Output
            .arg(Arg::new("out")
                .short('o')
                .long("out")
                .about("Output name")
                .takes_value(true)
                .default_value("pack")
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
                .takes_value(true)))



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

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .target(Target::Stderr)
        .init();



    // Collect the name
    info!("Packing tool");


    // INDEX
    if let Some(ref matches) = matches.subcommand_matches("index") {
        if matches.is_present("gfa")  {
            let j = matches.value_of("gfa").unwrap();
            let o = matches.value_of("output").unwrap();
            index::index_main::make_index(&j, o);
        } else if matches.is_present("pack"){
            let o = matches.value_of("output").unwrap();
            let p =  parse_smart(matches.value_of("pack").unwrap());
            let buf = p.compress_only_node();
            writer_compress_zlib(&buf, o);



        } else {
            info!("No input")
        }
        process::exit(0x0100);

    }

    if let Some(ref matches) = matches.subcommand_matches("info") {
        info!("Index info");
        if matches.is_present("exact"){
            if matches.is_present("all"){
                info::info::stats(matches.value_of("index").unwrap(), true, true);
            } else {
                info::info::stats(matches.value_of("index").unwrap(), true, false);
            }
        } else {
            info::info::stats(matches.value_of("index").unwrap(), false, false);
        }
        process::exit(0x0100);

    }

    // CONVERT
    if let Some(ref matches) = matches.subcommand_matches("convert"){
        let mut s = "";
        let mut p: PackCompact = PackCompact::new();
        let mut no_file = false;
        // Determine Input format
        if matches.is_present("pack") | (matches.is_present("meta") & matches.is_present("coverage")) | (matches.is_present("binary pack")){
            // READ "NORMAL" PACK FILE
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
                // READ BINARY PACK
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
                //READ COVERAGE AND META
            else {
                if Path::new(matches.value_of("coverage").unwrap()).exists() & Path::new(matches.value_of("meta").unwrap()).exists() {
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
            info!("There is no input file");
            process::exit(0x0100);
        }
        if p.coverage.len() == 0{
            info!("There is a problem with the input files");
            process::exit(0x0100);
        } else {
            info!("File is {}", s)
        }


        // OUTPUT FOR SPECIAL OUTPUT
        if matches.is_present("output coverage"){
            let buf = p.compress_only_coverage();
            writer_compress_zlib(&buf, matches.value_of("output coverage").unwrap());
        }

        if matches.is_present("output binary packing"){
            let buf = p.compress_all();
            writer_compress_zlib(&buf, matches.value_of("output binary packing").unwrap());
        }
        if matches.is_present("output packing"){
            write_pack(&p, matches.value_of("output packing").unwrap())
        }



        // NORMALIZING
        if matches.is_present("normalize"){
            p.normalize_wrapper("median");
        }

        if matches.is_present("threshold"){
            if ! matches.is_present("normalize"){
                p.normalize_wrapper("median");
            }
        }



        // THE REAL OUTPUT
        // ABSOLUTE THRESHOLD -> NO NORMALIZE
        // THRESHOLD -> NORMALIZE
        let mut mean_node_out: Vec<u8>;
        if matches.is_present("sequence"){
            info!("USING SEQUENCE");
            if matches.is_present("absolute threshold"){
                info!("Absolute threshold");
                let thresh: u16 = matches.value_of("absolute threshold").unwrap().parse().unwrap();
                mean_node_out = p.coverage2byte_thresh_bit(&thresh);
                write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
            } else  if matches.is_present("threshold"){
                info!("Threshold");
                let t: f32  = matches.value_of("threshold").unwrap().parse().unwrap();
                let thresh = t/ 100 as f32;
                mean_node_out = p.coverage2byte_thresh_normalized(&thresh);
                write_file(s, &mean_node_out, 1, matches.value_of("out").unwrap(), true)
            } else if matches.is_present("normalized"){
                info!("Normalized");
                mean_node_out = p.coverage2byte_normalized();
                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            } else {
                mean_node_out = p.coverage2byte();

                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            }
        } else {
            info!("USING NODES");
            if matches.is_present("absolute threshold") {
                info!("Absolute threshold");
                let thresh: u16 = matches.value_of("absolute threshold").unwrap().parse().unwrap();
                mean_node_out = binary2u8(&p.node2byte_thresh(&thresh));
                write_file(s, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
            } else  if matches.is_present("threshold"){
                info!("Threshold");
                let t: f32  = matches.value_of("threshold").unwrap().parse().unwrap();
                // This is very important
                let thresh = t/ 100 as f32;
                mean_node_out = binary2u8(&p.node2byte_thresh_normalized(&thresh));
                write_file(s, &mean_node_out, 1, matches.value_of("out").unwrap(), true)
            } else if matches.is_present("normalized") {
                info!("Normalized");
                mean_node_out = vec_f32_u82(&p.node2byte_normalized());
                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            } else {
                mean_node_out = vec_u16_u8(&p.node2byte());
                write_file(s, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            }
        }

    }


    // Read input:

}


