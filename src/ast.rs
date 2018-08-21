use std::fmt;
use std::iter::FromIterator;
use std::slice::Iter;
#[derive(Debug, Clone)]
pub enum AstVal {
    AstNil,
    AstBool(bool),
    AstInt(i32),
    AstFloat(f32),
    AstString(String),
    AstSymbol(String),
    AstKeyword(String),
    AstVec(Vec<AstVal>),
    AstList(Vec<AstVal>),
    AstMap(Vec<(AstVal, AstVal)>),
    AstMeta(Vec<(AstVal, AstVal)>),
    AstCommentLine(String),
}

impl AstVal {
    pub fn new_list() -> AstVal {
        AstVal::AstList(vec![])
    }
    pub fn new_map() -> AstVal {
        AstVal::AstMap(vec![])
    }
    pub fn new_vec() -> AstVal {
        AstVal::AstVec(vec![])
    }
    pub fn new_meta() -> AstVal {
        AstVal::AstMeta(vec![])
    }
    pub fn list_type(&self) -> Option<String> {
        match self {
            AstVal::AstList(v) => match &v[0] {
                AstVal::AstSymbol(s) => Some(s.clone()),
                _ => panic!("not callable list"),
            },
            _ => None,
        }
    }
    pub fn map_to_meta(self) -> AstVal {
        match self {
            AstVal::AstMap(v) => AstVal::AstMeta(v),
            _ => panic!("not map"),
        }
    }
    pub fn insert(&mut self, k: AstVal, v: AstVal) {
        match self {
            AstVal::AstMap(v1) | AstVal::AstMeta(v1) => {
                v1.push((k, v));
            }
            _ => panic!("can't insert"),
        }
    }
    pub fn push(&mut self, x: AstVal) {
        match self {
            AstVal::AstVec(v) | AstVal::AstList(v) => {
                v.push(x);
            }
            _ => panic!("can't push"),
        }
    }
    pub fn pop(&mut self) -> AstVal {
        match self {
            AstVal::AstList(v) => match v.pop() {
                Some(x) => x,
                None => AstVal::AstNil,
            },
            AstVal::AstVec(v) => match v.pop() {
                Some(x) => x,
                None => AstVal::AstNil,
            },

            _ => panic!("can't pop"),
        }
    }
    fn is_atom(&self) -> bool {
        match self {
            AstVal::AstList(_) => false,
            _ => true,
        }
    }
    fn is_nil(&self) -> bool {
        match self {
            AstVal::AstVec(v) => v.is_empty(),
            AstVal::AstNil => true,
            AstVal::AstList(v) => v.is_empty(),
            AstVal::AstMap(v) | AstVal::AstMeta(v) => {
                if v.len() == 0 {
                    true
                } else {
                    false
                }
            }
            AstVal::AstBool(false) => true,
            _ => false,
        }
    }
    pub fn len(&self) -> usize {
        match self {
            AstVal::AstVec(v) | AstVal::AstList(v) => v.len(),
            AstVal::AstMap(v) | AstVal::AstMeta(v) => v.len(),
            _ => panic!("no len method"),
        }
    }

    pub fn is_leaf_list(&self) -> bool {
        match self {
            AstVal::AstList(v) => {
                for i in v {
                    match i {
                        AstVal::AstList(v1) => {
                            if (v1.len() != 0) & (AstVal::AstSymbol(String::from("\'")) == v1[0]) {
                                continue;
                            } else {
                                return false;
                            }
                        }
                        _ => continue,
                    }
                }
                return true;
            }
            _ => return false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            AstVal::AstList(_) => {
                if self.is_leaf_list() {
                    return true;
                } else {
                    return false;
                }
            }
            _ => return true,
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            AstVal::AstList(_) => true,
            _ => false,
        }
    }

