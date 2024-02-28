use parse_wiki_text_2 as wiki;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;

struct RenderContext {
    file: File,
    pre: bool,
}

impl RenderContext {
    fn new(file: File) -> Self {
        Self { file, pre: false }
    }
}

fn render(
    ctx: &mut RenderContext,
    node: &wiki::Node,
    parent: Option<&wiki::Node>,
) -> io::Result<()> {
    match node {
        wiki::Node::Heading { level, nodes, .. } => render_heading(ctx, node, *level, nodes),
        wiki::Node::ParagraphBreak { .. } => render_paragraph_break(ctx),
        wiki::Node::Text { value, .. } => render_text(ctx, parent, value),
        wiki::Node::OrderedList { items, .. } => render_ordered_list(ctx, node, items),
        wiki::Node::UnorderedList { items, .. } => render_unordered_list(ctx, node, items),
        wiki::Node::Preformatted { nodes, .. } => render_preformatted(ctx, node, nodes),
        wiki::Node::StartTag { name, .. } => render_start_tag(ctx, name.as_ref()),
        wiki::Node::EndTag { name, .. } => render_end_tag(ctx, name.as_ref()),
        wiki::Node::ExternalLink { nodes, .. } => render_external_link(ctx, node, nodes),
        wiki::Node::Link { text, .. } => render_link(ctx, node, text),
        _ => Ok(()),
    }
}

fn render_link(ctx: &mut RenderContext, this: &wiki::Node, nodes: &[wiki::Node]) -> io::Result<()> {
    for node in nodes {
        render(ctx, node, Some(this))?;
    }

    Ok(())
}

fn render_external_link(
    ctx: &mut RenderContext,
    this: &wiki::Node,
    nodes: &[wiki::Node],
) -> io::Result<()> {
    for node in nodes {
        render(ctx, node, Some(this))?;
    }

    Ok(())
}

fn render_preformatted(
    ctx: &mut RenderContext,
    this: &wiki::Node,
    nodes: &[wiki::Node],
) -> io::Result<()> {
    // not in <pre> tag, but actual wiki::Node
    if !ctx.pre {
        write!(ctx.file, "\n\n```\n")?;
    }

    for node in nodes {
        render(ctx, node, Some(this))?;
    }

    if !ctx.pre {
        write!(ctx.file, "```\n\n")?;
    }

    Ok(())
}

fn render_start_tag(ctx: &mut RenderContext, name: &str) -> io::Result<()> {
    if name == "pre" {
        ctx.pre = true;
    }

    write!(ctx.file, "<{}>", name)?;
    Ok(())
}

fn render_end_tag(ctx: &mut RenderContext, name: &str) -> io::Result<()> {
    if name == "pre" {
        ctx.pre = false;
    }

    write!(ctx.file, "</{}>", name)?;
    Ok(())
}

fn render_heading(
    ctx: &mut RenderContext,
    this: &wiki::Node,
    level: u8,
    nodes: &[wiki::Node],
) -> io::Result<()> {
    write!(ctx.file, "\n\n<h{}>", level)?;
    for node in nodes {
        render(ctx, node, Some(this))?;
    }

    write!(ctx.file, "</h{}>\n\n", level)?;
    Ok(())
}

fn render_text(
    ctx: &mut RenderContext,
    parent: Option<&wiki::Node>,
    value: &str,
) -> io::Result<()> {
    // build link, if needed
    let out = if value.starts_with("http") {
        if let Some(wiki::Node::ExternalLink { .. }) = parent {
            if let Some((link, display)) = value.split_once(" ") {
                format!(
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                    link, display
                )
            } else {
                format!(
                    "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                    value, value
                )
            }
        } else {
            format!(
                "<a href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                value, value
            )
        }
    } else {
        // img pass through
        if value.starts_with("<img") {
            return write!(ctx.file, "{}", value);
        }

        format!("{}", value)
    };

    let out = if ctx.pre {
        format!(
            "<span>{}</span>\n",
            out.replace("\n", "")
                .replace("<", "&lt;")
                .replace(">", "&gt;")
        )
    } else {
        out
    };

    if let Some(parent) = parent {
        match parent {
            wiki::Node::Heading { .. } => write!(ctx.file, "{}", out),
            wiki::Node::OrderedList { .. } => write!(ctx.file, "{}", out),
            wiki::Node::UnorderedList { .. } => {
                if ctx.pre {
                    write!(ctx.file, "* {}", out)
                } else {
                    write!(ctx.file, "{}", out)
                }
            }
            wiki::Node::Preformatted { .. } => write!(ctx.file, "{}", out),
            wiki::Node::ExternalLink { .. } => write!(ctx.file, "{}", out),
            wiki::Node::Link { .. } => write!(ctx.file, "{}", out),
            _ => Ok(()),
        }
    } else {
        // no parent, root level
        write!(ctx.file, "{}", out.replace("\n", " "))
    }
}

fn render_paragraph_break(ctx: &mut RenderContext) -> io::Result<()> {
    write!(ctx.file, "\n\n")
}

