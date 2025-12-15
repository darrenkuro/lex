#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Char(char),
    Union,    // |
    Concat,   // ·
    Star,     // *
    Plus,     // +
    Question, // ?
    LParen,   // (
    RParen,   // )
}

pub fn to_postfix(regex: &str) -> Vec<Token> {
    let re = insert_concat_ops(regex);
    let mut output = String::new();
    let mut stack: Vec<char> = Vec::new();

    for c in re.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                output.push(c);
                output.push(' ');
            }
            '(' => stack.push(c),
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    output.push(top);
                    output.push(' ');
                }
            }
            '|' | '·' | '*' | '+' | '?' => {
                let prec1 = precedence(c);
                while let Some(&top) = stack.last() {
                    if top == '(' {
                        break;
                    }
                    let prec2 = precedence(top);
                    if prec2 >= prec1 {
                        output.push(stack.pop().unwrap());
                        output.push(' ');
                    } else {
                        break;
                    }
                }
                stack.push(c);
            }
            _ => {}
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op);
        output.push(' ');
    }

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_concat() {
        assert_eq!(to_postfix("ab"), "a b ·");
    }

    #[test]
    fn simple_union() {
        assert_eq!(to_postfix("a|b"), "a b |");
    }

    #[test]
    fn paren_and_star() {
        assert_eq!(to_postfix("a(b|c)*"), "a b c | * ·");
    }
}
