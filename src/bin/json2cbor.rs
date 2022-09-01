use std::fs::File;
use std::io::{BufReader, BufWriter};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Input file to read from. Reads from stdin if not present.
    #[clap(value_parser, id="INPUT_FILE")]
    input: Option<String>,

    /// Output file to write to. Writes to stdout if not present.
    #[clap(short, long, id="OUTPUT_FILE")]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let fin: Box<dyn std::io::Read> =
        match args.input {
            None => Box::new(std::io::stdin()),
            Some(file_name) => Box::new(File::open(file_name)?),
        };

    let fout: Box<dyn std::io::Write> =
        match args.output {
            None => Box::new(std::io::stdout()),
            Some(file_name) => Box::new(File::create(file_name)?),
        };

    let reader = BufReader::new(fin);
    let writer = BufWriter::new(fout);
    let cbor_writer = serde_cbor::ser::IoWrite::new(writer);

    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let mut serializer = serde_cbor::Serializer::new(cbor_writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;

    Ok(())
}
