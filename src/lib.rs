/*#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros; //not sure if I should use regex! macro, haven't made up mind
*/

#[cfg(test)]
extern crate test;
extern crate regex;

use regex::Regex;
#[cfg(test)]
use test::Bencher;


#[derive(Debug)]
pub enum Exp {
    INum(i32),
    FNum(f64),
    Sym(String),
    QSym(String),
    Sexp(Vec<Exp>),
}

pub fn parse_sexp<'a> (re:Regex, sexp:&'a str) -> Option<Vec<Exp>> {
    let mut exp_stack = vec![];
    exp_stack.push(vec![]);

    for cap in re.captures_iter(sexp) {
        if cap.name("lp").is_some() {
            exp_stack.push(vec![]);
        } else if cap.name("rp").is_some() {
            let exprs = exp_stack.pop();
            let scope = exp_stack.last_mut().expect("Requires scope");
            scope.push(Exp::Sexp(exprs.unwrap()));
        } else {
            let s =  cap.name("s");
            let qs =  cap.name("qs");
            let inum =  cap.name("inum");
            let fnum =  cap.name("fnum");

            let scope = exp_stack.last_mut().expect("Requires scope");
            if s.is_some() {scope.push(Exp::Sym(String::from_str(s.unwrap())));}
            else if qs.is_some() {scope.push(Exp::QSym(String::from_str(qs.unwrap())));}
            else if inum.is_some() {scope.push(Exp::INum(inum.unwrap().parse().unwrap()));}
            else if fnum.is_some() {scope.push(Exp::FNum(fnum.unwrap().parse().unwrap()));}
            else {panic!("unknown token! {:?}",cap.at(0));}
        }
    }

    exp_stack.pop()
}

pub fn parse(expr: &str) -> Option<Vec<Exp>> {
    let sxre = r#"(?P<lp>\()|(?P<rp>\))|(?P<qs>".*?"+)|(?P<num>(?P<fnum>-?\d+\.\d+)|(?P<inum>-?\d+))|(?P<s>[[:alnum:]+|[:punct:]+|[:graph:]+]+)"#;

    let re = match Regex::new(sxre){
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    parse_sexp(re.clone(), expr)
}



pub fn write_sexp (vsexp: &Vec<Exp>) -> String {
    let mut ws: String = String::new();
    for n in vsexp.iter() {
        let r = match *n {
            Exp::INum(ref i) => i.to_string(),
            Exp::FNum(ref f) => f.to_string(),
            Exp::QSym(ref qs) => qs.to_string(),
            Exp::Sym(ref s) => s.to_string(),
            Exp::Sexp(ref sexp) => "(".to_string() + write_sexp(sexp).as_slice() + ")",
        };
        
        let el = r+" "; //place a space between all elements in list
        ws.push_str(el.as_slice());
    }
    let new_len = ws.len() - 1; //truncate final space
    ws.truncate(new_len);
    ws
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
