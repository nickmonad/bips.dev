use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::prelude::*;
mod bip;

lazy_static! {
    static ref BIP_NUMBER: Regex =
        Regex::new(r"^bips/bip-([0-9]+)\.mediawiki$").expect("error parsing regex");
}

fn main() -> io::Result<()> {
    let cmd = std::env::args().into_iter().skip(1).take(1).next();
    if let Some(cmd) = cmd {
        let stdin = io::stdin();
        let lines = stdin.lock().lines().collect::<io::Result<Vec<String>>>()?;
        let input: Vec<(u32, String)> = lines
            .iter()
            .filter_map(|path| {
                // parse the bip number from the path
                // and map into tuple (number, path)
                BIP_NUMBER
                    .captures(&path)?
                    .get(1)?
                    .as_str()
                    .parse::<u32>()
                    .map_or(None, |n| Some((n, path.clone())))
            })
            .collect();

        match &cmd[..] {
            "count" => return cmd_count(&input),
            "generate" => return cmd_generate(&input),
            "show" => return cmd_show(&input),
            _ => {
                println!("unknown command!");
                return Ok(());
            }
        }
    }

    println!("must provide command");
    Ok(())
}

fn cmd_count(input: &[(u32, String)]) -> io::Result<()> {
    let mut stats = bip::Stats::new();

    for (bip, path) in input {
        bip::count(*bip, path, &mut stats)?;
    }

    for (k, v) in stats.nodes {
        println!("{} => {:?}", k, v);
    }

    Ok(())
}

fn cmd_generate(input: &[(u32, String)]) -> io::Result<()> {
    for (bip, path) in input {
        bip::generate(*bip, path)?;
    }

    Ok(())
}

fn cmd_show(input: &[(u32, String)]) -> io::Result<()> {
    for (_, path) in input {
        bip::show(path)?;
    }

    Ok(())
}
