use anyhow::Result;
use std::{io::Write, sync::LazyLock};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::{LinesWithEndings, as_24_bit_terminal_escaped},
};

static PS: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static TS: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

const THEME: &str = "base16-ocean.dark";

pub fn syntax_for(content_type: &str) -> Option<&'static SyntaxReference> {
    let ext = if content_type.contains("json") {
        "json"
    } else if content_type.contains("html") {
        "html"
    } else if content_type.contains("xml") {
        "xml"
    } else {
        return None;
    };

    PS.find_syntax_by_extension(ext)
}

pub fn highlight(out: &mut dyn Write, body: &str, syntax: &SyntaxReference) -> Result<()> {
    let mut h = HighlightLines::new(syntax, &TS.themes[THEME]);
    for line in LinesWithEndings::from(body) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &PS)?;
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        write!(out, "{escaped}")?;
    }
    write!(out, "\x1b[0m")?;
    Ok(())
}
