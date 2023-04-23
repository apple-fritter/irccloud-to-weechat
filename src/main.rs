use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use zip::read::ZipArchive;

fn main() {
    let file = File::open("irccloud_logs.zip").unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let path = Path::new(file.name());

        // Skip any files that are not log files
        if !path.extension().unwrap().to_str().unwrap().eq("txt") {
            continue;
        }

        let reader = BufReader::new(file);
        let mut output = File::create(format!("{}.weechatlog", path.file_stem().unwrap().to_str().unwrap())).unwrap();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.splitn(3, ' ').collect();

            // Skip any lines that do not contain enough information
            if parts.len() < 3 {
                continue;
            }

            let timestamp = format!("{}:{}:{}", &parts[0][0..2], &parts[0][3..5], &parts[0][6..8]);
            let message = format!("[{}] {}", timestamp, &parts[2]);

            // Write the message to the output file
            writeln!(output, "{}", message).unwrap();
        }
    }
}