    pub fn text(&self) -> String {
        return format!("{}", self);
    }
    pub fn first(&self) -> AstVal {
        match self {
            AstVal::AstList(v) => {
                if v.len() == 0 {
                    return AstVal::AstNil;
                } else {
                    return v[0].clone();
                }
            }
            _ => panic!("no method first"),
        }
    }
    pub fn rest(&self) -> AstVal {
        match self {
            AstVal::AstList(v) => {
                if v.len() > 1 {
                    return AstVal::AstList(v[1..].to_vec());
                } else {
                    return AstVal::AstNil;
                }
            }
            _ => panic!("no method rest"),
        }
    }
    pub fn cons(&self, a: AstVal) -> AstVal {
        match self {
            AstVal::AstList(v) => {
                let mut v = v.clone();
                let mut va = vec![a];
                va.append(&mut v);
                return AstVal::AstList(va);
            }
            _ => panic!("no method cons"),
        }
    }
    pub fn cons_mut(&mut self, a: AstVal) {
        let s = self.clone();
        match s {
            AstVal::AstList(mut v) => {
                let mut va = vec![a];
                va.append(&mut v);
                *self = AstVal::AstList(va);
            }
            _ => panic!("no method cons_mut"),
        }
    }
    pub fn iter(&self) -> Iter<AstVal> {
        match self {
            AstVal::AstList(v) => v.iter(),
            _ => panic!("no method iter"),
        }
    }
}

impl Iterator for AstVal {
    type Item = AstVal;
    fn next(&mut self) -> Option<AstVal> {
        match self {
            AstVal::AstList(_) => {
                let first = self.first();
                if self.is_nil() {
                    return None;
                }
                let rest = self.rest();
                *self = rest;
                return Some(first);
            }
            AstVal::AstNil => None,
            _ => panic!("no method next"),
        }
    }
}
impl FromIterator<AstVal> for AstVal {
    fn from_iter<I: IntoIterator<Item = AstVal>>(iter: I) -> Self {
        let mut l = AstVal::new_list();
        for i in iter {
            l.cons_mut(i);
        }
        l
    }
}

impl PartialEq for AstVal {
    fn eq(&self, other: &AstVal) -> bool {
        match (self, other) {
            (AstVal::AstNil, AstVal::AstNil) => true,
            (AstVal::AstNil, AstVal::AstList(v)) => v.is_empty(),
            (AstVal::AstList(v), AstVal::AstNil) => v.is_empty(),
            (AstVal::AstString(s1), AstVal::AstString(s2)) => s1 == s2,
            (AstVal::AstSymbol(s1), AstVal::AstSymbol(s2)) => s1 == s2,
            (AstVal::AstBool(b1), AstVal::AstBool(b2)) => b1 == b2,
            (AstVal::AstInt(i1), AstVal::AstInt(i2)) => i1 == i2,
            (AstVal::AstFloat(f1), AstVal::AstFloat(f2)) => f1 == f2,
            (AstVal::AstNil, AstVal::AstVec(_)) => other.is_nil(),
            (AstVal::AstVec(_), AstVal::AstNil) => self.is_nil(),
            (AstVal::AstMap(_), AstVal::AstNil) => self.is_nil(),
            (AstVal::AstNil, AstVal::AstMap(_)) => other.is_nil(),
            (AstVal::AstMeta(_), AstVal::AstNil) => self.is_nil(),
            (AstVal::AstNil, AstVal::AstMeta(_)) => other.is_nil(),
            _ => false,
        }
    }
}

impl fmt::Display for AstVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstVal::AstBool(s) => write!(f, "{}", s),
            AstVal::AstInt(s) => write!(f, "{}", s),
            AstVal::AstFloat(s) => write!(f, "{}", s),
            AstVal::AstString(s) => write!(f, "{}", s),
            AstVal::AstSymbol(s) => write!(f, "{}", s),
            AstVal::AstKeyword(s) => write!(f, "{}", s),
            AstVal::AstCommentLine(s) => write!(f, "{}", s),
            _ => panic!("can't do that"),
        }
    }
}

#[derive(Debug)]
enum AstErr {
    ErrString(&'static str),
    ErrAstVal(AstVal),
}

type AstResult = Result<AstVal, AstErr>;

macro_rules! cljlist {
    ($($x:expr),*) => {
        {
            let mut l: Vec<AstVal> = Vec::new();
            $( l.push($x); )*
                AstVal::AstList(l)
        }}
    ;
}
