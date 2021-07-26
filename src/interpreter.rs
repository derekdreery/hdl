//! A HDL simulator/interpreter

use anyhow::{ensure, format_err};
use std::collections::{HashMap, HashSet};
use string_interner::DefaultSymbol as Symbol;

use crate::Result;

pub struct Program {
    chips: HashMap<Symbol, Chip>,
}

impl Program {
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
}

pub struct Chip {
    ins: HashSet<Symbol>,
    outs: HashSet<Symbol>,
    /// In this repr, `parts` is a resolved sequence of steps to calculate the outputs.
    parts: Vec<Part>,
}

impl Chip {
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

pub type NameMap = HashMap<Symbol, Symbol>;
