use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    sync::LazyLock,
};

use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use ureq::Agent;

static SOURCES: &[&str] = &["https://t.me/s/warpplus", "https://t.me/s/warppluscn"];

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<code>([A-Za-z0-9-]+)<\/code>").unwrap());

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let agent = Agent::new();
    let mut keys = HashSet::new();

    for &url in SOURCES {
        println!("Currently searching in {}...", url);
        let body = agent.get(url).call()?.into_string()?;

        for regex_match in PATTERN.find_iter(&body) {
            let key = regex_match.as_str();
            let keylen = key.len();
            keys.insert(String::from(&key[6..(keylen - 7)]));
        }
    }

    let mut full = BufWriter::new(File::create("./full.txt")?);
    let mut lite = BufWriter::new(File::create("./lite.txt")?);

    let keys = keys.into_iter().collect::<Box<[String]>>();
    let mut written = HashSet::new();
    if keys.len() > 0 {
        println!("Selecting keys...");
        let mut cnt = 0u8;
        while cnt < 150 {
            if let Some(key) = keys.choose(&mut rng) {
                if written.insert(key) {
                    writeln!(&mut full, "{}", key)?;
                    cnt += 1;
                }
            }
        }

        cnt = 0;
        written.clear();
        while cnt < 75 {
            if let Some(key) = keys.choose(&mut rng) {
                if written.insert(key) {
                    writeln!(&mut lite, "{}", key)?;
                    cnt += 1;
                }
            }
        }
    }

    Ok(())
}
