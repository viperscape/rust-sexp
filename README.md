# Parsing S-Exp in Rust

from:
```
((data "quoted data" 123 4.5)
 (data (!@# (4.5) "(more" "data)")))
 ```

to:
``` rust
Some([Sexp([Sexp([Sym(data), QSym("quoted data"), INum(123), FNum(4.5)]), Sexp([Sym(data), Sexp([Sym(!@#), Sexp([FNum(4.5)]), QSym("(more"), QSym("data)")])])])])
```

and back:
```
((data "quoted data" 123 4.5) (data (!@# (4.5) "(more" "data)")))
```
