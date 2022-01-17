use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use anyhow::bail;
use clap::App;
use clap::Arg;

#[derive(Debug)]
struct Arguments {
    input_file: String,
    output_file: String,
    can_borrow: bool,
}

fn parse_arguments() -> Result<Arguments> {
    let app = App::new("trace_to_job_input")
        .version("0.1.0")
        .author("Vassilis Vassiliadis")
        .about("Convert Task trace to job input for \"dismem\" simulator")
        .arg(Arg::new("inputFile")
            .short('i')
            .long("inputFile")
            .takes_value(true)
            .required(true)
            .help("Path to the input trace .csv file"))
        .arg(Arg::new("outputFile")
            .short('o')
            .long("outputFile")
            .takes_value(true)
            .required(true)
            .help("Path to the output file"))
        .arg(Arg::new("borrow:y/n")
            .short('b')
            .long("borrowResources")
            .help("Whether output Jobs can borrow resources from nodes other the one that they \
            scheduled on. Default is: no")
            .possible_values(["y", "n"])
            .required(true));

    let args = app.get_matches();
    let input_file = args.value_of("inputFile").unwrap();
    let input_file = input_file.to_owned();

    let output_file = args.value_of("outputFile").unwrap();
    let output_file = output_file.to_owned();

    let can_borrow = args.value_of("borrow:y/n").unwrap();
    let can_borrow: bool = can_borrow.eq_ignore_ascii_case("y");

    Ok(Arguments { input_file, output_file, can_borrow })
}

fn main() -> Result<()> {
    let args = parse_arguments()?;

    let file = File::open(Path::new(&args.input_file))
        .context(format!("Unable to open input file{}", args.input_file))?;
    let br = BufReader::new(file);

    let file = File::create(Path::new(&args.output_file))
        .context(format!("Unable to create output file {}", args.output_file))?;
    let mut out = BufWriter::new(file);
    writeln!(&mut out, "#uid:usize;cores:f32;memory:f32;duration:f32;borrow:y/n;time_created:f32")?;

    let borrow = match args.can_borrow {
        true => 'y',
        false => 'n'
    };

    for (i, line) in br.lines().enumerate() {
        let line = line.context(format!("Unable to read line {}", i))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // VV: input line format is:
        // time_created:f32; time_started:f32; time_done:f32; cores:f32; memory:f32
        let tokens: Vec<_> = line.split(';').map(|s| s.trim()).collect();
        if tokens.len() != 5 {
            bail!("Line {} \"{}\" contains {} values instead of 5", i, line, tokens.len())
        }

        let time_created: f32 = tokens[0].parse()
            .context(format!("Unable to parse time_created:f32 {}", tokens[0]))?;
        let time_started: f32 = tokens[1].parse()
            .context(format!("Unable to parse time_started:f32 {}", tokens[1]))?;
        let time_done: f32 = tokens[2].parse()
            .context(format!("Unable to parse time_done:f32 {}", tokens[2]))?;
        let cores: f32 = tokens[3].parse()
            .context(format!("Unable to parse cores:f32 {}", tokens[3]))?;
        let memory: f32 = tokens[4].parse()
            .context(format!("Unable to parse memory:f32 {}", tokens[4]))?;

        let duration = time_done - time_started;
        // VV: output format is:
        // uid:usize; cores:f32; memory:f32; duration:f32; borrow:y/n; time_created:f32
        writeln!(&mut out, "?;{};{};{};{};{}", cores, memory, duration, borrow, time_created)?;
    }

    Ok(())
}