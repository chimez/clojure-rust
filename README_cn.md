# ClojureRust
## ClojureRust 是什么?
ClojureRust 是将Clojure编译到Rust源码的编译器. 
它的设计目标是生成可读的Rust源码,并尽可能保证与Rust原生代码相同的特性:无GC,内存安全,以及零开销抽象.
## 开始使用
### 版本与依赖信息
当前为 v0.0.1 版本,已经完成了一些基本的框架工作.
除了Rust 标准库外无其他依赖.
### 使用
现有的hello world 例子在 `example` 文件夹中.
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
这里展示了字符串类型,定义与调用函数,和 `let`, `if` 关键字的使用.

1. 生成代码: `./clojure-rust hello_world.clj`, 这会生成一个 `hello_world.rs` 文件.
2. 新建项目: `cargo new hello_world --bin`, 使用cargo新建一个项目.
3. 将代码放进去: `cp hello_world.rs hello_world/src/main.rs`, 因为入口函数在这里,所以将文件重命名是有必要的.
4. 将"标准库"放进去: `cp clojure-rust/src/cljtype.rs hello_world/src/cljtype.rs`, 这是运行代码所必要的核心库.
5. 运行: `cargo run`

## 项目结构
### 源码文件
```
.
├── main.rs ;总的入口函数
├── ast.rs ;抽象语法树的类型
├── reader.rs ;语法解析器
├── syntax.rs ;语义分析
├── translate.rs ;代码生成
└── cljtype.rs ;标准库
```
### 项目运行流程
```
 main.rs  --> type.RawReader  ;输入的clojure源码的字符串
   |               |
   v               v
reader.rs --> type.AstVal     ;匹配括号并将语法糖解除
   |               |
   v               v
syntax.rs --> type.SyntaxNode ;分析上下文,符号等
   |               |
   v               v
translate.rs --> string       ;rust代码
```
## 现在支持的功能
1. 定义与调用基本函数,暂不支持剩余参数,默认参数和闭包
2. `let`
3. `if`
4. `=` 
5. `println`

## TODO
1. 宏
2. 闭包
3. 标准库中加入更多函数,即`clojure.core`
## 许可证
尚未确定,与Rust相同或与Clojure相同.
