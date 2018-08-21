use ast::AstVal;
#[derive(Debug, Clone)]
pub struct SyntaxNode {
    pub this: AstVal,
    pub body: Vec<SyntaxNode>,
    pub context: Vec<Vec<AstVal>>,
}

impl SyntaxNode {
    fn new() -> SyntaxNode {
        SyntaxNode {
            this: AstVal::AstNil,
            body: Vec::new(),
            context: Vec::new(),
        }
    }
    fn push_body(&mut self, next: SyntaxNode) {
        self.body.push(next);
    }
    fn set_this(&mut self, this: AstVal) {
        self.this = this
    }
    fn new_leaf(a: AstVal) -> SyntaxNode {
        SyntaxNode {
            this: a,
            body: Vec::new(),
            context: Vec::new(),
        }
    }
    fn push_string(&mut self, s: String) {
        let n = SyntaxNode::new_leaf(AstVal::AstString(s));
        self.push_body(n);
    }
    fn push_str(&mut self, s: &str) {
        self.push_string(s.to_string());
    }
    fn push_context_vec(&mut self, v: Vec<AstVal>) {
        self.context.push(v);
    }
    fn pop_context_vec(&mut self) -> Option<Vec<AstVal>> {
        self.context.pop()
    }
    fn merge_context_vec(&mut self, v: &Vec<Vec<AstVal>>) {
        for i in v {
            self.push_context_vec(i.clone());
        }
    }
    fn merge_context(&mut self, n: &SyntaxNode) {
        self.merge_context_vec(&n.context);
    }
    pub fn is_in_context(&self, a: &AstVal) -> bool {
        for v in &self.context {
            for i in v {
                if a == i {
                    return true;
                }
            }
        }
        false
    }

