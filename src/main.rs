/*#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros; //not sure if I should use regex! macro, haven't made up mind
*/

#[cfg(test)]
extern crate test;
extern crate regex;

use regex::Regex;
#[cfg(test)]
use test::Bencher;


#[deriving(Show)]
enum Exp {
    INum(i32),
    FNum(f64),
    Sym(String),
    QSym(String),
    Sexp(Vec<Exp>),
}

fn parse_sexp<'a> (re:Regex, sexp:&'a str) -> Option<Vec<Exp>> {
    let mut exp_stack = vec![];
    exp_stack.push(vec![]);

    for cap in re.captures_iter(sexp) {
        if !cap.name("lp").is_empty() {
            exp_stack.push(vec![]);
        } else if !cap.name("rp").is_empty() {
            let exprs = exp_stack.pop();
            let scope = exp_stack.last_mut().expect("Requires scope");
            scope.push(Exp::Sexp(exprs.unwrap()));
        } else {
            let s =  cap.name("s");
            let qs =  cap.name("qs");
            let inum =  cap.name("inum");
            let fnum =  cap.name("fnum");

            let scope = exp_stack.last_mut().expect("Requires scope");
            if !s.is_empty() {scope.push(Exp::Sym(s.to_string()));}
            else if !qs.is_empty() {scope.push(Exp::QSym(qs.to_string()));}
            else if !inum.is_empty() {scope.push(Exp::INum(from_str::<i32>(inum).unwrap()));}
            else if !fnum.is_empty() {scope.push(Exp::FNum(from_str::<f64>(fnum).unwrap()));}
            else {panic!("unknown token! {}",cap.at(0));}
        }
    }

    exp_stack.pop()
}

fn parse(expr: &str) -> Option<Vec<Exp>> {
    let sxre = r#"(?P<lp>\()|(?P<rp>\))|(?P<qs>".*?"+)|(?P<num>(?P<fnum>-?\d+\.\d+)|(?P<inum>-?\d+))|(?P<s>[[:alnum:]+|[:punct:]+|[:graph:]+]+)"#;

    let re = match Regex::new(sxre){
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    parse_sexp(re.clone(), expr)
}

fn main () {
    let sexp = r#"((data "quoted data" 123 4.5)
 (data (!@# (4.5) "(more" "data)")))"#;

    println!("{}",parse(sexp));
}

#[bench]
fn bench_small(b: &mut Bencher) {
    let sexp = r#"((data "quoted data" 123 4.5)
 (data (!@# (4.5) "(more" "data)")))"#;
    b.bytes = sexp.len() as u64;
    b.iter(|| {
        parse(sexp);
    });
}

#[bench]
fn bench_medium(b: &mut Bencher) {
    let sexp = r#"(((S) (NP VP))
        ((VP) (V))
        ((VP) (V NP))
        ((V) died)
        ((V) employed)
        ((NP) nurses)
        ((NP) patients)
        ((NP) Medicenter)
        ((NP) \"Dr Chan\"))
    "#;
    b.bytes = sexp.len() as u64;
    b.iter(|| {
        parse(sexp);
    });
}
