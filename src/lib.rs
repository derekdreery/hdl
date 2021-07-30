#[macro_use]
extern crate lalrpop_util;

mod ast;
mod interpreter;

pub use interpreter::Program;
use std::sync::Arc;
use string_interner::StringInterner;

lalrpop_mod!(pub hdl);

pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

pub type Interner = Arc<StringInterner>;

#[cfg(test)]
mod tests {
    use string_interner::StringInterner;

    use super::hdl;

    #[test]
    fn it_works() {
        let parser = hdl::ProgramParser::new();
        let input = r#"
CHIP Eq3 {
    IN a, b, c;
    OUT out;
PARTS:
    Xor(a=a, b=b, out=neq1);
    Xor(a=b, b=c, out=neq2);
    Or(a=neq1, b=neq2, out=outOr);
    Not(in=outOr, out=out);
}

CHIP Eq2 {
    IN a, b;
    OUT out;
PARTS:
    Xor(a=a, b=b, out=out);
}
"#;
        let mut interner = StringInterner::new();
        let ast = parser.parse(&mut interner, input).unwrap();
        panic!("{}", ast.display(&interner));
    }
}
