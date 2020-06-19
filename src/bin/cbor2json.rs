use std::fs::File;
use std::io::{BufReader, BufWriter};
use anyhow::Result;
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Usage: cbor2json <input.cbor> <output.json>");
        return Ok(())
    }

    let fin = File::open(&args[1])?;
    let fout = File::create(&args[2])?;

    let reader = BufReader::new(fin);
    let writer = BufWriter::new(fout);

    let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
    let mut serializer = serde_json::Serializer::new(writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;

    Ok(())
}
