use std::fmt;
use std::iter::FromIterator;
use std::slice::Iter;
#[derive(Debug, Clone)]
pub enum CljVal {
    CljNil,
    CljBool(bool),
    CljInt(i32),
    CljFloat(f32),
    CljString(String),
    CljSymbol(String),
    CljKeyword(String),
    CljVec(Vec<CljVal>),
    CljList(Vec<CljVal>),
    CljMap(Vec<(CljVal, CljVal)>),
    CljMeta(Vec<(CljVal, CljVal)>),
    CljCommentLine(String),
}

impl CljVal {
    pub fn new_list() -> CljVal {
        CljVal::CljList(vec![])
    }
    pub fn new_map() -> CljVal {
        CljVal::CljMap(vec![])
    }
    pub fn new_vec() -> CljVal {
        CljVal::CljVec(vec![])
    }
    pub fn new_meta() -> CljVal {
        CljVal::CljMeta(vec![])
    }
    pub fn list_type(&self) -> Option<String> {
        match self {
            CljVal::CljList(v) => match &v[0] {
                CljVal::CljSymbol(s) => Some(s.clone()),
                _ => panic!("not callable list"),
            },
            _ => None,
        }
    }
    pub fn map_to_meta(self) -> CljVal {
        match self {
            CljVal::CljMap(v) => CljVal::CljMeta(v),
            _ => panic!("not map"),
        }
    }
    pub fn insert(&mut self, k: CljVal, v: CljVal) {
        match self {
            CljVal::CljMap(v1) | CljVal::CljMeta(v1) => {
                v1.push((k, v));
            }
            _ => panic!("can't insert"),
        }
    }
    pub fn push(&mut self, x: CljVal) {
        match self {
            CljVal::CljVec(v) | CljVal::CljList(v) => {
                v.push(x);
            }
            _ => panic!("can't push"),
        }
    }
    pub fn pop(&mut self) -> CljVal {
        match self {
            CljVal::CljList(v) => match v.pop() {
                Some(x) => x,
                None => CljVal::CljNil,
            },
            CljVal::CljVec(v) => match v.pop() {
                Some(x) => x,
                None => CljVal::CljNil,
            },

            _ => panic!("can't pop"),
        }
    }
    fn is_atom(&self) -> bool {
        match self {
            CljVal::CljList(_) => false,
            _ => true,
        }
    }
    fn is_nil(&self) -> bool {
        match self {
            CljVal::CljVec(v) => v.is_empty(),
            CljVal::CljNil => true,
            CljVal::CljList(v) => v.is_empty(),
            CljVal::CljMap(v) | CljVal::CljMeta(v) => {
                if v.len() == 0 {
                    true
                } else {
                    false
                }
            }
            CljVal::CljBool(false) => true,
            _ => false,
        }
    }
    pub fn len(&self) -> usize {
        match self {
            CljVal::CljVec(v) | CljVal::CljList(v) => v.len(),
            CljVal::CljMap(v) | CljVal::CljMeta(v) => v.len(),
            _ => panic!("no len method"),
        }
    }

