use clap::ArgMatches;
use log::info;
use packing_lib::convert::helper::{mean_vec_u16_f64, median_vec_u16_16, remove_zero_new};
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;
use std::fs::File;
use std::io::{self, Write};

pub fn stats_main(matches: &ArgMatches) {
    info!("Stats main");
    let (mut pc, index_present) = read_input(matches);
    let output = matches.value_of("output").unwrap_or("-");
    if output == "-" {
        stats_wrapper(&mut pc, index_present, &mut None);
    } else {
        let mut file = File::create(output).expect("Unable to create file");
        stats_wrapper(&mut pc, index_present, &mut Some(&mut file));
    }
}

fn write_to_file_or_stdout(file: &mut Option<&mut File>, content: &str) -> io::Result<()> {
    match file {
        Some(f) => {
            f.write_all(content.as_bytes())?;
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

pub fn stats_wrapper(pc: &mut PackCompact, _index_present: bool, file2: &mut Option<&mut File>) {
    write_to_file_or_stdout(file2, &format!("Name: {}", pc.name)).expect("Can not write file");
    if pc.is_binary == DataType::TypeBit {
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number presence entries {}",
                pc.bin_coverage.iter().filter(|x| **x).count()
            ),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number absence entries {}",
                pc.bin_coverage.iter().filter(|x| !**x).count()
            ),
        )
        .expect("Can not write file");
    } else if !pc.is_sequence {
        write_to_file_or_stdout(
            file2,
            &format!("Average (with zeros) {}", mean_vec_u16_f64(&pc.coverage)),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (with zeros) {}", median_vec_u16_16(&pc.coverage)),
        )
        .expect("Can not write file");
        let wo = remove_zero_new(&pc.coverage);
        write_to_file_or_stdout(
            file2,
            &format!("Average (without zeros) {}", mean_vec_u16_f64(&wo)),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (without zeros) {}", median_vec_u16_16(&wo)),
        )
        .expect("Can not write file");
    } else {
        write_to_file_or_stdout(
            file2,
            &format!("Average (with zeros) {}", mean_vec_u16_f64(&pc.coverage)),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (with zeros) {}", median_vec_u16_16(&pc.coverage)),
        )
        .expect("Can not write file");
        let wo = remove_zero_new(&pc.coverage);
        write_to_file_or_stdout(
            file2,
            &format!("Average (without zeros) {}", mean_vec_u16_f64(&wo)),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!("Median (without zeros) {}", median_vec_u16_16(&wo)),
        )
        .expect("Can not write file");
    }
}
