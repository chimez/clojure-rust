use ast::AstVal;
use syntax::SyntaxNode;
fn translate_fn(n: &SyntaxNode) -> String {
    let mut s = String::new();
    let mut main_flag = false;
    s.push_str("fn ");
    match &n.body[0].body[0].this {
        AstVal::AstString(s1) => {
            if s1 == "main" {
                main_flag = true;
            }
            s.push_str(&s1);
        }
        _ => panic!("not a function name"),
    }
    s.push_str("(");
    for item in &n.body[1].body {
        match &item.this {
            AstVal::AstString(s1) => {
                s.push_str(&s1);
                s.push_str(":&CljVal,");
            }
            _ => panic!("not a function parameter"),
        }
    }
    if main_flag {
        s.push_str(")");
    } else {
        s.push_str(")->CljVal");
    }
    s.push_str(&translate(&n.body[2]));
    if main_flag {
        s.pop();
        s.push_str(";}");
    }
    s
}

fn translate_let(n: &SyntaxNode) -> String {
    let mut s = String::new();
    s.push_str("let ");
    match &n.body[0].this {
        AstVal::AstSymbol(s1) => {
            s.push_str(&s1);
        }
        _ => panic!("can not let"),
    }
    s.push_str(" = ");
    s.push_str(&translate(&n.body[1]));
    s.push_str(";");
    s
}
fn translate_do(n: &SyntaxNode) -> String {
    let mut s = String::new();
    s.push_str("{");
    for item in &n.body {
        s.push_str(&translate(&item));
        s.push_str(";");
    }
    s.pop();
    s.push_str("}");
    s
}

fn translate_if(n: &SyntaxNode) -> String {
    let mut s = String::new();
    s.push_str("if(");
    s.push_str(&translate(&n.body[0]));
    s.push_str("){");
    s.push_str(&translate(&n.body[1]));
    s.push_str("}else{");
    s.push_str(&translate(&n.body[2]));
    s.push_str("}");
    s
}
fn translate_call(n: &SyntaxNode) -> String {
    let mut s = String::new();
    match &n.this {
        AstVal::AstSymbol(s1) => {
            s.push_str(&s1);
            s.push('(');
        }
        _ => panic!("not callable"),
    }
    for item in &n.body {
        s.push_str("&");
        s.push_str(&translate(&item));
        s.push(',');
    }
    s.push(')');
    s
}
fn translate_println(n: &SyntaxNode) -> String {
    let mut s0 = String::new();
    let mut s1 = String::new();
    for item in &n.body {
        s1.push_str(&translate(&item));
        s1.push_str(",");
        s0.push_str("{}");
    }
    let mut s = String::new();
    s.push_str("println!(\"");
    s.push_str(&s0);
    s.push_str("\",");
    s.push_str(&s1);
    s.push_str(");CljVal::CljNil");
    s
}
fn translate_equal(n: &SyntaxNode) -> String {
    let n1 = &n.body[0];
    let s1 = translate(&n1);
    let n2 = &n.body[1];
    let s2 = translate(&n2);
    let s = format!("( {} == {} )", s1, s2);
    s
}
pub fn translate(n: &SyntaxNode) -> String {
    if n.is_in_context(&n.this) {
        match &n.this {
            AstVal::AstSymbol(s) => return s.clone(),
            _ => panic!("can not happen"),
        }
    }
    match &n.this {
        AstVal::AstSymbol(t) => match t.as_str() {
            "fn" => translate_fn(&n),
            "let" => translate_let(&n),
            "if" => translate_if(&n),
            "do" => translate_do(&n),
            "println" => translate_println(&n),
            "=" => translate_equal(&n),
            _ => translate_call(&n),
        },
        AstVal::AstNil => String::from("CljVal::CljNil"),
        AstVal::AstBool(b) => format!("CljVal::CljBool({})", b),
        AstVal::AstInt(i) => format!("CljVal::CljInt({})", i),
        AstVal::AstFloat(f) => format!("CljVal::CljFloat({})", f),
        AstVal::AstString(s) => format!("CljVal::CljString(\"{}\".to_string())", s.to_string()),
        _ => panic!("not support yet!{:#?}", n),
    }
}
