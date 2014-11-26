/*#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;*/

extern crate regex;
use regex::Regex;

#[deriving(Show)]
enum Exp {
    Num(i32),
    FNum(f64), 
    Sym(String),
    QSym(String),
    Sexp(Box<Vec<Exp>>),
}

fn parse_sexp<'a> (re:Regex, reiter:&mut regex::FindCaptures, sexp:&'a str) -> Vec<Exp> {
    let mut vs: Vec<Exp> = Vec::new();
    
    'sexp: loop {
        let n = reiter.next();
        match n {
            Some(cap) => { //captures

                let lp =  cap.name("lp"); 
                let rp =  cap.name("rp"); 
                let s =  cap.name("s");
                let qs =  cap.name("qs"); 
                let inum =  cap.name("inum"); 
                let fnum =  cap.name("fnum");

                if lp != "" {
                    let rvs = parse_sexp(re.clone(), reiter, sexp);
                    vs.push(Sexp(box rvs));
                    continue;
                }
                else if s != "" {vs.push(Sym(s.to_string()));continue;}

                
                else if qs != "" {vs.push(QSym(qs.to_string()));continue;}

                
                else if inum != "" {vs.push(Num(from_str::<i32>(inum).unwrap()));continue;}

                else if fnum != "" {vs.push(FNum(from_str::<f64>(fnum).unwrap()));continue;}                

                else if rp != "" {break 'sexp;}

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
