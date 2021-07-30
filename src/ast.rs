use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use string_interner::{DefaultSymbol as Symbol, StringInterner};

use crate::Interner;

macro_rules! delim {
    ($iter:expr, $this:expr, $action_item:expr, $action_delim:expr) => {{
        let mut iter = $iter;
        let itm = iter.next();
        if let Some(itm_) = itm {
            $action_item($this, itm_)?;
            for itm__ in iter {
                $action_delim($this)?;
                $action_item($this, itm__)?;
            }
        }
    }};
}

/// This is the ast as directly parsed by the parser. It is then converted into
/// a form for more efficient evaluation.
#[derive(Debug)]
pub struct Program {
    pub chips: HashMap<Symbol, Chip>,
}

#[derive(Debug)]
pub struct Chip {
    pub name: Symbol,
    pub ins: HashSet<Symbol>,
    pub outs: HashSet<Symbol>,
    /// This is unordered until the program is validated. After it is in a valid order of operations,
    /// if validation succeeded.
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
    pub chip_name: Symbol,
    /// in param name -> var name.
    pub name_map: HashMap<Symbol, Symbol>,
}

pub trait Visitor {
    type Output;
    fn visit_program(&mut self, program: &Program) -> Self::Output;
    fn visit_chip(&mut self, chip: &Chip) -> Self::Output;
    fn visit_part(&mut self, part: &Part) -> Self::Output;
}

// Displaying

impl Program {
    pub fn display<'a>(&'a self, interner: &'a StringInterner) -> impl fmt::Display + 'a {
        struct ProgramDisplay<'a>(&'a StringInterner, &'a Program);
        impl<'a> fmt::Display for ProgramDisplay<'a> {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                Display {
                    interner: self.0,
                    fmt,
                }
                .visit_program(self.1)
            }
        }
        ProgramDisplay(interner, self)
    }
}

struct Display<'a, 'b> {
    interner: &'a StringInterner,
    fmt: &'a mut fmt::Formatter<'b>,
}

impl Display<'_, '_> {
    fn write_symbol(&mut self, s: Symbol) -> fmt::Result {
        self.fmt
            .write_str(self.interner.resolve(s).ok_or(fmt::Error)?)
    }
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.fmt.write_str(s)
    }
}

impl Visitor for Display<'_, '_> {
    type Output = fmt::Result;

    fn visit_program(&mut self, program: &Program) -> Self::Output {
        delim!(
            program.chips.values(),
            self,
            |this: &mut Self, chip| this.visit_chip(chip),
            |this: &mut Self| { this.fmt.write_str("\n\n") }
        );
        Ok(())
    }

    fn visit_chip(&mut self, chip: &Chip) -> Self::Output {
        write!(
            self.fmt,
            "CHIP {} {{\n  IN ",
            self.interner.resolve(chip.name).ok_or(fmt::Error)?
        )?;
        delim!(
            chip.ins.iter().copied(),
            self,
            |this: &mut Self, in_| this.write_symbol(in_),
            |this: &mut Self| this.write_str(", ")
        );
        write!(self.fmt, ";\n  OUT ")?;
        delim!(
            chip.outs.iter().copied(),
            self,
            |this: &mut Self, out| this.write_symbol(out),
            |this: &mut Self| this.write_str(", ")
        );
        write!(self.fmt, ";\nPARTS:\n  ")?;
        delim!(
            chip.parts.iter(),
            self,
            |this: &mut Self, part| this.visit_part(part),
            |this: &mut Self| this.write_str("\n  ")
        );
        write!(self.fmt, "\n}}")
    }

    fn visit_part(&mut self, part: &Part) -> Self::Output {
        self.write_symbol(part.chip_name)?;
        self.write_str("(")?;
        delim!(
            part.name_map.iter(),
            self,
            |this: &mut Self, (key, value): (&Symbol, &Symbol)| {
                this.write_symbol(*key)?;
                this.write_str("=")?;
                this.write_symbol(*value)
            },
            |this: &mut Self| this.write_str(", ")
        );
        write!(self.fmt, ")")
    }
}

/// Validating

struct Validator;

impl Visitor for Validator {
    type Output = ();

    fn visit_program(&mut self, program: &Program) -> Self::Output {
        todo!()
    }

    fn visit_chip(&mut self, chip: &Chip) -> Self::Output {
        todo!()
    }

    fn visit_part(&mut self, part: &Part) -> Self::Output {
        todo!()
    }
}
