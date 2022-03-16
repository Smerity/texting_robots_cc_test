use std::sync::atomic::{AtomicU64, Ordering};
use std::{env, fs};

use bstr::{BStr, ByteSlice};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::bytes::Regex;
use warc::WarcReader;

use texting_robots::Robot;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory with robots.warc.gz files>", args[0]);
        return Ok(());
    }
    let dirs = &args[1..];

    let warc_splitter = Regex::new("\r\n\r\n").unwrap();

    let mut fns = vec![];

    for dir in dirs {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(p) = path.to_str() {
                if p.ends_with(".warc.gz") {
                    fns.push(path);
                }
            }
        }
    }

    let bar = ProgressBar::new(fns.len() as u64);
    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} / {eta}] {wide_bar} {msg:>9} robots.txt responses from {pos}/{len} files",
    ));

    let total_robots: AtomicU64 = AtomicU64::new(0);
    fns.par_iter().for_each(|filename| {
        let wr = match WarcReader::from_path_gzip(filename) {
            Ok(wr) => wr,
            Err(_) => return,
        };
        for record in wr.iter_records() {
            let record = record.unwrap();
            // WARC records track warcinfo / request / response / ...
            if record.warc_type().to_string() != "response" {
                continue;
            }

            let payload: &BStr = record.body().as_bstr();

            // Filter for: "HTTP/1.1 200 OK", "GET /robots.txt HTTP/1.1", "HTTP/1.0 200 Found", ...
            // This will be imperfect but we don't need every possible response
            let filters: Vec<&str> = vec!["get", "200"];
            let line_end = payload.find(b"\n");
            if line_end.is_none() {
                continue;
            }
            let line = &payload[..line_end.unwrap()];
            if !filters
                .iter()
                .any(|x| line.to_ascii_lowercase().find(x).is_some())
            {
                continue;
            }

            let fields: Vec<&[u8]> = warc_splitter.splitn(payload, 2).collect();
            if fields.len() != 2 {
                println!(
                    "ERROR: {} - {:?}",
                    fields.len(),
                    String::from_utf8_lossy(payload)
                );
                continue;
            }
            let (_, mut body) = (fields[0], fields[1]);
            // Google only processes the first 500 kibibytes of the response
            if body.len() > 500_000 {
                body = &body[..500_000];
            }
            //println!("=-=-=\n{}\n", String::from_utf8_lossy(body));

            match Robot::new("*", body) {
                Ok(r) => r.allowed("/"),
                Err(_) => continue,
            };

            total_robots.fetch_add(1, Ordering::SeqCst);
        }

        bar.inc(1);
        bar.set_message(format!("{}", total_robots.load(Ordering::SeqCst)));
    });

    Ok(())
}
