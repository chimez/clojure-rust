use ast::AstVal;
use std::char::decode_utf16;
#[derive(Debug)]
enum ReadState {
    EOF,
    Continue(AstVal),
    Delimited(char),
}
#[derive(Debug, Clone)]
pub struct RawReader(Vec<char>);

impl RawReader {
    pub fn new(s: String) -> RawReader {
        let s: Vec<u16> = s.encode_utf16().collect();
        let v = decode_utf16(s.iter().cloned()).collect::<Vec<_>>();
        let mut v1: Vec<char> = Vec::new();
        for c in v {
            match c {
                Ok(c) => v1.push(c),
                _ => panic!("not in utf-16"),
            }
        }
        v1.reverse();
        RawReader(v1)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn consume_char(&mut self) -> Option<char> {
        match self.0.pop() {
            Some(c) => Some(c),
            None => None,
        }
    }
    fn pre_read_next_char(&self) -> Option<char> {
        if self.len() == 0 {
            return None;
        };
        Some(self.0[self.len() - 1])
    }
    pub fn read(&mut self) -> Option<AstVal> {
        match read_internal(self) {
            ReadState::Continue(a) => Some(a),
            ReadState::EOF => None,
            _ => panic!("error"),
        }
    }
    fn unread_char(&mut self, ch: char) {
        self.0.push(ch);
    }
}

fn read_number(reader: &mut RawReader) -> AstVal {
    let mut s = String::new();
    loop {
        match reader.pre_read_next_char() {
            Some(next) => {
                if (next.is_ascii_digit())
                    | (next == '.')
                    | (next == 'e')
                    | (next == '-')
                    | (next == '+')
                {
                    reader.consume_char();
                    s.push(next)
                } else {
                    break;
                }
            }
            None => {
                break;
            }
        };
    }
    match s.parse::<i32>() {
        Ok(x) => AstVal::AstInt(x),
        Err(_) => match s.parse::<f32>() {
            Ok(x) => AstVal::AstFloat(x),
            Err(_) => panic!("read number"),
        },
    }
}

fn is_macro_terminating(ch: char) -> bool {
    match ch {
        '\"' => true,
        ';' => true,
        '@' => true,
        '^' => true,
        '`' => true,
        '~' => true,
        '(' => true,
        ')' => true,
        '[' => true,
        ']' => true,
        '{' => true,
        '}' => true,
        '\\' => true,
        _ => false,
    }
}
fn read_token(reader: &mut RawReader) -> Option<String> {
    if reader.len() == 0 {
        return None;
    };
    let mut s = String::new();
    loop {
        if reader.len() == 0 {
            return Some(s);
        };
        let ch = reader.pre_read_next_char().unwrap();
        if (ch == ' ') | (ch == '\n') | (is_macro_terminating(ch)) {
            break;
        } else {
            let ch = reader.consume_char().unwrap();
            s.push(ch);
        }
    }
    Some(s)
}

fn escape_char(reader: &mut RawReader) -> char {
    reader.consume_char();
    let ch = reader.consume_char();
    match ch {
        Some('t') => '\t',
        Some('n') => '\n',
        Some('r') => '\r',
        Some('\\') => '\\',
        Some('"') => '"',
        Some(c) => panic!(format!("no define char:{}", c)),
        None => panic!("nothing after \\!"),
    }
}
fn read_string(reader: &mut RawReader) -> ReadState {
    match reader.pre_read_next_char() {
        Some('"') => ReadState::Continue(AstVal::AstString(read_string_inner(reader))),
        _ => panic!("read string error"),
    }
}

fn read_string_inner(reader: &mut RawReader) -> String {
    let mut s = String::new();
    reader.consume_char();
    loop {
        if reader.len() == 0 {
            return s;
        };
        let ch = reader.pre_read_next_char().unwrap();
        match ch {
            '\\' => s.push(escape_char(reader)),
            '"' => {
                reader.consume_char();
                return s;
            }
            _ => {
                reader.consume_char();
                s.push(ch)
            }
        }
    }
}

fn read_symbol(reader: &mut RawReader) -> AstVal {
    let token = match read_token(reader) {
        Some(t) => t,
        None => panic!("nothing"),
    };
    if token == "nil" {
        AstVal::AstNil
    } else if token == "true" {
        AstVal::AstBool(true)
    } else if token == "false" {
        AstVal::AstBool(false)
    } else {
        AstVal::AstSymbol(token)
    }
}

fn read_delimited(reader: &mut RawReader, delim: char, into_l: &mut AstVal) -> AstVal {
    // println!("into read delimited internal");
    reader.consume_char();
    let mut l = into_l.clone();
    loop {
        match read_internal(reader) {
            ReadState::Delimited(c) => {
                if c == delim {
                    return l;
                } else {
                    continue;
                }
            }
            ReadState::EOF => {
                panic!("read delimited internal");
            }
            ReadState::Continue(c) => {
                l.push(c);
            }
        }
    }
}
fn read_list(reading: &mut RawReader) -> ReadState {
    let mut into_list = AstVal::new_list();
    let the_list = read_delimited(reading, ')', &mut into_list);
    if the_list == AstVal::AstNil {
        ReadState::Continue(AstVal::AstNil)
    } else {
        ReadState::Continue(the_list)
    }
}
fn read_unmatched_delimiter(reader: &mut RawReader) -> ReadState {
    // panic!("unmatched delimiter")
    let ch = reader.consume_char().unwrap();
    ReadState::Delimited(ch)
}
fn read_keyword(reader: &mut RawReader) -> ReadState {
    reader.consume_char();
    match reader.pre_read_next_char() {
        Some(' ') => panic!("read keyword"),
        _ => match read_token(reader) {
            Some(s) => ReadState::Continue(AstVal::AstKeyword(s)),
            None => panic!("read keyword"),
        },
    }
}
fn read_comment(reader: &mut RawReader) -> ReadState {
    reader.consume_char();
    let mut s = String::new();
    loop {
        match reader.consume_char() {
            Some('\n') => {
                break;
            }
            Some(c) => {
                s.push(c);
            }
            None => {
                break;
            }
        }
    }
    ReadState::Continue(AstVal::AstCommentLine(s))
}
fn wrapping_reader(reader: &mut RawReader, ch: char) -> ReadState {
    reader.consume_char();
    let mut l = AstVal::new_list();
    l.push(AstVal::AstSymbol(ch.to_string()));
    match read_internal(reader) {
        ReadState::Continue(x) => l.push(x),
        _ => panic!("nothing after '"),
    }
    ReadState::Continue(l)
}
fn desugar_meta(f: AstVal) -> AstVal {
    let mut m = AstVal::new_meta();
    match f {
        AstVal::AstKeyword(s) => {
            m.insert(AstVal::AstKeyword(s), AstVal::AstBool(true));
            return m;
        }
        AstVal::AstSymbol(s) | AstVal::AstString(s) => {
            m.insert(
                AstVal::AstKeyword(String::from("tag")),
                AstVal::AstString(s),
            );
            return m;
        }
        _ => f,
    }
}
fn read_meta_sugar(reader: &mut RawReader) -> AstVal {
    match read_internal(reader) {
        ReadState::Continue(f) => {
            if let AstVal::AstMeta(m) = desugar_meta(f) {
                return AstVal::AstMeta(m);
            } else {
                panic!("read meta 1")
            }
        }
        _ => panic!("read meta"),
    }
}

fn read_meta(reader: &mut RawReader) -> ReadState {
    // println!("into meta");
    reader.consume_char();
    match reader.pre_read_next_char() {
        Some('{') => match read_map(reader) {
            ReadState::Continue(a) => ReadState::Continue(a.map_to_meta()),
            _ => panic!("read meta 3"),
        },
        Some(_) => ReadState::Continue(read_meta_sugar(reader)),
        _ => panic!("read meta 0"),
    }
}

fn read_vector(reader: &mut RawReader) -> ReadState {
    let mut into_vec = AstVal::new_vec();
    let the_vector = read_delimited(reader, ']', &mut into_vec);
    if the_vector == AstVal::AstNil {
        ReadState::Continue(AstVal::AstNil)
    } else {
        ReadState::Continue(the_vector)
    }
}
fn read_map(reader: &mut RawReader) -> ReadState {
    let mut into_map = AstVal::new_map();
    let the_map = read_delimited(reader, '}', &mut into_map);
    if the_map.len() % 2 == 1 {
        panic!("map count")
    }
    if the_map == AstVal::AstNil {
        ReadState::Continue(AstVal::AstNil)
    } else {
        ReadState::Continue(the_map)
    }
}
fn read_symbol_or_number(reader: &mut RawReader) -> AstVal {
    reader.consume_char();
    match reader.pre_read_next_char() {
        Some(ch) if ch.is_ascii_digit() => {
            reader.unread_char('-');
        read_number(reader)
        }
        Some(_) | None => {
            reader.unread_char('-');
            read_symbol(reader)
        }
    }
}
fn which_macro(ch: char) -> Option<fn(&mut RawReader) -> ReadState> {
    match ch {
        '"' => Some(read_string),
        ':' => Some(read_keyword),
        '(' => Some(read_list),
        ')' => Some(read_unmatched_delimiter),
        '^' => Some(read_meta),
        '[' => Some(read_vector),
        ']' => Some(read_unmatched_delimiter),
        '{' => Some(read_map),
        '}' => Some(read_unmatched_delimiter),
        ';' => Some(read_comment),
        // TODO: support macro
        // '`' => Some(read_syntax_quote),
        // '~' => Some(read_unquote),
        // TODO: lambda macro fn #(%)
        // '%'=>Some(read_arg),
        // TODO: # macro
        _ => None,
    }
}
fn read_internal(reader: &mut RawReader) -> ReadState {
    // println!("{:#?}", reader);
    // println!("into read internal!");
    loop {
        match reader.pre_read_next_char() {
            Some(' ') | Some('\t') | Some('\n') => {
                // println!("read a char");
                reader.consume_char();
                continue;
            }
            Some('-') => return ReadState::Continue(read_symbol_or_number(reader)),
            Some(ch) if (ch.is_ascii_digit()) => {
                // println!("read number");
                return ReadState::Continue(read_number(reader));
            }
            Some(ch) if (ch == '\'') | (ch == '@') => {
                return wrapping_reader(reader, ch);
            }
            Some(ch) => match which_macro(ch) {
                None => {
                    // println!("read symbol");
                    return ReadState::Continue(read_symbol(reader));
                }
                Some(f) => {
                    // println!("read macro");
                    match f(reader) {
                        ReadState::Continue(res) => return ReadState::Continue(res),
                        ReadState::EOF => return ReadState::EOF,
                        ReadState::Delimited(ch) => return ReadState::Delimited(ch),
                    }
                }
            },
            None => {
                return ReadState::EOF;
            }
        }
    }
}