fn render_ordered_list(
    ctx: &mut RenderContext,
    this: &wiki::Node,
    items: &[wiki::ListItem],
) -> io::Result<()> {
    if ctx.pre {
        // render children as-is
        for item in items {
            for node in &item.nodes {
                render(ctx, node, Some(this))?;
            }
        }

        return Ok(());
    }

    write!(ctx.file, "<ol>")?;

    for item in items {
        write!(ctx.file, "<li>")?;

        for node in &item.nodes {
            render(ctx, node, Some(this))?;
        }

        write!(ctx.file, "</li>")?;
    }

    write!(ctx.file, "</ol>")
}

fn render_unordered_list(
    ctx: &mut RenderContext,
    this: &wiki::Node,
    items: &[wiki::ListItem],
) -> io::Result<()> {
    if ctx.pre {
        // render children as-is
        for item in items {
            for node in &item.nodes {
                render(ctx, node, Some(this))?;
            }
        }

        return Ok(());
    }

    write!(ctx.file, "<ul>")?;

    for item in items {
        write!(ctx.file, "<li>")?;

        for node in &item.nodes {
            render(ctx, node, Some(this))?;
        }

        write!(ctx.file, "</li>")?;
    }

    write!(ctx.file, "</ul>")
}

fn render_front_matter(ctx: &mut RenderContext, node: Option<&wiki::Node>) -> io::Result<()> {
    let mut front = FrontMatter::default();

    if let Some(wiki::Node::Preformatted { nodes, .. }) = node {
        let mut nodes = nodes.iter().peekable();
        while let Some(node) = nodes.next() {
            if let wiki::Node::Text { value, .. } = node {
                let split = value.trim().splitn(2, ':').collect::<Vec<&str>>();
                if split.len() < 2 {
                    continue;
                }

                let (k, v) = (&split[0][..], split[1].trim());
                match k {
                    "BIP" => front.bip = v.parse::<u32>().unwrap(),
                    "Title" => front.title = v.to_string(),
                    "Created" => front.created = v.to_string(),
                    "Status" => front.status = vec![format!("{}", v)],
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

                        front.authors = authors;
                    }
                    _ => {}
                }
            }
        }
    }

    // apply github link
    front.github = format!(
        "https://github.com/bitcoin/bips/blob/master/bip-{:04}.mediawiki",
        front.bip
    );

    // yes, this format is wonky...
    // need to prevent leading whitespace from each line
    write!(
        ctx.file,
        r#"
+++
title = "{title}"
date = {created}
weight = {bip}

[taxonomies]
authors = {authors:?}
status = {status:?}

[extra]
bip = {bip}
status = {status:?}
github = "{github}"
note = "THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING"
+++

"#,
        title = front.title,
        created = front.created,
        bip = front.bip,
        authors = front.authors,
        status = front.status,
        github = front.github
    )?;

    Ok(())
}

/// Generate a BIP, converting from mediawiki to HTML.
/// This creates a new content directory for Zola to serve, containing an index.md file,
/// prepended with parsed front matter metadata.
pub fn generate(bip: u32, path: &str) -> io::Result<()> {
    // read input bip.mediawiki
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    // create output directory and file
    let dir = format!("web/content/{}", bip);
    let generated = format!("{}/index.md", dir);

    match fs::create_dir(dir) {
        Err(err) => {
            if err.kind() != io::ErrorKind::AlreadyExists {
                return Err(err);
            }
        }
        _ => {}
    }

    let file = File::create(generated)?;
    // render mediawiki content as html
    let wikitext = wiki::Configuration::default().parse(&content);
    let mut ctx = RenderContext::new(file);

    // NOTE: we assume the "front matter" <pre> tag is always first, so render that
    let nodes = wikitext.nodes.iter();
    render_front_matter(&mut ctx, nodes.clone().skip(1).next())?;

    // render the rest of the document...
    for node in nodes {
        render(&mut ctx, &node, None)?;
    }

    Ok(())
}

/// Recursively find all wiki::Node elements in the given bip, by file path
/// results in a map of wiki::Node "name" to the total count,
/// and a set of bips that node type is found in
pub fn count(bip: u32, path: &str, stats: &mut Stats) -> io::Result<()> {
    // read input bip.mediawiki
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    // parse mediawiki content
    let wikitext = wiki::Configuration::default().parse(&content);
    for node in wikitext.nodes {
        record_node(bip, &node, stats);
    }

    Ok(())
}

/// Display a "pretty-printed" version of the wikitext.
pub fn show(path: &str) -> io::Result<()> {
    // read input bip.mediawiki
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    // parse mediawiki content
    let wikitext = wiki::Configuration::default().parse(&content);
    for node in wikitext.nodes {
        println!("{:#?}", node);
    }

    Ok(())
}

