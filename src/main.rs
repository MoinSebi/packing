mod reader;
mod core;
mod vg_parser;
mod helper;
mod writer;

use clap::{App, Arg};
use crate::vg_parser::{parse_node_mean, parse_node_thresh, parse_all};
use crate::writer::write_file;
use crate::helper::{vec_u16_u8, binary2dec_bed, binary2u8};

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


        .get_matches();

    println!("Packing tool");
    let (name, mean_node): (String, Vec<u16>);
    let mean_node_out:Vec<u8>;
    if matches.is_present("vg"){
        if matches.is_present("coverage"){
            if matches.is_present("threshhold") {
                let thresh: u16 = matches.value_of("threshhold").unwrap().parse().unwrap();
                let p = parse_all(matches.value_of("vg").unwrap());
                mean_node_out = p.cov2u8_thres(&thresh);
                write_file(&p.name, &mean_node_out, thresh, matches.value_of("out").unwrap(), true);
            } else {
                let p = parse_all(matches.value_of("vg").unwrap());
                mean_node_out = p.cov2u8();
                write_file(&p.name, &mean_node_out, 0, matches.value_of("out").unwrap(), true);
            }
        }
        else {
            if matches.is_present("threshhold") {
                let thresh: u16 = matches.value_of("threshhold").unwrap().parse().unwrap();
                let (name, mean_node) = parse_node_thresh(matches.value_of("vg").unwrap(), thresh);
                mean_node_out = binary2u8(mean_node);
                write_file(&name, &mean_node_out, thresh, matches.value_of("out").unwrap(), false);
            } else {
                let (name, mean_node) = parse_node_mean(matches.value_of("vg").unwrap());
                mean_node_out = vec_u16_u8(&mean_node);
                write_file(&name, &mean_node_out, 0, matches.value_of("out").unwrap(), false);
            }
        }
    }


}


#[cfg(test)]
mod tests {
    use crate::vg_parser::parse_all;
    use crate::writer::write_file;
    use crate::core::Pack;

    #[test]
    fn it_works() {
        let (name, mean_node): (String, Vec<u16>);
        let mean_node_out:Vec<u8>;
        let thresh: u16 = 10;
        let p: Pack = parse_all("/home/svorbrugg_local/Rust/packing/9986.100k.txt");
        mean_node_out = p.cov2u8_thres(&thresh);
        write_file(&p.name, &mean_node_out, thresh, "test.cov", true);
    }
}