    pub fn is_leaf_list(&self) -> bool {
        match self {
            CljVal::CljList(v) => {
                for i in v {
                    match i {
                        CljVal::CljList(v1) => {
                            if (v1.len() != 0) & (CljVal::CljSymbol(String::from("\'")) == v1[0]) {
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
            CljVal::CljList(_) => {
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
            CljVal::CljList(_) => true,
            _ => false,
        }
    }

    pub fn text(&self) -> String {
        return format!("{}", self);
    }
    pub fn first(&self) -> CljVal {
        match self {
            CljVal::CljList(v) => {
                if v.len() == 0 {
                    return CljVal::CljNil;
                } else {
                    return v[0].clone();
                }
            }
            _ => panic!("no method first"),
        }
    }
    pub fn rest(&self) -> CljVal {
        match self {
            CljVal::CljList(v) => {
                if v.len() > 1 {
                    return CljVal::CljList(v[1..].to_vec());
                } else {
                    return CljVal::CljNil;
                }
            }
            _ => panic!("no method rest"),
        }
    }
    pub fn cons(&self, a: CljVal) -> CljVal {
        match self {
            CljVal::CljList(v) => {
                let mut v = v.clone();
                let mut va = vec![a];
                va.append(&mut v);
                return CljVal::CljList(va);
            }
            _ => panic!("no method cons"),
        }
    }
    pub fn cons_mut(&mut self, a: CljVal) {
        let mut s = self.clone();
        match s {
            CljVal::CljList(mut v) => {
                let mut va = vec![a];
                va.append(&mut v);
                *self = CljVal::CljList(va);
            }
            _ => panic!("no method cons_mut"),
        }
    }
    pub fn iter(&self) -> Iter<CljVal> {
        match self {
            CljVal::CljList(v) => v.iter(),
            _ => panic!("no method iter"),
        }
    }
}

impl Iterator for CljVal {
    type Item = CljVal;
    fn next(&mut self) -> Option<CljVal> {
        match self {
            CljVal::CljList(_) => {
                let first = self.first();
                if self.is_nil() {
                    return None;
                }
                let rest = self.rest();
                *self = rest;
                return Some(first);
            }
            CljVal::CljNil => None,
            _ => panic!("no method next"),
        }
    }
}
impl FromIterator<CljVal> for CljVal {
    fn from_iter<I: IntoIterator<Item = CljVal>>(iter: I) -> Self {
        let mut l = CljVal::new_list();
        for i in iter {
            l.cons_mut(i);
        }
        l
    }
}

impl PartialEq for CljVal {
    fn eq(&self, other: &CljVal) -> bool {
        match (self, other) {
            (CljVal::CljNil, CljVal::CljNil) => true,
            (CljVal::CljNil, CljVal::CljList(v)) => v.is_empty(),
            (CljVal::CljList(v), CljVal::CljNil) => v.is_empty(),
            (CljVal::CljString(s1), CljVal::CljString(s2)) => s1 == s2,
            (CljVal::CljSymbol(s1), CljVal::CljSymbol(s2)) => s1 == s2,
            (CljVal::CljBool(b1), CljVal::CljBool(b2)) => b1 == b2,
            (CljVal::CljInt(i1), CljVal::CljInt(i2)) => i1 == i2,
            (CljVal::CljFloat(f1), CljVal::CljFloat(f2)) => f1 == f2,
            (CljVal::CljNil, CljVal::CljVec(_)) => other.is_nil(),
            (CljVal::CljVec(_), CljVal::CljNil) => self.is_nil(),
            (CljVal::CljMap(_), CljVal::CljNil) => self.is_nil(),
            (CljVal::CljNil, CljVal::CljMap(_)) => other.is_nil(),
            (CljVal::CljMeta(_), CljVal::CljNil) => self.is_nil(),
            (CljVal::CljNil, CljVal::CljMeta(_)) => other.is_nil(),
            _ => false,
        }
    }
}

impl fmt::Display for CljVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CljVal::CljBool(s) => write!(f, "{}", s),
            CljVal::CljInt(s) => write!(f, "{}", s),
            CljVal::CljFloat(s) => write!(f, "{}", s),
            CljVal::CljString(s) => write!(f, "{}", s),
            CljVal::CljSymbol(s) => write!(f, "{}", s),
            CljVal::CljKeyword(s) => write!(f, "{}", s),
            CljVal::CljCommentLine(s) => write!(f, "{}", s),
            _ => panic!("can't do that"),
        }
    }
}

#[derive(Debug)]
enum CljErr {
    ErrString(&'static str),
    ErrCljVal(CljVal),
}

type CljResult = Result<CljVal, CljErr>;

macro_rules! cljlist {
    ($($x:expr),*) => {
        {
            let mut l: Vec<CljVal> = Vec::new();
            $( l.push($x); )*
                CljVal::CljList(l)
        }}
    ;
}
