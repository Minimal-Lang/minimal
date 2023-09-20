use std::{fs::File, io::Read, path::Path};

/// Reads a file into a `Vec<char>`.
pub fn read_file_contents<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<char>> {
    let mut file = File::open(path.as_ref())?;
    let size = file.metadata().map(|m| m.len()).unwrap_or(0);

    let mut string = String::with_capacity(size as usize);

    file.read_to_string(&mut string)?;

    let contents = string.chars().collect();

    Ok(contents)
}

pub fn exclude_comment_lines(contents: &[char]) -> Vec<char> {
    let mut vec: Vec<char> = Vec::new();

    for line in contents.split(|v| *v == '\n') {
        if !line.starts_with(&['~']) {
            for c in line {
                vec.push(*c);
            }
            vec.push('\n');
        }
    }

    vec
}
