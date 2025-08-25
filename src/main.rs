use std::env;
use std::io;
use std::process;

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     // if pattern.chars().count() == 1 {
//     //     return input_line.contains(pattern);
//     // } else if pattern.contains("\\d") {
//     //     return input_line.chars().any(|c| c.is_numeric());
//     // } else if pattern.contains("\\w") {
//     //     return input_line.chars().any(|c| c.is_alphanumeric() || c == '_');
//     // } else if pattern.starts_with("[^") && pattern.ends_with("]") {
//     //     return input_line.chars().any(|c| !pattern[2..pattern.len()-1].contains(c));
//     // } else if pattern.starts_with("[") && pattern.ends_with("]") {
//     //     return input_line.chars().any(|c| pattern[1..pattern.len()-1].contains(c));
//     // } else {
//     //     panic!("Unhandled pattern: {}", pattern)
//     // }
//     if pattern == "" && input_line == "" {
//         return true;
//     } else if pattern[0..1] == *"\\" {
//         if pattern[1..2] == *"d" && input_line[0..1].chars().any(|c| c.is_numeric()) {
//             return match_pattern(&input_line[1..], &pattern[2..]);
//         } else if pattern[1..2] == *"w" && input_line[0..1].chars().any(|c| c.is_alphanumeric() || c == '_') {
//             return match_pattern(&input_line[1..], &pattern[2..]);
//         } else {
//             panic!("Unhandled pattern: {}", pattern)
//         }
//     } else if pattern[0..1] == *"[" {
//         let neg = false;
//         if pattern[1..2] == *"^" {
//             neg = true;
//             pattern = pattern[2..];
//         }
//         let mut chars = "";
//         while pattern.len() > 0 && pattern[0..1] != *"]" {
//             chars.push_str(&pattern[0..1]);
//             pattern = pattern[1..].to_string();
//         }
//         if neg && input_line[0..1].chars().any(|c| !chars.contains(c)) {
//             return match_pattern(&input_line[1..], &pattern);
//         } else if !neg && input_line[0..1].chars().any(|c| chars.contains(c)) {
//             return match_pattern(&input_line[1..], &pattern);
//         } else {
//             panic!("Unhandled pattern: {}", pattern)
//         }
//     } else if pattern[0..1] == input_line[0..1] {
//         return match_pattern(&input_line[1..], &pattern[1..]);
//     } else {
//         panic!("Unhandled pattern: {}", pattern)
//     }
// }

// fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     // Base case: both pattern and input are empty
//     if pattern.is_empty() && input_line.is_empty() {
//         return true;
//     }
    
//     // If pattern is empty but input isn't, no match
//     if pattern.is_empty() {
//         return false;
//     }
    
//     // If input is empty but pattern isn't, no match (except for empty pattern case above)
//     if input_line.is_empty() {
//         return false;
//     }
    
//     // Handle escape sequences (\d, \w)
//     if pattern.starts_with('\\') && pattern.len() >= 2 {
//         let escape_char = pattern.chars().nth(1).unwrap();
//         let first_input_char = input_line.chars().next().unwrap();
        
//         match escape_char {
//             'd' => {
//                 if first_input_char.is_ascii_digit() {
//                     return match_pattern(&input_line[1..], &pattern[2..]);
//                 }
//                 return false;
//             }
//             'w' => {
//                 if first_input_char.is_alphanumeric() || first_input_char == '_' {
//                     return match_pattern(&input_line[1..], &pattern[2..]);
//                 }
//                 return false;
//             }
//             _ => panic!("Unhandled escape sequence: \\{}", escape_char)
//         }
//     }
    
//     // Handle character classes [abc] or [^abc]
//     if pattern.starts_with('[') {
//         if let Some(close_bracket_pos) = pattern.find(']') {
//             let mut is_negated = false;
//             let mut char_class_start = 1;
            
//             // Check for negation [^...]
//             if pattern.len() > 2 && pattern.chars().nth(1) == Some('^') {
//                 is_negated = true;
//                 char_class_start = 2;
//             }
            
//             let char_class = &pattern[char_class_start..close_bracket_pos];
//             let first_input_char = input_line.chars().next().unwrap();
            
//             let char_matches = char_class.contains(first_input_char);
            
//             if (is_negated && !char_matches) || (!is_negated && char_matches) {
//                 return match_pattern(&input_line[1..], &pattern[close_bracket_pos + 1..]);
//             }
//             return false;
//         } else {
//             panic!("Unclosed character class in pattern: {}", pattern);
//         }
//     }
    
//     // Handle literal character matching
//     let pattern_char = pattern.chars().next().unwrap();
//     let input_char = input_line.chars().next().unwrap();
    
//     if pattern_char == input_char {
//         return match_pattern(&input_line[1..], &pattern[1..]);
//     }
    
//     false
// }

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // Try to match the pattern starting at every position in the input
    let chars: Vec<char> = input_line.chars().collect();
    if pattern.starts_with('^') {
        let remaining_pattern: String = pattern.chars().skip(1).collect();
        if match_pattern_at_position(&input_line, &remaining_pattern) {
            return true;
        }
    } else {
        for i in 0..=chars.len() {
            let remaining_input: String = chars[i..].iter().collect();
            if match_pattern_at_position(&remaining_input, pattern) {
                return true;
            }
        }
    }
    false
}

fn match_pattern_at_position(input_line: &str, pattern: &str) -> bool {
    // Base case: both pattern and input are empty
    if pattern.is_empty() {
        return true; // Empty pattern matches
    }
    
    // If input is empty but pattern isn't, no match
    if input_line.is_empty() {
        return false;
    }
    
    // Handle escape sequences (\d, \w)
    if pattern.starts_with('\\') && pattern.len() >= 2 {
        let escape_char = pattern.chars().nth(1).unwrap();
        let first_input_char = input_line.chars().next().unwrap();
        
        match escape_char {
            'd' => {
                if first_input_char.is_ascii_digit() {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern);
                }
                return false;
            }
            'w' => {
                if first_input_char.is_alphanumeric() || first_input_char == '_' {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern);
                }
                return false;
            }
            '\\' => {
                if first_input_char == '\\' {
                    let remaining_input: String = input_line.chars().skip(1).collect();
                    let remaining_pattern: String = pattern.chars().skip(2).collect();
                    return match_pattern_at_position(&remaining_input, &remaining_pattern);
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
                return match_pattern_at_position(&remaining_input, &remaining_pattern);
            }
            return false;
        } else {
            panic!("Unclosed character class in pattern: {}", pattern);
        }
    }
    
    // Handle literal character matching
    let pattern_char = pattern.chars().next().unwrap();
    let input_char = input_line.chars().next().unwrap();
    
    if pattern_char == input_char {
        let remaining_input: String = input_line.chars().skip(1).collect();
        let remaining_pattern: String = pattern.chars().skip(1).collect();
        return match_pattern_at_position(&remaining_input, &remaining_pattern);
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
