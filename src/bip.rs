use parse_wiki_text as wiki;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::process::Command;
use tera;
use tera::{Context, Tera};

/// Process a BIP. This creates a new content directory for Zola to serve,
/// containing an index.md file, prepended with parsed front matter metadata.
pub fn process(number: u32, path: &str) -> io::Result<()> {
    // read input bip.mediawiki
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    // parse mediawiki content
    let wikitext = wiki::Configuration::default().parse(&content);
    let pre = wikitext
        .nodes
        .iter()
        .find_map(|node| match node {
            wiki::Node::Preformatted { .. } => Some(node),
            _ => None,
        })
        .unwrap();

    let meta = Meta::from_node(path, &pre);
    let template = Template::new("templates/**/*");

    // Create new directory and file for the final metadata + markdown
    fs::create_dir_all(format!("web/content/{}", number))?;
    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("web/content/{}/index.md", number))?;

    // First, write the metadata
    output.write_all(template.render(meta)?.as_bytes())?;
    output.write_all(b"\n")?;

    // Next, call out to pandoc for mediawiki to markdown conversion
    let pandoc = Command::new("pandoc")
        .args(&[path, "-f", "mediawiki", "-t", "gfm"])
        .output()?;

    // Finally, append the markdown to the output
    io::copy(&mut &pandoc.stdout[..], &mut output).map(|_| ())
}

#[derive(Debug, Serialize)]
pub struct Meta {
    bip: u32,
    title: String,
    created: String,
    github: String,
    status: Vec<String>,
    authors: Vec<String>,
}

impl Meta {
    pub fn from_node(path: &str, node: &wiki::Node) -> Meta {
        let mut info = Self::default();

        if let wiki::Node::Preformatted { nodes, .. } = node {
            let mut nodes = nodes.iter().peekable();
            while let Some(node) = nodes.next() {
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
                        "Status" => info.status = vec![format!("\"{}\"", v)],
                        "Author" => {
                            let mut authors = vec![clean_author(v)];
                            loop {
                                if let Some(n) = nodes.peek() {
                                    if is_new_section(n) {
                                        break;
                                    }
                                }

                                if let Some(n) = nodes.next() {
                                    if let wiki::Node::Text { value, .. } = n {
                                        authors.push(clean_author(value))
                                    }
                                }
                            }

                            info.authors = authors;
                        }
                        _ => {}
                    }
                }
            }
        }

        // apply github link
        let base = path.split("/").nth(1).unwrap();
        info.github = format!("https://github.com/bitcoin/bips/blob/master/{}", base);

        info
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            bip: 0,
            title: String::from(""),
            created: String::from(""),
            github: String::from(""),
            status: vec![],
            authors: vec![],
        }
    }
}

pub struct Template {
    client: Tera,
}

impl Template {
    pub fn new(dir: &str) -> Self {
        let mut t = Tera::new(dir).unwrap();
        t.register_filter("dequote", Template::filter_dequote);

        Self { client: t }
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

    pub fn render(&self, bip: Meta) -> io::Result<String> {
        Ok(self
            .client
            .render("front-matter.toml", &Context::from_serialize(bip).unwrap())
            .unwrap())
    }
}

fn clean_author(author: &str) -> String {
    let split = author.trim().splitn(2, '<').collect::<Vec<&str>>();
    format!("\"{}\"", &split[0][..].trim())
}

fn is_new_section(node: &wiki::Node) -> bool {
    if let wiki::Node::Text { value, .. } = node {
        return value.trim().splitn(2, ':').collect::<Vec<&str>>().len() >= 2;
    }

    false
}
