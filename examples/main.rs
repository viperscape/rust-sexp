extern crate sexp;

fn main () {
    let sexp = r#"((data "quoted data" 123 4.5)
 (data (!@# (4.5) "(more" "data)")))"#;

    let psexp = sexp::parse(sexp);

    println!("{:?}",psexp);

    match psexp {
        Some(parsed) => println!("{:?}",sexp::write_sexp(&parsed)),
        None => println!("none")
    }
}
