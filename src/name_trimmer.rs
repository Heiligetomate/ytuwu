const DEFAULT_REPLACE_CHARS: [char; 8] = ['(', ')', '[', ']', '{', '}', '/', '\''];
const ERASE_DUPLICATES     : [char; 2] = ['-', '_']; 


pub fn trim(mut input: String, space: &str) -> String {
    input = input.replace(" ", space);
    for chr in &DEFAULT_REPLACE_CHARS {
        input = input.replace(*chr, "");
    }
    trim_doubled_special_chars(input).to_lowercase()
}

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
