/// List of chars that should be replaced with another char
const DEFAULT_REPLACE_CHARS: [char; 8] = ['(', ')', '[', ']', '{', '}', '/', '\''];

/// List of chars that should not be multiple times in a row in a name
const ERASE_DUPLICATES: [char; 2] = ['-', '_'];

/// calls trim with - as space
pub fn default_trim(input: &str) -> String {
    trim(input, "-")
}

/// Replaces every char that is contained in DEFAULT_REPLACE_CHARS wih the given space
/// Calls trim_doubled_special_chars to erase all duplicates
pub fn trim(input: &str, space: &str) -> String {
    let mut input = input.to_owned();
    input = input.replace(" ", space);
    for chr in &DEFAULT_REPLACE_CHARS {
        input = input.replace(*chr, "");
    }
    trim_doubled_special_chars(input).to_lowercase()
}

/// Removes every duplicate char that is also contained in ERASE_DUPLICATES
fn trim_doubled_special_chars(mut input: String) -> String {
    let mut new_name = String::new();
    let mut previous_char = ' ';
    for chr in input.drain(..) {
        if chr == previous_char && ERASE_DUPLICATES.contains(&chr) {
            continue;
        }
        new_name.push(chr);
        previous_char = chr;
    }
    new_name
}

// TODO: This is garbage
