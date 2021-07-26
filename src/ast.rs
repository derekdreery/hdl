use std::fmt;
use string_interner::{DefaultSymbol as Symbol, StringInterner};

#[derive(Debug)]
pub struct Program {
    pub chips: Vec<Chip>,
}

impl Program {
    pub fn display<'a>(&'a self, interner: &'a StringInterner) -> impl fmt::Display + 'a {
        Display {
            inner: self,
            interner,
        }
    }
}
#[derive(Debug)]
pub struct Chip {
    pub name: Symbol,
    pub ins: Vec<Symbol>,
    pub outs: Vec<Symbol>,
    pub parts: Vec<Part>,
}

impl Chip {
    pub fn display<'a>(&'a self, interner: &'a StringInterner) -> impl fmt::Display + 'a {
        Display {
            inner: self,
            interner,
        }
    }
}

#[derive(Debug)]
pub struct Part {
    pub chip_name: Symbol,
    pub name_maps: Vec<NameMap>,
}

#[derive(Debug)]
pub struct NameMap {
    pub key: Symbol,
    pub value: Symbol,
}

// Displaying

#[derive(Copy, Clone)]
struct Display<'a, T> {
    inner: T,
    interner: &'a StringInterner,
}

impl<'a, T: Copy> Display<'a, T> {
    /// Map the contained value leaving the interner untouched.
    fn map<U>(&self, mut f: impl FnMut(T) -> U) -> Display<'a, U> {
        Display {
            inner: f(self.inner),
            interner: self.interner,
        }
    }

    /// Shortcut for mapping the field, resolving symbol, then converting None to fmt::Error.
    fn resolve_field<'b>(&'b self, f: impl Fn(T) -> &'b Symbol) -> Result<&'a str, fmt::Error> {
        self.map(f).resolve().ok_or(fmt::Error)
    }
}

impl<'a> Display<'a, &Symbol> {
    /// Resolve the symbol using the interner.
    fn resolve(self) -> Option<&'a str> {
        self.interner.resolve(*self.inner)
    }
}

impl<'a, T> Display<'a, T>
where
    T: IntoIterator + Copy,
{
    fn iter(&'a self) -> impl Iterator<Item = Display<'a, <T as IntoIterator>::Item>> + 'a {
        self.inner.into_iter().map(move |item| Display {
            inner: item,
            interner: self.interner,
        })
    }
}

impl fmt::Display for Display<'_, &Program> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_list(self.map(|prog| &prog.chips).iter(), "\n\n", f)
    }
}

impl fmt::Display for Display<'_, &Chip> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.resolve_field(|chip| &chip.name)?;
        write!(f, "CHIP {} {{\n  IN ", name)?;
        write_list(self.map(|chip| &chip.ins).iter(), ", ", f)?;
        write!(f, ";\n  OUT ")?;
        write_list(self.map(|chip| &chip.outs).iter(), ", ", f)?;
        write!(f, ";\nPARTS:\n  ")?;
        write_list(self.map(|t| &t.parts).iter(), "\n  ", f)?;
        write!(f, "\n}}")
    }
}

impl fmt::Display for Display<'_, &Part> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chip_name = self.resolve_field(|part| &part.chip_name)?;
        write!(f, "{}(", chip_name)?;
        write_list(self.map(|part| &part.name_maps).iter(), ", ", f)?;
        write!(f, ")")
    }
}

impl fmt::Display for Display<'_, &NameMap> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = self.resolve_field(|map| &map.key)?;
        let value = self.resolve_field(|map| &map.value)?;
        write!(f, "{}={}", key, value)
    }
}

impl fmt::Display for Display<'_, &Symbol> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.resolve().ok_or(fmt::Error)?)
    }
}

fn write_list<'a, T>(
    ts: impl IntoIterator<Item = Display<'a, T>>,
    sep: &str,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
    Display<'a, T>: fmt::Display,
{
    let mut ts = ts.into_iter();
    write!(f, "{}", ts.next().ok_or(fmt::Error)?)?;
    for t in ts {
        write!(f, "{}{}", sep, t)?;
    }
    Ok(())
}
