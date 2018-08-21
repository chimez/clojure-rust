let s1 = String::from(
                     ";asd
((p ^\"sd\" 341 ;assss
jj ^{:a \"gg\"} (- 23 g)
:ad?)
+ 1 [1 {asd ee} 2 3] 2 (- 2 3)
'(a sd @(ss) d)
@oi );(+ 3 3)\n
    ;asd",
                     );
let s2 = String::from(
                     "
; comment line
(first list)
((double list) (+ 1 2))
(string \"hello\")
(int 1 2 3 4)
(float 1. 3.4 -9e3 7e-10)
(keyword :keyword)
(metadata ^\"string\" ^symbol ^{:key val})
(vector [1 2 3 (list)])
(map {a b c d})
(deref @a)
(strange name star* question? exclamation! a->b *stars*)
(special macro vs negative number -> --> -10)
(quote '(a quote))
(comment line in list
   ; comment line
   (+ 2 3))
(新玩法 中文编程)

",
                     );

let mut r1 = RawReader::new(s1);
println!("{:#?}", r1.read());

let mut r2 = RawReader::new(s2);
println!("{:#?}", r2.read());
