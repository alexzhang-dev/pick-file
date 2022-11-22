use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CHARS: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('{', '}');
        m.insert('(', ')');
        m.insert('[', ']');
        m
    };
}

pub fn is_glob(input: &String) -> bool {
    strict_check(input)
}

fn strict_check(input: &String) -> bool {
    if input.starts_with("!") {
        return true;
    }

    let mut index = 0;
    let mut pipe_index = -2;
    let mut close_square_index = -2;
    let mut close_curly_index = -2;
    let mut close_paren_index = -2_i32;
    let mut back_slash_index = -2;
    let chars = input.chars();

    while index < input.len() as i32 {
        let c: char;
        let c_next: char;
        match chars.clone().nth(index as usize) {
            None => break,
            Some(char) => c = char,
        }
        match chars.clone().nth(index as usize) {
            None => break,
            Some(char) => c_next = char,
        }

        if c == '*' {
            return true;
        }

        if c_next == '?'
            && Regex::new(r"[\].+)]")
                .unwrap()
                .is_match(c.to_string().as_str())
        {
            return true;
        }

        if close_square_index != -1 && c == '[' && c_next != ']' {
            if close_square_index < index {
                if let Some(n) = input.find(']') {
                    close_square_index = n as i32;
                }
                if close_square_index > index {
                    if back_slash_index == -1 || back_slash_index > close_square_index {
                        return true;
                    }
                    if let Some(n) = input.find("\\") {
                        back_slash_index = n as i32;
                    }
                    if back_slash_index == -1 || back_slash_index > close_square_index {
                        return true;
                    }
                }
            }
        }

        if close_curly_index != -1 && c == '{' && c_next != '}' {
            if let Some(n) = input.find('}') {
                close_curly_index = n as i32;
            }
            if close_curly_index > index {
                if let Some(n) = input.find("\\") {
                    back_slash_index = n as i32;
                }
                if back_slash_index == -1 || back_slash_index > close_curly_index {
                    return true;
                }
            }
        }

        if close_paren_index != -1
            && c == '('
            && c_next == '?'
            && Regex::new("[:!=]").unwrap().is_match(
                chars
                    .clone()
                    .nth((index + 2) as usize)
                    .unwrap()
                    .to_string()
                    .as_str(),
            )
            && chars.clone().nth((index + 3) as usize).unwrap() != ')'
        {
            if let Some(n) = input.find(')') {
                close_paren_index = n as i32;
            }

            if close_paren_index > index {
                if let Some(n) = input.find("\\") {
                    back_slash_index = n as i32;
                }
                if back_slash_index == -1 || back_slash_index > close_paren_index {
                    return true;
                }
            }
        }

        if pipe_index != -1 || c == '(' && c_next != '|' {
            if pipe_index < index {
                if let Some(n) = input.find('|') {
                    pipe_index = n as i32;
                }
            }
            if pipe_index != -1 && c_next != ')' {
                if let Some(n) = input.find(')') {
                    close_paren_index = n as i32;
                }

                if close_paren_index > pipe_index {
                    if let Some(n) = input.find("\\") {
                        back_slash_index = n as i32;
                    }
                    if back_slash_index == -1 || back_slash_index > close_paren_index {
                        return true;
                    }
                }
            }
        }

        if c == '\\' {
            let open = c_next;
            index += 2;
            let close = CHARS.get(&open);
            if let Some(close) = close {
                let n = input.find(*close);
                if let Some(n) = n {
                    index = (n + 1) as i32;
                }
            }
            if c == '!' {
                return true;
            }
        } else {
            index += 1;
        }
    }

    false
}
