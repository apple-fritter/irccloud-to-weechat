use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

struct WeechatLine {
    timestamp: String,
    sender: String,
    message: String,
}

fn main() {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: irccloud-to-weechat <input_zip>");
        return;
    }

    // Extract the input file path from the arguments
    let input_zip = &args[1];

    // Open the zip archive
    let archive = match ZipArchive::new(File::open(input_zip)?) {
        Ok(archive) => archive,
        Err(err) => {
            println!("Failed to open zip archive: {}", err);
            return;
        }
    };

    // Process each file in the archive
    for i in 0..archive.len() {
        let entry = match archive.by_index(i) {
            Ok(entry) => entry,
            Err(err) => {
                println!("Failed to read entry: {}", err);
                continue;
            }
        };

        // Extract the file name and extension
        let file_name = entry.name();
        let path = Path::new(file_name);
        let file_name_stem = path.file_stem().unwrap().to_str().unwrap();
        let output_file_name = format!("{}.weechatlog", file_name_stem);

        // Create the output file
        let output_file = match File::create(output_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to create output file: {}", err);
                continue;
            }
        };

        let output_path = PathBuf::from(&output_file_name);

        // Process the lines of the input file
        let reader = BufReader::new(entry);
        let weechat_lines: Vec<WeechatLine> = reader
            .lines()
            .filter_map(|line| parse_line(&line.ok()?, &output_path))
            .collect();

        // Write the Weechat-formatted lines to the output file
        let mut output_writer = BufWriter::new(output_file);
        for weechat_line in weechat_lines {
            writeln!(
                output_writer,
                "{} {} {}",
                weechat_line.timestamp,
                weechat_line.sender,
                weechat_line.message
            )
            .unwrap();
        }
    }

    println!("Conversion completed successfully!");
}

fn parse_line(line: &str, output_path: &Path) -> Option<WeechatLine> {
    // Extract the date, time, sender, and message from the line
    let parts: Vec<&str> = line.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return None;
    }

    let timestamp = parts[0];
    let sender = parts[1];
    let message = parts[2];

    Some(WeechatLine {
        timestamp: timestamp.to_owned(),
        sender: sender.to_owned(),
        message: message.to_owned(),
    })
}
