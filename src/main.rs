use clap::Parser;
use flate2::read::GzDecoder;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

/// Simple MRT splitter (binary safe)
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    input: PathBuf,

    #[arg(short, long, default_value = "output_chunks")]
    output: PathBuf,

    #[arg(short = 'n', long, default_value_t = 10000)]
    records: usize,

    #[arg(short, long)]
    count: bool,
}

fn open_input(path: &PathBuf) -> Box<dyn Read> {
    let file = File::open(path).expect("❌ Failed to open input file");
    if path.extension().map_or(false, |ext| ext == "gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    }
}

fn read_mrt_record<R: Read>(reader: &mut R) -> Option<Vec<u8>> {
    let mut header = [0u8; 12];
    if reader.read_exact(&mut header).is_err() {
        return None;
    }

    // Length is last 4 bytes of the header
    let length = u32::from_be_bytes([header[8], header[9], header[10], header[11]]) as usize;

    let mut body = vec![0u8; length];
    if reader.read_exact(&mut body).is_err() {
        return None;
    }

    let mut record = Vec::with_capacity(12 + length);
    record.extend_from_slice(&header);
    record.extend_from_slice(&body);

    Some(record)
}

fn main() {
    let args = Args::parse();
    let input = open_input(&args.input);
    let mut reader = BufReader::new(input);

    if args.count {
        let mut count = 0;
        while let Some(_) = read_mrt_record(&mut reader) {
            count += 1;
            if count % 10000 == 0 {
                print!("... {} records counted\r", count);
                std::io::stdout().flush().unwrap();
            }
        }
        println!("\n🧮 Total MRT records: {}", count);
        return;
    }

    // Splitting mode
    create_dir_all(&args.output).expect("❌ Could not create output directory");

    let mut file_index = 0;
    let mut record_count = 0;
    let mut current_writer = BufWriter::new(
        File::create(args.output.join(format!("part_{:03}.mrt", file_index)))
            .expect("❌ Could not create output file"),
    );

    while let Some(record) = read_mrt_record(&mut reader) {
        current_writer.write_all(&record).unwrap();
        record_count += 1;

        if record_count >= args.records {
            current_writer.flush().unwrap();
            file_index += 1;
            record_count = 0;
            current_writer = BufWriter::new(
                File::create(args.output.join(format!("part_{:03}.mrt", file_index)))
                    .expect("❌ Could not create next output file"),
            );
        }
    }

    current_writer.flush().unwrap();
    println!("✅ Done. Created {} file(s) in {:?}", file_index + 1, args.output);
}
