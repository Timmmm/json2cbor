use std::fs::File;
use std::io::{BufReader, BufWriter};
use anyhow::Result;
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: json2cbor <input.json> <output.cbor>");
        return Ok(())
    }

    let fin = File::open(&args[1])?;
    let fout = File::create(&args[2])?;

    let reader = BufReader::new(fin);
    let writer = BufWriter::new(fout);
    let cbor_writer = serde_cbor::ser::IoWrite::new(writer);

    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let mut serializer = serde_cbor::Serializer::new(cbor_writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;

    Ok(())
}
