use parse_wiki_text as wiki;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use tera;
use tera::{Context, Tera};

struct Templater {
    client: Tera,
}

impl Templater {
    fn new(dir: &str) -> Templater {
        let mut t = Tera::new(dir).unwrap();
        t.register_filter("dequote", Templater::filter_dequote);

        Templater { client: t }
    }

    fn filter_dequote(
        value: &tera::Value,
        _args: &HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        match value {
            tera::Value::String(s) => Ok(tera::Value::String(
                s.as_str().replace("\'", "").replace("\"", "").to_string(),
            )),
            _ => Err(tera::Error::msg("must be applied to string values")),
        }
    }

    fn render(&self, bip: BipInfo) -> io::Result<String> {
        Ok(self
            .client
            .render("front-matter.toml", &Context::from_serialize(bip).unwrap())
            .unwrap())
    }
}

#[derive(Debug, Serialize)]
struct BipInfo {
    bip: u32,
    title: String,
    created: String,
    status: String,
}

impl BipInfo {
    fn from_node(node: &wiki::Node) -> BipInfo {
        let mut info = BipInfo::default();
        if let wiki::Node::Preformatted { nodes, .. } = node {
            for node in nodes {
                if let wiki::Node::Text { value, .. } = node {
                    let split = value.trim().splitn(2, ':').collect::<Vec<&str>>();
                    if split.len() < 2 {
                        continue;
                    }

                    let (k, v) = (&split[0][..], split[1].trim());
                    match k {
                        "BIP" => info.bip = v.parse::<u32>().unwrap(),
                        "Title" => info.title = v.to_string(),
                        "Created" => info.created = v.to_string(),
                        "Status" => info.status = v.to_string(),
                        _ => {}
                    }
                }
            }
        }

        info
    }
}

impl Default for BipInfo {
    fn default() -> BipInfo {
        BipInfo {
            bip: 0,
            title: String::from(""),
            created: String::from(""),
            status: String::from(""),
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // create tera instance
    let templater = Templater::new("templates/**/*");

    // read input bip.mediawiki
    let mut input = File::open(filename)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    // parse mediawiki content
    let wikitext = wiki::Configuration::default().parse(&content);
    let preformatted = wikitext
        .nodes
        .iter()
        .find_map(|node| match node {
            wiki::Node::Preformatted { .. } => Some(node),
            _ => None,
        })
        .unwrap();

    let info = BipInfo::from_node(&preformatted);
    println!("{}", templater.render(info).unwrap());

    Ok(())
}
