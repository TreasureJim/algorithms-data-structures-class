#![allow(dead_code)]

pub enum ErrorType {
    ClosingWithoutOpening,
    MissingClose,
}

pub struct ErrorMsg {
    missing: SymbolDiagnostic,
    err_type: ErrorType,
}

impl ErrorMsg {
    pub fn new(missing: SymbolDiagnostic, err_type: ErrorType) -> Self {
        Self { missing, err_type }
    }

    pub fn new_error(
        symbol: SymbolChar,
        line: usize,
        char_index: usize,
        err_type: ErrorType,
    ) -> Self {
        let diag = SymbolDiagnostic::new(symbol, line, char_index);
        Self {
            missing: diag,
            err_type,
        }
    }
}

#[derive(PartialEq)]
pub enum SymbolChar {
    Bracket,
    CurlyBracket,
    SquareBracket,
    /*
     * When enabled ignores any other characters including the multiline comment
     */
    Comment,
    /*
     * Can open without closing
     * Cannot close without opening
     * When opened ignores regular comments eg. (replace \ with /)
     *   \*
     *   * // *\
     */
    MultilineComment,
}

// enum ClosingSymbol {
//     Bracket,
//     CurlyBracket,
//     SquareBracket,
//     MultilineComment,
// }

pub struct SymbolDiagnostic {
    symbol: SymbolChar,
    line: usize,
    index: usize,
}

impl SymbolDiagnostic {
    pub fn new(symbol: SymbolChar, line: usize, index: usize) -> Self {
        Self {
            symbol,
            line,
            index,
        }
    }
}

pub fn verify_code(code: &str) -> Result<(), ErrorMsg> {
    let mut symbols = Vec::new();
    // true when in multiline comment
    let mut is_comment = false;

    for (line_index, line) in code.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (char_index, char) in chars.iter().enumerate() {
            let next_char = chars.get(char_index + 1).unwrap_or(&'\n');

            // check opening symbols
            let matching_char = match (char, next_char) {
                ('(', _) => Some(SymbolChar::Bracket),
                ('{', _) => Some(SymbolChar::CurlyBracket),
                ('[', _) => Some(SymbolChar::SquareBracket),
                ('/', '*') => Some(SymbolChar::MultilineComment),
                ('/', '/') => Some(SymbolChar::Comment),
                _ => None,
            };
            if let Some(matching_char) = matching_char {
                if matching_char == SymbolChar::MultilineComment {
                    is_comment = true;
                    symbols.push(SymbolDiagnostic::new(matching_char, line_index, char_index));
                    continue;
                }

                if !is_comment {
                    if matching_char == SymbolChar::Comment {
                        break;
                    }

                    symbols.push(SymbolDiagnostic::new(matching_char, line_index, char_index));
                    continue;
                }
            }

            // closing symbols
            let matching_char = match (char, next_char) {
                (')', _) => Some(SymbolChar::Bracket),
                ('}', _) => Some(SymbolChar::CurlyBracket),
                (']', _) => Some(SymbolChar::SquareBracket),
                ('*', '/') => Some(SymbolChar::MultilineComment),
                _ => None,
            };
            if let Some(matching_char) = matching_char {
                let previous_match = symbols.last();

                // if there is a closing symbol without ANY opening symbols
                if previous_match.is_none() {
                    return Err(ErrorMsg::new_error(
                        matching_char,
                        line_index,
                        char_index,
                        ErrorType::ClosingWithoutOpening,
                    ));
                }
                let previous_match = previous_match.expect("This should be impossible");

                if previous_match.symbol == matching_char {
                    if matching_char == SymbolChar::MultilineComment {
                        is_comment = false;
                    }

                    // remove the last opening symbol
                    symbols.truncate(symbols.len() - 1);
                } else {
                    return Err(ErrorMsg::new_error(
                        matching_char,
                        line_index,
                        char_index,
                        ErrorType::MissingClose,
                    ));
                }
            }
        }
    }

    // if symbols still left
    if !symbols.is_empty() {
        if symbols.last().unwrap().symbol == SymbolChar::MultilineComment {
            return Ok(());
        }
        let last_diag = symbols.remove(symbols.len() - 1);
        return Err(ErrorMsg::new(last_diag, ErrorType::MissingClose));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::verify_code;

    const VALID_FILE_PATH: &str = "./valid.c";
    const INVALID_FILE_PATH: &str = "./invalid.c";

    #[test]
    fn valid_c() {
        let test_str = r"int a; /* random comment */";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a = 55; // This is a comment / [";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a;";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a; /* for storing width * height */";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a = b*c;";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a = b / c;";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"public void doIt(int x) {System.out.println(x*100);}";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int []arr = new int[10];";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"/* */ {}";
        assert!(verify_code(test_str).is_ok());

        // taken from invalid example but is actually valid
        let test_str = r"int b = 5; /* this is a comment /";
        assert!(verify_code(test_str).is_ok());
    }

    #[test]
    fn invalid_c() {
        let test_str = r"int [arr = new int[10];";
        assert!(verify_code(test_str).is_err());

        let test_str = r"{a=b;";
        assert!(verify_code(test_str).is_err());

        let test_str = r"}";
        assert!(verify_code(test_str).is_err());
    }

    #[test]
    fn valid_cpp() {
        let test_str = r"if(a == b) {a++;}";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"if(a < (b*c)) {t = 5;}";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int []b = new int[5];";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"[](){}";
        assert!(verify_code(test_str).is_ok());

        let test_str = r"int a = 5; // init a to 5";
        assert!(verify_code(test_str).is_ok());
    }

    #[test]
    fn invalid_cpp() {
        let test_str = r"for(int i=0;i<10;i++] {a+= b;}";
        assert!(verify_code(test_str).is_err());

        let test_str = r"{abc)";
        assert!(verify_code(test_str).is_err());
    }
}
