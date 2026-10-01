use std::io::{self, Write};

pub struct MatchLocation {
    pub line: usize,
    pub byte_offset: usize,
}

pub fn search<W: Write>(
    haystack: &[u8],
    pattern: &[u8],
    writer: &mut W,
) -> io::Result<Vec<MatchLocation>> {
    let mut locations = Vec::new();
    let mut offset = 0usize;

    for (i, line) in haystack.split(|&b| b == b'\n').enumerate() {
        let line_number = i + 1;
        if let Some(rel_offset) = find(line, pattern) {
            locations.push(MatchLocation {
                line: line_number,
                byte_offset: offset + rel_offset,
            });
            write!(writer, "{line_number}:")?;
            writer.write_all(line)?;
            writer.write_all(b"\n")?;
        }
        offset += line.len() + 1;
    }

    Ok(locations)
}

fn find(haystack: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    if pattern.len() > haystack.len() {
        return None;
    }
    haystack.windows(pattern.len()).position(|w| w == pattern)
}
