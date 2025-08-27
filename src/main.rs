use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // Handle anchors first
    let (has_start_anchor, has_end_anchor, core_pattern) = parse_anchors(pattern);
    
    if has_start_anchor && has_end_anchor {
        // Pattern must match the entire line
        return match_pattern_at_position(input_line, &core_pattern, true);
    } else if has_start_anchor {
        // Pattern must match from the beginning
        return match_pattern_at_position(input_line, &core_pattern, false);
    } else if has_end_anchor {
        // Pattern must match at the end
        return match_at_end(input_line, &core_pattern);
    } else {
        // Pattern can match anywhere
        let chars: Vec<char> = input_line.chars().collect();
        for i in 0..=chars.len() {
            let remaining_input: String = chars[i..].iter().collect();
            if match_pattern_at_position(&remaining_input, &core_pattern, false) {
                return true;
            }
        }
        return false;
    }
}

fn parse_anchors(pattern: &str) -> (bool, bool, String) {
    let mut has_start = false;
    let mut has_end = false;
    let mut core_pattern = pattern.to_string();
    
    if pattern.starts_with('^') {
        has_start = true;
        core_pattern = pattern.chars().skip(1).collect();
    }
    
    if core_pattern.ends_with('$') {
        has_end = true;
        // Remove the last character ($)
        core_pattern = core_pattern.chars().take(core_pattern.chars().count() - 1).collect();
    }
    
    (has_start, has_end, core_pattern)
}

fn match_at_end(input_line: &str, pattern: &str) -> bool {
    // Try matching the pattern at every position, but only succeed if it matches at the end
    let chars: Vec<char> = input_line.chars().collect();
    for i in 0..=chars.len() {
        let remaining_input: String = chars[i..].iter().collect();
        if match_pattern_at_position(&remaining_input, pattern, true) {
            return true;
        }
    }
    false
}

fn match_pattern_at_position(input_line: &str, pattern: &str, must_consume_all: bool) -> bool {
    // Base case: pattern is empty
    if pattern.is_empty() {
        if must_consume_all {
            return input_line.is_empty(); // Must have consumed all input
        } else {
            return true; // Empty pattern matches
        }
    }
    
    // If input is empty but pattern isn't, no match
    if input_line.is_empty() {
        return false;
    }
    
    // Handle escape sequences (\d, \w, \\)
    if pattern.starts_with('\\') && pattern.len() >= 2 {
        let escape_char = pattern.chars().nth(1).unwrap();
        let first_input_char = input_line.chars().next().unwrap();
        
        match escape_char {
            'd' => {
                if first_input_char.is_ascii_digit() {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern, must_consume_all);
                }
                return false;
            }
            'w' => {
                if first_input_char.is_alphanumeric() || first_input_char == '_' {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern, must_consume_all);
                }
                return false;
            }
            '\\' => {
                if first_input_char == '\\' {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern, must_consume_all);
                }
                return false;
            }
            _ => panic!("Unhandled escape sequence: \\{}", escape_char)
        }
    }
    
    // Handle character classes [abc] or [^abc]
    if pattern.starts_with('[') {
        if let Some(close_bracket_pos) = pattern.find(']') {
            let mut is_negated = false;
            let mut char_class_start = 1;
            
            // Check for negation [^...]
            if pattern.len() > 2 && pattern.chars().nth(1) == Some('^') {
                is_negated = true;
                char_class_start = 2;
            }
            
            let char_class = &pattern[char_class_start..close_bracket_pos];
            let first_input_char = input_line.chars().next().unwrap();
            
            let char_matches = char_class.contains(first_input_char);
            
            if (is_negated && !char_matches) || (!is_negated && char_matches) {
                let remaining_input: String = input_line.chars().skip(1).collect();
                let remaining_pattern: String = pattern.chars().skip(close_bracket_pos + 1).collect();
                return match_pattern_at_position(&remaining_input, &remaining_pattern, must_consume_all);
            }
            return false;
        } else {
            panic!("Unclosed character class in pattern: {}", pattern);
        }
    }

    // Handle repetition operators (*, +, ?)
    if let Some(repeat_op) = pattern.chars().nth(1) {
        match repeat_op {
            '+' => {
                let pattern_char = pattern.chars().next().unwrap();
                let remaining_pattern: String = pattern.chars().skip(2).collect();
                
                // Must match at least one character
                if let Some(first_input_char) = input_line.chars().next() {
                    if first_input_char != pattern_char {
                        return false;
                    }
                    
                    // Try matching with different amounts of the repeated character
                    let mut current_input = input_line;
                    
                    // Consume characters while they match
                    while !current_input.is_empty() {
                        if let Some(next_char) = current_input.chars().next() {
                            if next_char == pattern_char {
                                current_input = &current_input[next_char.len_utf8()..];
                                
                                // Try matching the rest of the pattern at this position
                                if match_pattern_at_position(current_input, &remaining_pattern, must_consume_all) {
                                    return true;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    
                    // Try one final match if we consumed all characters
                    return match_pattern_at_position(current_input, &remaining_pattern, must_consume_all);
                }
                return false;
            }
            '*' => {
                let pattern_char = pattern.chars().next().unwrap();
                let remaining_pattern: String = pattern.chars().skip(2).collect();
                
                // Try matching without consuming any characters first
                if match_pattern_at_position(input_line, &remaining_pattern, must_consume_all) {
                    return true;
                }
                
                // Try matching with different amounts of the repeated character
                let mut current_input = input_line;
                
                while !current_input.is_empty() {
                    if let Some(next_char) = current_input.chars().next() {
                        if next_char == pattern_char {
                            current_input = &current_input[next_char.len_utf8()..];
                            
                            // Try matching the rest of the pattern at this position
                            if match_pattern_at_position(current_input, &remaining_pattern, must_consume_all) {
                                return true;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                
                return false;
            }
            '?' => {
                let pattern_char = pattern.chars().next().unwrap();
                let remaining_pattern: String = pattern.chars().skip(2).collect();
                
                // Try without consuming the character first
                if match_pattern_at_position(input_line, &remaining_pattern, must_consume_all) {
                    return true;
                }
                
                // Try consuming one character if it matches
                if let Some(first_char) = input_line.chars().next() {
                    if first_char == pattern_char {
                        let remaining_input = &input_line[first_char.len_utf8()..];
                        return match_pattern_at_position(remaining_input, &remaining_pattern, must_consume_all);
                    }
                }
                
                return false;
            }
            _ => {}
        }
    }

    // Handle literal character matching
    let pattern_char = pattern.chars().next().unwrap();
    let input_char = input_line.chars().next().unwrap();
    
    if pattern_char == input_char {
        let remaining_input: String = input_line.chars().skip(1).collect();
        let remaining_pattern: String = pattern.chars().skip(1).collect();
        return match_pattern_at_position(&remaining_input, &remaining_pattern, must_consume_all);
    }
    
    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
