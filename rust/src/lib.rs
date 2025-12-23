use std::fs;
use std::path::Path;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Read input file from a custom path
pub fn read_input_from_path<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {:?}", path.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert_eq!(2 + 2, 4);
    }
}
