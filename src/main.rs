use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};

struct WeechatLine {
    date: String,
    time: String,
    nick: String,
    msg: String,
}

impl WeechatLine {
    fn new(date: String, time: String, nick: String, msg: String) -> Self {
        WeechatLine {
            date,
            time,
            nick,
            msg,
        }
    }
}

fn parse_line(line: &str) -> Option<WeechatLine> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() == 3 {
        let timestamp = parts[0];
        let nick = parts[1];
        let msg = parts[2];

        let datetime_parts: Vec<&str> = timestamp.split('T').collect();
        if datetime_parts.len() == 2 {
            let date = datetime_parts[0];
            let time = datetime_parts[1];

            Some(WeechatLine::new(date.to_string(), time.to_string(), nick.to_string(), msg.to_string()))
        } else {
            None
        }
    } else {
        None
    }
}

fn convert_log(input_path: &str, output_path: &str) -> Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let weechat_lines: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| parse_line(&line))
        .collect();

    let output_file = File::create(output_path)?;
    let mut writer = std::io::BufWriter::new(output_file);

    for line in weechat_lines {
        writeln!(writer, "{} {} <{}> {}", line.date, line.time, line.nick, line.msg)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_path> <output_path>", args[0]);
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    if let Err(err) = convert_log(input_path, output_path) {
        eprintln!("Error: {}", err);
    } else {
        println!("Conversion completed successfully.");
    }
}
