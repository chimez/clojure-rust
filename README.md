# ClojureRust
## What is ClojureRust?
ClojureRust is a compiler for Clojure that targets Rust source code. 
It is designed to generate readable Rust source code, and ensure the same characteristics as Rust native code: no GC, memory security, and zero-headed abstraction.
## Getting Started
### Releases and dependency information
Latest release: v0.0.1
Some basic framework had been done.
There are no dependencies other than the Rust standard library.
### Usage
The hello world example is in the `example` folder.
```
; hello_world.clj
(defn f [x]
  (let [y "world"]
    (if (= y "e")
      (println "error")
      (println x " " y "!"))))

(defn main []
  (f "hello"))
```
This shows the string type, the definiting and the calling function, and the use of the `let` `if` keyword.

1. generate code: `./clojure-rust hello_world.clj`, which will generate a `hello_world.rs` file.
2. new project: `cargo new hello_world --bin`, use cargo to create a new project.
3. put the code into it: `cp hello_world.rs hello_world/src/main.rs`, it is necessary to rename the file, because the entry function is here.
4. put the "standard library" into it: `cp clojure-rust/src/cljtype.rs hello_world/src/cljtype.rs`, which is the necessary library to build the project.
5. run it: `cargo run`

## Project structure
### Source files
```
.
├── main.rs ;entry function
├── ast.rs ;Abstract syntax tree type
├── reader.rs ;grammar parser
├── syntax.rs ;Semantic Analysis
├── translate.rs ;Code generation
└── cljtype.rs ;Standard library
```
### Project operation process
```
 main.rs  --> type.RawReader  ;the string of input Clojure source code
   |               |
   v               v
reader.rs --> type.AstVal     ;Match parentheses and desugar
   |               |
   v               v
syntax.rs --> type.SyntaxNode ;Analysis context, symbols, etc.
   |               |
   v               v
translate.rs --> string       ;Rust source code
```
## Currently supported features
1. Define and call basic functions, not support remaining parameters, default parameters and closures
2. `let`
3. `if`
4. `=` 
5. `println`

## TODO
1. macros
2. closures
3. more functions in the standard library i.e. `clojure.core`
## License
not sure yet, same as Rust or Clojure.
