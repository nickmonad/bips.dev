use lazy_static::lazy_static;
use regex::Regex;
use std::convert::Infallible;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
enum FileFormat {
    Markdown,
    Mediawiki,
}

/// Generate a BIP, converting from mediawiki to HTML.
/// This creates a new content directory for Zola to serve, containing an index.md file,
/// prepended with parsed front matter metadata.
pub fn generate<P: AsRef<Path>>(bip: u32, path: P) -> io::Result<()> {
    // create output directory and file
    let dir = format!("web/content/{}", bip);
    let markdown = format!("{}/index.md", dir);

    if let Err(err) = fs::create_dir(dir) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(err);
        }
    }

    // collect front matter
    let mut pre = false;
    let mut pre_lines = Vec::new();
    let lines = read_lines(&path)?;
    for line in lines.flatten() {
        if !pre {
            if line.trim() == "```" || line.trim() == "<pre>" {
                pre = true;
                continue;
            }
        }

        if pre {
            if line.trim() == "```" || line.trim() == "</pre>" {
                // end of front matter, bail
                break;
            }
        }

        pre_lines.push(line);
    }

    let format = if let Some(ext) = path.as_ref().extension() {
        if ext == "md" {
            FileFormat::Markdown
        } else if ext == "mediawiki" {
            FileFormat::Mediawiki
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "unknown file type"));
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "no extension"));
    };

    let mut output = File::create(&markdown)?;
    let fm = front_matter(&pre_lines, format);
    write!(output, "{}\n\n", fm)?;

    // markdown files do not need rendered, write as is
    if format == FileFormat::Markdown {
        let lines = read_lines(&path)?;
        for line in lines.flatten() {
            write!(output, "{}\n", line)?;
        }

        return Ok(());
    }

    // mediawiki
    // start rendering
    let mut context = RenderContext {
        tag: RenderTag::None,
        refs: Vec::default(),
    };

    // render the rest of the document, line by line
    let lines = read_lines(&path)?;
    for line in lines.flatten() {
        if let Some(rendered) = context.render(&line) {
            write!(output, "{}", rendered)?;
        }
    }

    Ok(())
}

struct RenderContext {
    tag: RenderTag,

    #[allow(dead_code)]
    refs: Vec<String>,
}

enum RenderTag {
    None,
    PreTag,
    PreFormatted,
    UnorderedList,
    OrderedList,
    Table(TableContext),
}

#[derive(Clone, Default)]
struct TableContext {
    header: bool,
    row: Vec<String>,
}

