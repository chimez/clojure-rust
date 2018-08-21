mod cljtype;
use cljtype::CljVal;
fn f(x: &CljVal) -> CljVal {
    {
        let y = CljVal::CljString("world".to_string());;        if ({ (y == CljVal::CljString("e".to_string())) }) {
            {
                println!("{}", CljVal::CljString("error".to_string()),);
                CljVal::CljNil
            }
        } else {
            {
                println!(
                    "{}{}{}{}",
                    x,
                    CljVal::CljString(" ".to_string()),
                    y,
                    CljVal::CljString("!".to_string()),
                );
                CljVal::CljNil
            }
        }
    }
}
fn main() {
    f(&CljVal::CljString("hello".to_string()));
}