    fn disseminate_context(&mut self) {
        for mut item in self.body.iter_mut() {
            item.merge_context_vec(&self.context);
            item.disseminate_context();
        }
    }
}
fn syntax_let(a: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match a {
        AstVal::AstList(v) => {
            if v.len() < 2 {
                panic!("not enough let")
            }
            match &v[0] {
                AstVal::AstSymbol(s) => {
                    if s == "let" {
                        result.set_this(AstVal::AstSymbol("do".to_string()));
                    } else {
                        panic!("not a let")
                    }
                }
                _ => panic!("not a let"),
            }
            match &v[1] {
                AstVal::AstVec(v1) => {
                    if v1.len() % 2 != 0 {
                        panic!("not enoug")
                    }
                    let mut symbol_flag = true;
                    let mut let_expr = SyntaxNode::new();
                    let mut context_vec: Vec<AstVal> = Vec::new();

                    for item in v1 {
                        if symbol_flag {
                            let mut symbol = SyntaxNode::new();
                            symbol.set_this(item.clone());
                            let_expr.set_this(AstVal::AstSymbol("let".to_string()));
                            let_expr.push_body(symbol);
                            context_vec.push(item.clone());
                        } else {
                            let expr = dispatch_syntax(item);
                            let_expr.push_body(expr);
                            result.push_body(let_expr.clone());
                            let_expr = SyntaxNode::new();
                        }
                        symbol_flag = !symbol_flag;
                    }
                    result.push_context_vec(context_vec);
                }
                _ => panic!("not a let vec"),
            }
            if v.len() == 2 {
                return result;
            } else {
                for item in v[2..].to_vec() {
                    result.push_body(dispatch_syntax(&item));
                }
                return result;
            }
        }
        _ => panic!("not a let"),
    }
}
fn syntax_if(a: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match a {
        AstVal::AstList(v) => {
            if v.len() != 4 {
                panic!("not a full if");
            }
            match &v[0] {
                AstVal::AstSymbol(s) => {
                    if s == "if" {
                        result.set_this(v[0].clone());
                        let mut n = SyntaxNode::new();
                        n.set_this(AstVal::AstSymbol("do".to_string()));
                        n.push_body(dispatch_syntax(&v[1]));
                        result.push_body(n);
                        let mut n = SyntaxNode::new();
                        n.set_this(AstVal::AstSymbol("do".to_string()));
                        n.push_body(dispatch_syntax(&v[2]));
                        result.push_body(n);
                        let mut n = SyntaxNode::new();
                        n.set_this(AstVal::AstSymbol("do".to_string()));
                        n.push_body(dispatch_syntax(&v[3]));
                        result.push_body(n);
                        return result;
                    } else {
                        panic!("not a if")
                    }
                }
                _ => panic!("not a symbol list"),
            }
        }
        _ => panic!("not a list"),
    }
}
fn syntax_call(a: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match a {
        AstVal::AstList(v) => match &v[0] {
            AstVal::AstSymbol(s) => {
                result.set_this(AstVal::AstSymbol(s.clone()));
                if v.len() > 1 {
                    for brother in v[1..].to_vec() {
                        result.push_body(dispatch_syntax(&brother));
                    }
                }
                return result;
            }
            _ => panic!("can't be called"),
        },
        _ => {
            result.set_this(a.clone());
            return result;
        }
    }
}
fn syntax_equal(a: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match a {
        AstVal::AstList(v) => {
            if v.len() != 3 {
                panic!("not a equal = call")
            }
            match &v[0] {
                AstVal::AstSymbol(s) => {
                    if s == "=" {
                        result.set_this(AstVal::AstSymbol(s.to_string()));
                        result.push_body(dispatch_syntax(&v[1]));
                        result.push_body(dispatch_syntax(&v[2]));
                        return result;
                    }else{
                        panic!("not a equal")
                    }
                }
                _ => panic!("not a equal"),
            }
        }
        _ => panic!("not a equal list"),
    }
}
fn syntax_defn(a: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match a {
        AstVal::AstList(v) => {
            if v.len() <= 3 {
                panic!("not enough defn")
            }
            match &v[0] {
                AstVal::AstSymbol(s) => {
                    if s != "defn" {
                        panic!("not a defn")
                    } else {
                        result.set_this(AstVal::AstSymbol("fn".to_string()));
                    }
                }
                _ => panic!("not a defn"),
            }
            match &v[1] {
                AstVal::AstSymbol(s) => {
                    let mut n = SyntaxNode::new();
                    n.set_this(AstVal::AstSymbol("name".to_string()));
                    n.push_string(s.clone());
                    result.push_body(n);
                }
                _ => panic!("no function name"),
            }
            match &v[2] {
                AstVal::AstVec(v1) => {
                    let mut context_vec: Vec<AstVal> = Vec::new();
                    let mut n = SyntaxNode::new();

                    n.set_this(AstVal::AstSymbol("parameters".to_string()));
                    for item in v1 {
                        match item {
                            AstVal::AstSymbol(s) => {
                                n.push_string(s.clone());
                                context_vec.push(AstVal::AstSymbol(s.clone()));
                            }
                            _ => panic!("not a parameter"),
                        }
                    }
                    result.push_body(n);
                    result.push_context_vec(context_vec);
                }
                AstVal::AstNil => {
                    let mut n = SyntaxNode::new();
                    n.set_this(AstVal::AstSymbol("parameters".to_string()));
                    result.push_body(n);
                }
                _ => panic!("no function parameter"),
            }
            let mut d = SyntaxNode::new();
            for item in v[3..].to_vec() {
                d.set_this(AstVal::AstSymbol("do".to_string()));
                d.push_body(dispatch_syntax(&item));
            }
            result.push_body(d);
            return result;
        }
        _ => panic!("not a defn"),
    }
}
fn dispatch_syntax(c: &AstVal) -> SyntaxNode {
    let mut result = SyntaxNode::new();
    match c.list_type() {
        Some(s) => match s.as_str() {
            "defn" => {
                result = syntax_defn(c);
            }
            "if" => {
                result = syntax_if(c);
            }
            "let" => {
                result = syntax_let(c);
            }
            "=" => {
                result = syntax_equal(c);
            }
            _ => {
                result = syntax_call(c);
            }
        },
        None => {
            result.set_this(c.clone());
        }
    }
    result
}

pub fn syntax(c: &AstVal) -> SyntaxNode {
    let mut z = dispatch_syntax(&c);
    z.disseminate_context();
    z
}