pub struct Stats {
    pub nodes: HashMap<String, (u32, HashSet<u32>)>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn record(&mut self, bip: u32, name: &str) {
        if let Some((count, bips)) = self.nodes.get_mut(name) {
            bips.insert(bip);
            *count = *count + 1;
        } else {
            let mut set = HashSet::new();
            set.insert(bip);

            self.nodes.insert(name.into(), (1, set));
        }
    }
}

fn record_node(bip: u32, node: &wiki::Node, s: &mut Stats) {
    match node {
        wiki::Node::Bold { .. } => s.record(bip, "Bold"),
        wiki::Node::BoldItalic { .. } => s.record(bip, "BoldItalic"),
        wiki::Node::Category { ordinal, .. } => {
            s.record(bip, "Category");

            for node in ordinal {
                record_node(bip, &node, s);
            }
        }
        wiki::Node::CharacterEntity { .. } => s.record(bip, "CharacterEntity"),
        wiki::Node::Comment { .. } => s.record(bip, "Comment"),
        wiki::Node::DefinitionList { .. } => s.record(bip, "DefinitionList"),
        wiki::Node::EndTag { .. } => s.record(bip, "EndTag"),
        wiki::Node::ExternalLink { nodes, .. } => {
            s.record(bip, "ExternalLink");

            for node in nodes {
                record_node(bip, &node, s);
            }
        }
        wiki::Node::Heading { nodes, .. } => {
            s.record(bip, "Heading");

            for node in nodes {
                record_node(bip, &node, s);
            }
        }
        wiki::Node::HorizontalDivider { .. } => s.record(bip, "HorizontalDivider"),
        wiki::Node::Image { text, .. } => {
            s.record(bip, "Image");

            for node in text {
                record_node(bip, &node, s);
            }
        }
        wiki::Node::Italic { .. } => s.record(bip, "Italic"),
        wiki::Node::Link { text, .. } => {
            s.record(bip, "Link");

            for node in text {
                record_node(bip, &node, s);
            }
        }
        wiki::Node::MagicWord { .. } => s.record(bip, "MagicWord"),
        wiki::Node::OrderedList { items, .. } => {
            s.record(bip, "OrderedList");

            for item in items {
                for node in &item.nodes {
                    record_node(bip, node, s);
                }
            }
        }
        wiki::Node::ParagraphBreak { .. } => s.record(bip, "ParagraphBreak"),
        wiki::Node::Parameter { default, name, .. } => {
            s.record(bip, "Parameter");

            if let Some(nodes) = default {
                for node in nodes {
                    record_node(bip, node, s);
                }
            }

            for node in name {
                record_node(bip, node, s);
            }
        }
        wiki::Node::Preformatted { nodes, .. } => {
            s.record(bip, "Preformatted");

            for node in nodes {
                record_node(bip, node, s);
            }
        }
        wiki::Node::Redirect { .. } => s.record(bip, "Redirect"),
        wiki::Node::StartTag { .. } => s.record(bip, "StartTag"),
        wiki::Node::Table {
            attributes,
            captions,
            rows,
            ..
        } => {
            s.record(bip, "Table");

            // table.attributes
            for node in attributes {
                record_node(bip, node, s);
            }

            // table.captions
            for caption in captions {
                if let Some(attributes) = &caption.attributes {
                    for node in attributes {
                        record_node(bip, node, s);
                    }
                }

                for node in &caption.content {
                    record_node(bip, node, s);
                }
            }

            // table.rows
            for row in rows {
                for node in &row.attributes {
                    record_node(bip, node, s);
                }

                for cell in &row.cells {
                    if let Some(attributes) = &cell.attributes {
                        for node in attributes {
                            record_node(bip, node, s);
                        }
                    }

                    for node in &cell.content {
                        record_node(bip, node, s);
                    }
                }
            }
        }
        wiki::Node::Tag { nodes, .. } => {
            s.record(bip, "Tag");

            for node in nodes {
                record_node(bip, node, s);
            }
        }
        wiki::Node::Template {
            name, parameters, ..
        } => {
            s.record(bip, "Template");

            for node in name {
                record_node(bip, node, s);
            }

            for p in parameters {
                if let Some(name) = &p.name {
                    for node in name {
                        record_node(bip, node, s);
                    }
                }

                for node in &p.value {
                    record_node(bip, node, s);
                }
            }
        }
        wiki::Node::Text { .. } => s.record(bip, "Text"),
        wiki::Node::UnorderedList { items, .. } => {
            s.record(bip, "UnorderedList");

            for item in items {
                for node in &item.nodes {
                    record_node(bip, node, s);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FrontMatter {
    bip: u32,
    title: String,
    created: String,
    github: String,
    status: Vec<String>,
    authors: Vec<String>,
}

impl Default for FrontMatter {
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

fn clean_author(author: &str) -> String {
    let split = author.trim().splitn(2, '<').collect::<Vec<&str>>();
    format!("{}", &split[0][..].trim())
}

fn is_new_section(node: &wiki::Node) -> bool {
    if let wiki::Node::Text { value, .. } = node {
        return value.trim().splitn(2, ':').collect::<Vec<&str>>().len() >= 2;
    }

    false
}
