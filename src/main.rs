// use std::io;
// use std::io::{BufRead, Write};
mod syntax;
use syntax::syntax;
mod translate;
use translate::translate;
mod ast;
mod reader;
use reader::RawReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;
// TODO:REPL
// fn repl_read_line() -> String {
//     let stdout = io::stdout();
//     let mut handle = stdout.lock();
//     handle.write(b"user> ").unwrap();
//     handle.flush().unwrap();

//     let mut input_buffer = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();
//     handle.read_line(&mut input_buffer).unwrap();
//     format!("{}", input_buffer)
// }

fn main() {
    let mut file_name = String::new();
    let mut index = 0;
    for argument in env::args() {
        if index == 1 {
            file_name = argument;
            break;
        }
        index = index + 1;
    }
    let mut file = File::open(&file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut file = File::create(file_name.replace(".clj", ".rs")).unwrap();
    file.write_fmt(format_args!("mod cljtype;\nuse cljtype::CljVal;\n")).unwrap();
    let mut r = RawReader::new(contents);
    loop {
        match r.read() {
            None => break,
            Some(x) => {
                let c = syntax(&x);
                let t = translate(&c);
                file.write_fmt(format_args!("{}\n", t)).unwrap();
            }
        }
    }
}
