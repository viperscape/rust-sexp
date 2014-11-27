/*#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros; //not sure if I should use regex! macro, haven't made up mind
*/

extern crate regex;
use regex::Regex;

#[deriving(Show)]
enum Exp {
    INum(i32),
    FNum(f64), 
    Sym(String),
    QSym(String),
    Sexp(Box<Vec<Exp>>),
}

fn parse_sexp<'a> (re:Regex, reiter:&mut regex::FindCaptures, sexp:&'a str) -> Vec<Exp> {
    let mut vs: Vec<Exp> = Vec::new();
    
    loop {
        match reiter.next() {
            Some(cap) => { //captures

                let lp =  cap.name("lp"); 
                let rp =  cap.name("rp"); 
                let s =  cap.name("s");
                let qs =  cap.name("qs"); 
                let inum =  cap.name("inum"); 
                let fnum =  cap.name("fnum");

                if lp != "" {
                    let rvs = parse_sexp(re.clone(), reiter, sexp);
                    vs.push(Exp::Sexp(box rvs));
                    continue;
                }
                else if s != "" {vs.push(Exp::Sym(s.to_string()));}

                
                else if qs != "" {vs.push(Exp::QSym(qs.to_string()));}

                
                else if inum != "" {vs.push(Exp::INum(from_str::<i32>(inum).unwrap()));}

                else if fnum != "" {vs.push(Exp::FNum(from_str::<f64>(fnum).unwrap()));}                

                else if rp != "" {break;}

                else {panic!("unknown token! {}",cap.at(0));}
            },
            None => break
        };
    }                                            
    vs
}

fn main () {
    let sxre = r#"(?P<lp>\()|(?P<rp>\))|(?P<qs>".*?"+)|(?P<num>(?P<fnum>-?\d+\.\d+)|(?P<inum>-?\d+))|(?P<s>[[:alnum:]+|[:punct:]+|[:graph:]+]+)"#;

    let re = match Regex::new(sxre){
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    let sexp = r#"((data "quoted data" 123 4.5)
 (data (!@# (4.5) "(more" "data)")))"#;

    let mut reiter = re.captures_iter(sexp);
    println!("{}",parse_sexp(re.clone(), &mut reiter,sexp));
}
