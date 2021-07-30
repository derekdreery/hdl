//! A HDL simulator/interpreter

use anyhow::{ensure, format_err, Context};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};
use string_interner::DefaultSymbol as Symbol;

use crate::{ast, Interner, Result};

/// An ast that is slightly quicker to traverse when running a program.
pub struct Program {
    interner: Interner,
    chips: HashMap<Symbol, Chip>,
}

impl TryFrom<ast::Program> for Program {
    type Error = anyhow::Error;

    fn try_from(prog: ast::Program) -> Result<Self, Self::Error> {
        // first pass is to gather chips and i/o. Main pass then checks body
        todo!()
    }
}

impl Program {
    /// Evaluate the outputs of a chip with the inputs set to `input`.
    pub fn evaluate(
        &self,
        entry: Symbol,
        input: &HashMap<Symbol, bool>,
    ) -> Result<HashMap<Symbol, bool>> {
        // first check that the inputs are correct.
        let entry_chip = self
            .chips
            .get(&entry)
            .ok_or(format_err!("entry point chip not found"))?;
        ensure!(entry_chip.validate_input(input), "chip input incorrect");
        todo!()
    }

    /// Check that the program is valid (all referenced chips exist,
    /// and their I/O match).
    fn validate(&self) -> Result {
        for (chip_name, chip) in &self.chips {
            chip.validate(self)
                .with_context(|| format!("in chip {}", self.get_str(*chip_name)))?;
        }
        Ok(())
    }

    /// Resolve a symbol in the interner.
    fn get_str(&self, s: Symbol) -> &str {
        self.interner
            .resolve(s)
            .unwrap_or("<unrecognised interned string>")
    }
}

pub struct Chip {
    ins: HashSet<Symbol>,
    outs: HashSet<Symbol>,
    /// In this repr, `parts` is a resolved sequence of steps to calculate the outputs.
    parts: Vec<Part>,
}

impl Chip {
    fn validate(&self, prog: &Program) -> Result {
        for part in &self.parts {
            part.validate(prog)
                .with_context(|| format!("in part {}", prog.get_str(part.chip)))?;
        }
        Ok(())
    }

    fn validate_input<T>(&self, input: &HashMap<Symbol, T>) -> bool {
        if self.ins.len() != input.len() {
            return false;
        }

        self.ins.iter().all(|key| input.get(key).is_some())
    }
}

pub struct Part {
    chip: Symbol,
    name_map: NameMap,
}

impl Part {
    /// Validate this part against a program.
    ///
    /// Involves checking the referenced chip exists and that the parameter match.
    fn validate(&self, prog: &Program) -> Result {
        let target = prog
            .chips
            .get(&self.chip)
            .ok_or_else(|| format_err!("part references chip which does not exist"))?;
        self.validate_params(target)
    }

    /// Check the parameters of this part match the parameters of the chip it references.
    fn validate_params(&self, target: &Chip) -> Result {
        ensure!(
            self.name_map.len() == target.ins.len() + target.outs.len(),
            "parameter names do not match"
        );
        ensure!(
            target
                .ins
                .iter()
                .chain(target.outs.iter())
                .all(|sym| self.name_map.get(sym).is_some()),
            "parameter names do not match"
        );
        Ok(())
    }
}

/// Map of parameters: parameter name -> parameter value.
pub type NameMap = HashMap<Symbol, Symbol>;