impl RenderContext {
    fn render(&mut self, line: &str) -> Option<String> {
        match self.tag {
            RenderTag::None => {
                // preformatted, tag
                // NOTE: see source for BIP 10 for a case where "</pre>" actually _starts_ preformatted
                if line == "<pre>" || line == "</pre>" || line.starts_with("<source") {
                    self.tag = RenderTag::PreTag;
                    return Some("```\n".into());
                }

                // preformatted, space
                if line.starts_with(" ") {
                    self.tag = RenderTag::PreFormatted;
                    return Some(format!("```\n{}", self.render(line).unwrap_or("".into())));
                }

                // empty line, write new line
                if line == "" {
                    return Some("\n".into());
                }

                // heading
                if line.starts_with("=") {
                    return Some(format!("{}\n\n", render_heading(line)));
                }

                // ordered list
                if line.starts_with("#") {
                    self.tag = RenderTag::OrderedList;
                    return self.render(line);
                }

                // unordered list
                if line.starts_with("*") {
                    self.tag = RenderTag::UnorderedList;
                    return self.render(line);
                }

                // table
                if line.starts_with("{|") {
                    self.tag = RenderTag::Table(TableContext::default());

                    // edge case
                    // sometimes authors don't include a new line between
                    // the last line and the start of the table
                    return Some("\n".into());
                }

                // references
                // only check for "beginning" of tag, not sure if there's a consistency in ending
                if line.starts_with("<references") {
                    let mut empty = Vec::default();

                    return Some(
                        self.refs
                            .iter()
                            .enumerate()
                            .map(|(i, reference)| {
                                format!(
                                    "{}. [^](#{}) {}",
                                    i + 1,
                                    ref_id(i + 1),
                                    render_line(reference, &mut empty).unwrap()
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("\n"),
                    );
                }

                // definition list? or indentation? ignore....
                if line.starts_with(":") {
                    return self.render(line.trim_start_matches(":").trim_start_matches(" "));
                }

                // no marker at beginning, render the line
                Some(format!("{}\n", render_line(line, &mut self.refs).unwrap()))
            }
            RenderTag::PreFormatted => {
                // end preformatted
                if line == "" || line.starts_with(char::is_alphanumeric) {
                    // close out
                    self.tag = RenderTag::None;
                    return Some(format!("```\n{}\n", self.render(line).unwrap_or("".into())));
                }

                // write as is, no modification
                Some(format!("{}\n", line))
            }
            RenderTag::PreTag => {
                // end preformatted
                if line == "</pre>" || line == "</source>" {
                    // close out
                    self.tag = RenderTag::None;
                    return Some("```\n".into());
                }

                // write as is, no modification
                Some(format!("{}\n", line))
            }
            RenderTag::OrderedList => {
                // end ordered list
                if !line.starts_with("#") {
                    self.tag = RenderTag::None;
                    return Some(format!("\n{}", self.render(line).unwrap_or("".into())));
                }

                // edge case
                // BIP 36
                if line.starts_with("#*") {
                    return Some(format!(
                        "{}* {}\n",
                        " ".repeat(4),
                        render_line(&line["#*".len()..line.len()], &mut self.refs).unwrap()
                    ));
                }

                // check for the list level (*, **, etc)
                let level: usize = line
                    .chars()
                    .map_while(|c| if c == '#' { Some(1) } else { None })
                    .sum();

                let indent = " ".repeat(if level > 1 {
                    usize::pow(2, level as u32)
                } else {
                    0
                });

                let trimmed = line.trim_start_matches("#");
                Some(format!(
                    "{}1. {}\n",
                    indent,
                    render_line(trimmed, &mut self.refs).unwrap()
                ))
            }
            RenderTag::UnorderedList => {
                if !line.starts_with("*") {
                    self.tag = RenderTag::None;
                    return Some(format!("\n{}", self.render(line).unwrap_or("".into())));
                }

                // edge case, BIP 107
                let clean = line.replace("#", " ");

                // check for the list level (*, **, etc)
                let level: usize = clean
                    .chars()
                    .map_while(|c| if c == '*' { Some(1) } else { None })
                    .sum();

                let indent = " ".repeat(if level > 1 {
                    usize::pow(2, level as u32)
                } else {
                    0
                });

                // sometimes, an unordered list is written without any space after the '*' in the
                // source document. The zola markdown to html rendering doesn't like that, so we have
                // to "push out" all unordered lists by one space, so they render correctly
                Some(format!(
                    "{}* {}\n",
                    indent,
                    render_line(&clean[level..line.len()], &mut self.refs).unwrap()
                ))
            }
            RenderTag::Table(ref mut table) => {
                if line.starts_with("|}") {
                    // write any remaining row
                    let buffer = format!(
                        "|{}|\n\n",
                        table
                            .row
                            .iter()
                            .map(|r| render_line(r, &mut self.refs).unwrap())
                            .collect::<Vec<String>>()
                            .join("|")
                    );

                    self.tag = RenderTag::None;
                    return Some(buffer);
                }

                if line.contains("colspan") {
                    // ignore for now
                    return None;
                }

                // table header
                // convert to markdown format
                if line.starts_with("!") {
                    let headers: Vec<&str> = line["!".len()..line.len()]
                        .split("!!")
                        .map(|s| {
                            // edge case, see BIP 136
                            // remove style from header
                            let t = s.trim();
                            if t.starts_with("style=") {
                                if let Some((_, keep)) = t.split_once("|") {
                                    return keep.trim();
                                }
                            }

                            t
                        })
                        .collect();

                    if headers.len() > 1 {
                        // multiple column values on a single line
                        // collect and defer rendering
                        table.row = headers.iter().map(|s| s.to_string()).collect();
                        return None;
                    }

                    // columns are spread across multiple lines
                    // collect and defer rendering
                    table.row.push(headers[0].to_string());
                    return None;
                }

                // row separation
                // write rows in buffer
                // MUST be before check below for .starts_with("|")
                if line.starts_with("|-") {
                    if table.row.is_empty() {
                        // metadata before getting to table data, nothing to do
                        return None;
                    }

                    let mut buffer = String::new();

                    // write rows
                    buffer.push_str(&format!(
                        "|{}|\n",
                        table
                            .row
                            .iter()
                            .map(|r| render_line(r, &mut self.refs).unwrap())
                            .collect::<Vec<String>>()
                            .join("|")
                    ));

                    if !table.header {
                        // write out "bottom" of header
                        buffer.push_str(&format!("|{}\n", "-|".repeat(table.row.len())));
                        table.header = true;
                    }

                    // reset rows
                    table.row = Vec::new();
                    return Some(buffer);
                }

                // table row
                if line.starts_with("|") {
                    let columns: Vec<&str> = line["|".len()..line.len()]
                        .split("||")
                        .map(|s| {
                            // edge case, see BIP 98
                            // remove any leading metadata in the cell
                            // "| scope="col"| A" ---> "A"
                            let t = s.trim();
                            if t.starts_with("scope=\"col\"|") || t.starts_with("scope=\"row\"|") {
                                let marker = "scope=\"___\"";
                                if marker.len() + 1 < t.len() {
                                    t[(marker.len() + 1)..t.len()].trim()
                                } else {
                                    ""
                                }
                            } else {
                                t
                            }
                        })
                        .map(|s| {
                            // edge case, see BIP 136
                            // remove any style metadata in the cell
                            // | style="..." | A ---> A
                            let t = s.trim();
                            if t.starts_with("style=") {
                                if let Some((_, keep)) = t.split_once("|") {
                                    return keep.trim();
                                }
                            }

                            t
                        })
                        .map(|s| {
                            // edge case, see BIP 136
                            // remove "rowspan"
                            // | rowspan="..." | A ---> A
                            let t = s.trim();
                            if t.starts_with("rowspan=") {
                                if let Some((_, keep)) = t.split_once("|") {
                                    return keep.trim();
                                }
                            }

                            t
                        })
                        .collect();

                    if columns.len() > 1 {
                        // multiple column values on a single line
                        // collect and defer rendering
                        table.row = columns.iter().map(|s| s.to_string()).collect();
                    } else {
                        // columns are spread across multiple lines
                        // collect and defer rendering
                        table.row.push(columns[0].to_string());
                    }
                }

                None
            }
        }
    }
}

fn render_heading(line: &str) -> String {
    let heading = line.replace("=", "");
    let level: u8 = line
        .chars()
        .map_while(|c| if c == '=' { Some(1) } else { None })
        .sum();

    format!("<h{}>{}</h{}>", level, heading, level)
}

fn render_line(line: &str, refs: &mut Vec<String>) -> Result<String, Infallible> {
    replace_nop(line.into())
        .and_then(replace_internal_links)
        .and_then(replace_references(refs))
        .and_then(replace_external_links)
        .and_then(replace_bold)
        .and_then(replace_italic)
        .and_then(replace_code_tag)
}

fn replace_nop(line: String) -> Result<String, Infallible> {
    Ok(line)
}

fn replace_code_tag(line: String) -> Result<String, Infallible> {
    Ok(line.replace("<code>", "`").replace("</code>", "`"))
}

fn replace_bold(line: String) -> Result<String, Infallible> {
    let mut buffer = String::new();
    let bold: Vec<(usize, &str)> = line.match_indices("'''").collect();

    let mut ptr = 0;
    for (i, _) in bold {
        buffer.push_str(&line[ptr..i]);
        buffer.push_str("**"); // FIX THIS!!!

        ptr = i + "'''".len();
    }

    buffer.push_str(&line[ptr..line.len()]);
    Ok(buffer)
}

fn replace_italic(line: String) -> Result<String, Infallible> {
    let mut buffer = String::new();
    let italic: Vec<(usize, &str)> = line.match_indices("''").collect();

    let mut ptr = 0;
    for (i, _) in italic {
        buffer.push_str(&line[ptr..i]);
        buffer.push_str("_");

        ptr = i + "''".len();
    }

    buffer.push_str(&line[ptr..line.len()]);
    Ok(buffer)
}

fn replace_references(
    refs: &'_ mut Vec<String>,
) -> impl FnMut(String) -> Result<String, Infallible> + '_ {
    move |line: String| -> Result<String, Infallible> {
        let mut buffer = String::new();
        let tags: Vec<((usize, &str), (usize, &str))> = line
            .match_indices("<ref>")
            .zip(line.match_indices("</ref>"))
            .collect();

        let mut ptr = 0;
        for ((start, _), (end, _)) in tags {
            if start > end {
                // edge case, BIP 331 split ref across line, fix later
                continue;
            }

            let ref_n = refs.len() + 1;

            buffer.push_str(&line[ptr..start]);
            buffer.push_str(&format!(
                "<sup id=\"{}\"><a href=\"#{}\">{}</a></sup>",
                ref_id(ref_n),
                ref_id(ref_n),
                ref_n
            ));

            ptr = end + "</ref>".len();

            // save reference
            refs.push(line[(start + "<ref>".len())..end].into());
        }

        buffer.push_str(&line[ptr..line.len()]);
        Ok(buffer)
    }
}

// internal links "[[ ... ]]"
fn replace_internal_links(line: String) -> Result<String, Infallible> {
    let mut buffer = String::new();
    let internal: Vec<((usize, &str), (usize, &str))> = line
        .match_indices("[[")
        .zip(line.match_indices("]]"))
        .collect();

    // edge case
    // BIP 140, author split link across multiple lines... need to fix
    if internal.len() == 0 {
        return Ok(line);
    }

    let mut ptr: usize = 0;
    for ((start, _), (end, _)) in internal {
        let content = &line[(start + "[[".len())..end]; // after [[, and up to ]]
        let components: Vec<&str> = content.split("|").collect();

        let tag = if components[0].starts_with("File:") {
            // is file link? assume image
            let image = components[0].split(":").collect::<Vec<&str>>()[1];
            format!("<img src=\"{}\" />", image)
        } else {
            // actual link
            let (link, name): (String, String) = if let Some(bip) = bip_link(components[0]) {
                if components.len() == 2 {
                    (format!("/{}", bip), components[1].into())
                } else {
                    (format!("/{}", bip), components[0].into())
                }
            } else {
                if bip_file(components[0]) {
                    let github = "https://github.com/bitcoin/bips/blob/master";
                    if components.len() == 2 {
                        (
                            format!("{}/{}", github, components[0]),
                            format!("{}", components[1]),
                        )
                    } else {
                        (
                            format!("{}/{}", github, components[0]),
                            format!("{}", components[0]),
                        )
                    }
                } else {
                    if components.len() == 2 {
                        (format!("{}", components[0]), format!("{}", components[1]))
                    } else {
                        (format!("{}", components[0]), format!("{}", components[0]))
                    }
                }
            };

            format!("<a href=\"{}\" target=\"_blank\">{}</a>", link, name,)
        };

        buffer.push_str(&line[ptr..start]);
        buffer.push_str(&tag);

        ptr = end + "]]".len();
    }

    buffer.push_str(&line[ptr..line.len()]);
    Ok(buffer)
}

#[rustfmt::skip]
lazy_static! {
    static ref BIP_FILE: Regex = Regex::new(r"^bip-([0-9]+)/.*").expect("error parsing regex");
    static ref BIP_NUMBER: Regex = Regex::new(r"bip-([0-9]+)\.mediawiki").expect("error parsing regex");
}

fn bip_link(link: &str) -> Option<u32> {
    BIP_NUMBER
        .captures(link)?
        .get(1)?
        .as_str()
        .parse::<u32>()
        .ok()
}

fn bip_file(link: &str) -> bool {
    BIP_FILE.is_match(link)
}

fn ref_id(n: usize) -> String {
    format!("cite_ref_{}", n)
}

// external links => [ ... ]
fn replace_external_links(line: String) -> Result<String, Infallible> {
    let mut buffer = String::new();
    let mut external: Vec<(usize, usize)> = vec![];
    let mut open: u32 = 0;
    let mut start: Option<usize> = None;
    for (i, c) in line.char_indices() {
        if c == '[' {
            if open == 0 {
                start = Some(i);
            }

            open += 1;
            continue;
        }

        if c == ']' {
            if open > 0 {
                open -= 1;
            }

            if open == 0 {
                let end = i;
                if let Some(start) = start {
                    external.push((start, end));
                }
            }

            continue;
        }
    }

    let mut ptr: usize = 0;
    for (start, end) in external {
        let content = &line[(start + 1)..end]; // after [, and up to ]
        let components = content.split_once(" ");
        let anchor = if let Some((link, name)) = components {
            if let Some(bip) = bip_link(link) {
                a_href(&format!("/{}", bip), name)
            } else {
                a_href(link, name)
            }
        } else {
            // not a link! write the whole thing
            format!("{}", &line[start..end + "]".len()])
        };

        buffer.push_str(&line[ptr..start]);
        buffer.push_str(&anchor);

        ptr = end + "]".len();
    }

    buffer.push_str(&line[ptr..line.len()]);
    Ok(buffer)
}

#[derive(Debug)]
pub struct FrontMatter {
    bip: u32,
    title: String,
    assigned: String,
    github: String,
    status: Vec<String>,
    authors: Vec<String>,
}

impl Default for FrontMatter {
    fn default() -> Self {
        Self {
            bip: 0,
            title: String::from(""),
            assigned: String::from(""),
            github: String::from(""),
            status: vec![],
            authors: vec![],
        }
    }
}

fn front_matter<T: AsRef<str>>(pre: &[T], format: FileFormat) -> String {
    let mut fm = FrontMatter::default();
    let mut lines = pre.iter().peekable();

    while let Some(line) = lines.next() {
        let split = line.as_ref().trim().splitn(2, ':').collect::<Vec<&str>>();
        if split.len() < 2 {
            continue;
        }

        let (key, value) = (&split[0][..], split[1].trim());
        match key {
            "BIP" => fm.bip = value.parse::<u32>().unwrap(),
            "Title" => fm.title = value.to_string(),
            "Assigned" | "Created" => fm.assigned = value.to_string(),
            "Status" => fm.status = vec![format!("{}", value)],
            "Author" | "Authors" => {
                let mut authors = vec![clean_author(value)];

                loop {
                    if let Some(line) = lines.peek() {
                        if is_new_section_s(line.as_ref()) {
                            break;
                        }
                    }

                    if let Some(line) = lines.next() {
                        authors.push(clean_author(line.as_ref()));
                    }
                }

                fm.authors = authors;
            }
            // skip all other keys
            _ => {}
        }
    }

    // apply github link
    fm.github = format!(
        "https://github.com/bitcoin/bips/blob/master/bip-{:04}.{}",
        fm.bip,
        match format {
            FileFormat::Markdown => "md",
            FileFormat::Mediawiki => "mediawiki",
        },
    );

    // yes, this format is wonky...
    // need to prevent leading whitespace from each line
    format!(
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
+++"#,
        title = fm.title.replace("\"", "\\\""),
        created = fm.assigned,
        bip = fm.bip,
        authors = fm.authors,
        status = fm.status,
        github = fm.github
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn clean_author(author: &str) -> String {
    let split = author.trim().splitn(2, '<').collect::<Vec<&str>>();
    format!("{}", &split[0][..].trim())
}

fn is_new_section_s(s: &str) -> bool {
    return s.trim().splitn(2, ':').collect::<Vec<&str>>().len() >= 2;
}

fn a_href(target: &str, link: &str) -> String {
    format!("<a href=\"{target}\" target=\"_blank\">{link}</a>")
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(input: Vec<String>) -> Vec<String> {
        let mut context = RenderContext {
            tag: RenderTag::None,
            refs: Vec::default(),
        };

        input
            .iter()
            .filter_map(|line| context.render(line))
            .collect()
    }

    fn lines(from: &str) -> Vec<String> {
        from.lines()
            .map(|s| s.trim_start())
            .map(|s| s.to_string())
            .collect()
    }

    // front matter

    #[test]
    fn front_matter_one_author() {
        let pre = [
            "BIP: 420",
            "Layer: Applications",
            "Title: Test BIP",
            "Author: Nick Miller <test@test.com>",
            "Comments-Summary: No comments yet.",
            "Comments-URI: https://notused.com",
            "Status: Draft",
            "Type: Informational",
            "Assigned: 2025-01-01",
        ];

        let expected = [
            "",
            "+++",
            "title = \"Test BIP\"",
            "date = 2025-01-01",
            "weight = 420",
            "",
            "[taxonomies]",
            "authors = [\"Nick Miller\"]",
            "status = [\"Draft\"]",
            "",
            "[extra]",
            "bip = 420",
            "status = [\"Draft\"]",
            "github = \"https://github.com/bitcoin/bips/blob/master/bip-0420.mediawiki\"",
            "note = \"THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING\"",
            "+++",
        ];

        assert_eq!(
            front_matter(&pre, FileFormat::Mediawiki)
                .split("\n")
                .collect::<Vec<_>>(),
            expected,
        );
    }

    #[test]
    fn front_matter_multiple_authors() {
        let pre = [
            "BIP: 420",
            "Layer: Applications",
            "Title: Test BIP",
            "Author: Nick Miller <test@test.com>",
            "        Satoshi Nakamoto <lol@test.com>",
            "Comments-Summary: No comments yet.",
            "Comments-URI: https://notused.com",
            "Status: Draft",
            "Type: Informational",
            "Assigned: 2025-01-01",
        ];

        let expected = [
            "",
            "+++",
            "title = \"Test BIP\"",
            "date = 2025-01-01",
            "weight = 420",
            "",
            "[taxonomies]",
            "authors = [\"Nick Miller\", \"Satoshi Nakamoto\"]",
            "status = [\"Draft\"]",
            "",
            "[extra]",
            "bip = 420",
            "status = [\"Draft\"]",
            "github = \"https://github.com/bitcoin/bips/blob/master/bip-0420.mediawiki\"",
            "note = \"THIS FILE IS AUTOMATICALLY GENERATED - NOT MEANT FOR EDITING\"",
            "+++",
        ];

        assert_eq!(
            front_matter(&pre, FileFormat::Mediawiki)
                .split("\n")
                .collect::<Vec<_>>(),
            expected,
        );
    }

    // render_line

    #[test]
    fn render_line_no_change() {
        assert_eq!(
            render_line("this line will stay the same", &mut Vec::default()).unwrap(),
            "this line will stay the same".to_string(),
        )
    }

    #[test]
    fn render_line_bold_and_italic() {
        assert_eq!(
            render_line(
                "this is some '''bold''' and ''italic'' text",
                &mut Vec::default()
            )
            .unwrap(),
            "this is some **bold** and _italic_ text".to_string(),
        )
    }

    // bold

    #[test]
    fn bold() {
        assert_eq!(
            render_line("this is some '''bold''' text", &mut Vec::default()).unwrap(),
            "this is some **bold** text".to_string(),
        )
    }

    // italic

    #[test]
    fn italic() {
        assert_eq!(
            render_line("this is some ''italic'' text", &mut Vec::default()).unwrap(),
            "this is some _italic_ text".to_string(),
        )
    }

    // references

    #[test]
    fn reference() {
        assert_eq!(
            render_line("test<ref>[foo bar]</ref>", &mut Vec::default()).unwrap(),
            "test<sup id=\"cite_ref_1\"><a href=\"#cite_ref_1\">1</a></sup>".to_string(),
        )
    }

    #[test]
    fn reference_surrounded() {
        assert_eq!(
            render_line("test<ref>[foo bar]</ref> more", &mut Vec::default()).unwrap(),
            "test<sup id=\"cite_ref_1\"><a href=\"#cite_ref_1\">1</a></sup> more".to_string(),
        )
    }

    #[test]
    fn reference_multiple() {
        let mut refs: Vec<String> = Vec::new();

        assert_eq!(
            render_line("first<ref>[foo bar]</ref> reference", &mut refs).unwrap(),
            "first<sup id=\"cite_ref_1\"><a href=\"#cite_ref_1\">1</a></sup> reference".to_string(),
        );

        assert_eq!(
            render_line("second<ref>[bar baz]</ref> reference", &mut refs).unwrap(),
            "second<sup id=\"cite_ref_2\"><a href=\"#cite_ref_2\">2</a></sup> reference"
                .to_string(),
        );

        assert_eq!(refs, vec!["[foo bar]".to_string(), "[bar baz]".to_string()],)
    }

    // internal links

    #[test]
    fn internal_link_bip() {
        assert_eq!(
            replace_internal_links("[[bip-0001.mediawiki|BIP 1]]".into()).unwrap(),
            a_href("/1", "BIP 1"),
        )
    }

    // external links

    #[test]
    fn external_link_simple() {
        assert_eq!(
            replace_external_links("[https://test.com test]".into()).unwrap(),
            a_href("https://test.com", "test"),
        )
    }

    #[test]
    fn external_link_multiple() {
        assert_eq!(
            replace_external_links(
                "[https://test.com test] hello [https://test.com another]".into()
            )
            .unwrap(),
            format!(
                "{} hello {}",
                a_href("https://test.com", "test"),
                a_href("https://test.com", "another")
            ),
        )
    }

    #[test]
    fn external_link_no_space() {
        assert_eq!(
            replace_external_links("[https://test.com test][https://test.com another]".into())
                .unwrap(),
            format!(
                "{}{}",
                a_href("https://test.com", "test"),
                a_href("https://test.com", "another")
            ),
        )
    }

    #[test]
    fn external_link_nested_bracket() {
        assert_eq!(
            replace_external_links("[https://test.com [lol] yep]".into()).unwrap(),
            a_href("https://test.com", "[lol] yep")
        )
    }

    #[test]
    fn external_link_bip() {
        assert_eq!(
            replace_external_links(
                "[https://github.com/bitcoin/bips/blob/master/bip-0065.mediawiki BIP-65]".into()
            )
            .unwrap(),
            a_href("/65", "BIP-65")
        )
    }

    // tables

    #[test]
    fn render_table_normal() {
        let input = lines(
            r#"{| class="wikitable"
            !colspan=3| template
            |-
            ! ColumnA !! ColumnB !! ColumnC
            |-
            | a || b || c
            |-
            | e || f || g
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|ColumnA|ColumnB|ColumnC|\n|-|-|-|\n".into(),
            "|a|b|c|\n".into(),
            "|e|f|g|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_multiple_lines() {
        let input = lines(
            r#"{|
            !thisisa
            !thisisb
            !thisisc
            |-
            |a
            |bb
            |ccc
            |-
            |a
            |bb
            |ccc
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|thisisa|thisisb|thisisc|\n|-|-|-|\n".into(),
            "|a|bb|ccc|\n".into(),
            "|a|bb|ccc|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_multiple_lines_no_explicit_header() {
        let input = lines(
            r#"{|
            |thisisa
            |thisisb
            |thisisc
            |-
            |a
            |bb
            |ccc
            |-
            |a
            |bb
            |ccc
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|thisisa|thisisb|thisisc|\n|-|-|-|\n".into(),
            "|a|bb|ccc|\n".into(),
            "|a|bb|ccc|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_single_then_multiple() {
        let input = lines(
            r#"{| class="wikitable"
            !colspan=2| testing testing
            |-
            ! A !! B
            |-
            | foo
            | bar
            |-
            | hello
            | world
            |-
            | lol
            | wtf
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|A|B|\n|-|-|\n".into(),
            "|foo|bar|\n".into(),
            "|hello|world|\n".into(),
            "|lol|wtf|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_no_proper_header() {
        let input = lines(
            r#"{|
            | A || B
            |-
            | foo || bar
            |-
            | hello || world
            |-
            | lol || wtf
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|A|B|\n|-|-|\n".into(),
            "|foo|bar|\n".into(),
            "|hello|world|\n".into(),
            "|lol|wtf|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_metadata() {
        let input = lines(
            r#"{| class="wikitable"
            |-
            | scope="col"| A
            | scope="col"| B
            | scope="col"| C
            |-
            | scope="row"| aaa
            | bbb
            | ccc
            |-
            | scope="row"| aaa
            | bbb
            | ccc
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|A|B|C|\n|-|-|-|\n".into(),
            "|aaa|bbb|ccc|\n".into(),
            "|aaa|bbb|ccc|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_metadata_empty() {
        let input = lines(
            r#"{| class="wikitable"
            |-
            | scope="col"|
            | scope="col"| B
            | scope="col"| C
            |-
            | scope="row"| aaa
            | bbb
            | ccc
            |-
            | scope="row"| aaa
            | bbb
            | ccc
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "||B|C|\n|-|-|-|\n".into(),
            "|aaa|bbb|ccc|\n".into(),
            "|aaa|bbb|ccc|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_row_style() {
        let input = lines(
            r#"{| class="wikitable"
            !
            !A
            !B
            !C
            !D
            |-
            | style="background: #99DDFF; color: black; text-align : center;" | empty
            |a
            |b
            | style="background: #99DDFF; color: black; text-align : center;" | c
            |d
            |-
            | style="background: #DDDDDD; color: black; text-align : center;" | empty
            |a
            |b
            | style="background: #DDDDDD; color: black; text-align : center;" | c
            |d
            |-
            | style="background: #EEDD88; color: black; text-align : center;" | empty
            |a
            |b
            | style="background: #EEDD88; color: black; text-align : center;" | c
            |d
            |-
            | style="background: #FFAABB; color: black; text-align : center;" | empty
            |a
            |b
            | style="background: #FFAABB; color: black; text-align : center;" | c
            |d
            |-
            | style="background: #BBCC33; color: black; text-align : center;" | empty
            |a
            |b
            | style="background: #BBCC33; color: black; text-align : center;" | c
            |d
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "||A|B|C|D|\n|-|-|-|-|-|\n".into(),
            "|empty|a|b|c|d|\n".into(),
            "|empty|a|b|c|d|\n".into(),
            "|empty|a|b|c|d|\n".into(),
            "|empty|a|b|c|d|\n".into(),
            "|empty|a|b|c|d|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_header_style() {
        let input = lines(
            r#"{| class="wikitable" style="text-align: center"
            !style="ignore"|A
            !style="ignore"|B
            !style="ignore"|C
            |-
            |a
            |b
            |c
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|A|B|C|\n|-|-|-|\n".into(),
            "|a|b|c|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }

    #[test]
    fn render_table_row_rowspan_ignore() {
        let input = lines(
            r#"{|
            !A
            !B
            !C
            |-
            | rowspan="ignore" | a
            |bb
            |ccc
            |-
            | rowspan="ignore" | a
            |bb
            |ccc
            |}"#,
        );

        let expected: Vec<String> = vec![
            "\n".into(),
            "|A|B|C|\n|-|-|-|\n".into(),
            "|a|bb|ccc|\n".into(),
            "|a|bb|ccc|\n\n".into(),
        ];

        assert_eq!(run(input), expected);
    }
}
