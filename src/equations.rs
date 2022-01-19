use std::slice::Iter;

use crate::Writable;
use std::io::Write;

/// A single equation.
///
/// # Examples
///
/// The `Equation` struct is designed to represent one-line equation.
/// Creating one is as simple as using the constructor.
///
/// ```rust
/// # use latex::Equation;
/// let eq = Equation::new("y = mx + c");
/// ```
///
/// For convenience, you can also convert from a `&str` to an `Equation` using
/// `into()`.
///
/// ```rust
/// # use latex::Equation;
/// let eq: Equation = "y = mx + c".into();
/// ```
///
/// You can also assign a `label` to an equation so it can be referenced later.
///
/// ```rust
/// # use latex::Equation;
/// # let mut eq: Equation = "y = mx + c".into();
/// eq.set_label("basic-linear-equation");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Equation {
    /// Text of equation as "y = mx +c"
    pub text: String,
    /// Label of equation
    pub label: Option<String>,
    /// Shows if equation is numbered
    ///
    /// # Note
    /// To use non-numbered equation add `\usepackage{amsmath}` to document.
    pub numbered: bool,
}

impl Equation {
    /// Create a new `Equation`.
    pub fn new<S: AsRef<str>>(src: S) -> Self {
        Equation {
            text: src.as_ref().to_string(),
            label: None,
            numbered: true,
        }
    }

    /// Create an equation which has a label.
    pub fn with_label(label: &str, text: &str) -> Self {
        let mut eq = Self::new(text);
        eq.label = Some(label.to_owned());
        eq
    }

    /// Setter fot label
    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_owned());
    }
}

impl Writable for Equation {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let eq = format!(
            "equation{}",
            match self.numbered {
                true => "",
                false => "*",
            }
        );

        write!(writer, "\\begin{{{}}}\n", &eq)?;
        if let Some(ref label) = self.label {
            write!(writer, "\\label{{{}}}\n", label.as_str())?;
        }
        write!(writer, "{}\n", &self.text)?;
        write!(writer, "\\end{{{}}}\n", &eq)?;

        Ok(())
    }
}

/// An equation for align.
///
/// # Examples
///
/// The `AlignEquation` struct is designed to be added to an `Align` object.
/// Creating one is as simple as using the constructor.
///
/// ```rust
/// # use latex::AlignEquation;
/// let eq = AlignEquation::new("y &= mx + c");
/// ```
///
/// For convenience, you can also convert from a `&str` to an `Equation` using
/// `into()`.
///
/// ```rust
/// # use latex::AlignEquation;
/// let eq: AlignEquation = "y &= mx + c".into();
/// ```
///
/// Result in latex:
///
/// ```tex
/// \begin{equation}
/// y = mx + c
/// \end{equation}
///
/// You can also assign a `label` to an equation so it can be referenced later.
///
/// ```rust
/// # use latex::AlignEquation;
/// # let mut eq: AlignEquation = "y = mx + c".into();
/// eq.label("basic-linear-equation");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AlignEquation {
    /// Text of equation as "y = mx +c"
    pub text: String,
    /// Label of equation
    pub label: Option<String>,
    /// Shows if equation is numbered
    ///
    /// # Note
    /// To use non-numbered equation add `\usepackage{amsmath}` to document.
    pub numbered: bool,
}

impl AlignEquation {
    /// Create a new `AlignEquation`.
    pub fn new<S: AsRef<str>>(src: S) -> Self {
        Self {
            text: src.as_ref().to_string(),
            label: None,
            numbered: true,
        }
    }

    /// Create an equation which has a label.
    pub fn with_label(label: &str, text: &str) -> Self {
        let mut eq = AlignEquation::new(text);
        eq.label = Some(label.to_owned());
        eq
    }

    /// Setter fot label
    pub fn set_label(&mut self, label: &str) {
        self.label = Some(label.to_owned());
    }
}

/// A list of equations to be used in an `align` environment.
///
/// # Note
///
/// Using this environment requires you to include the `amsmath` package in
/// your preamble.
///
///
/// # Examples
///
/// The typical workflow is to create an empty `Align` and then add equations
/// to it.
///
/// ```rust
/// use latex::{Align, AlignEquation};
/// let mut equations = Align::new();
/// equations.push("y &= mx + c")
///          .push(AlignEquation::with_label("emc2", "E &= m c^2"));
/// ```
///
/// When rendered, you should get something like this:
///
/// ```tex
/// \begin{align}
/// y &= mx + c \\
/// E &= m c^2 \\
/// \end{align}
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Align {
    /// Rows with equations
    items: Vec<AlignEquation>,
    /// Label of align
    ///
    /// # Note
    ///
    /// If Align has label, when all equations inside will lost label.
    /// Otherwise, you will have the following error due to compliation:
    /// `Package amsmath Error: Multiple \label's: label 'eq:some' will be lost. \end{align}`
    pub label: Option<String>,
    /// Shows if equation is numbered
    pub numbered: bool,
}

impl Align {
    /// Create an equation which has a label.
    pub fn with_label(label: &str) -> Self {
        Self {
            items: Vec::new(),
            label: Some(label.to_owned()),
            numbered: true,
        }
    }

    /// Create an empty equation list.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            label: None,
            numbered: true,
        }
    }

    /// Iterate over each of this equations in the list.
    pub fn iter(&self) -> Iter<AlignEquation> {
        self.items.iter()
    }

    /// Add an equation to the end of the list.
    pub fn push<E: Into<AlignEquation>>(&mut self, eq: E) -> &mut Self {
        self.items.push(eq.into());
        self
    }
}

fn write_equation<W: Write>(
    writer: &mut W,
    item: &AlignEquation,
    numbered: bool,
) -> Result<(), std::io::Error> {
    if !item.numbered && numbered {
        writeln!(writer, "\\nonumber")?;
    }

    write!(writer, "{} ", item.text)?;

    if let Some(ref label) = item.label {
        write!(writer, "\\label{{{}}} ", label.as_str())?;
    }

    writeln!(writer, "\\\\")?;

    Ok(())
}

impl Writable for Align {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let eq = format!(
            "align{}",
            match self.numbered {
                true => "",
                false => "*",
            }
        );

        write!(writer, "\\begin{{{}}}\n", &eq)?;

        for e in self.items.iter() {
            write_equation(writer, e, self.numbered)?;
        }

        write!(writer, "\\end{{{}}}\n", &eq)?;

        Ok(())
    }
}

impl<'a> From<&'a str> for Equation {
    fn from(other: &'a str) -> Equation {
        Equation::new(other)
    }
}

impl<'a> From<&'a str> for AlignEquation {
    fn from(other: &'a str) -> AlignEquation {
        AlignEquation::new(other)
    }
}

impl<'a> From<&'a str> for Align {
    /// Convert a string into a single equation wrapped in an `align`.
    fn from(other: &'a str) -> Align {
        let mut eq = Align::new();
        eq.push(other);
        eq
    }
}

#[cfg(test)]
mod test {

    use super::{Align, AlignEquation, Equation};
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn render_empty_align() {
        let should_be = "\\begin{align}\n\\end{align}\n";

        let equations = Align::new();
        test_element(&[&equations], should_be);
    }

    #[test]
    fn render_simple_equation() {
        let should_be = "\\begin{equation}\nx &= y + \\sigma\n\\end{equation}\n";

        let eq = Equation::new(r"x &= y + \sigma");
        test_element(&[&eq], should_be);
    }

    #[test]
    fn render_several_equations() {
        let should_be = r"\begin{align}
E &= m c^2 \label{eq:mass-energy-equivalence} \\
y &= m x + c \\
\end{align}
";

        let mut equations = Align::new();

        equations
            .push(AlignEquation::with_label(
                "eq:mass-energy-equivalence",
                "E &= m c^2",
            ))
            .push("y &= m x + c");
        test_element(&[&equations], should_be);
    }

    #[test]
    fn equation_with_label() {
        let should_be =
            "\\begin{equation}\n\\label{eq:mass-energy-equivalence}\nE &= m c^2\n\\end{equation}\n";

        let mut eq = Equation::new("E &= m c^2");
        eq.set_label("eq:mass-energy-equivalence");

        test_element(&[&eq], should_be);
    }

    #[test]
    fn equation_with_no_numbering() {
        let should_be = "\\begin{equation*}\nE &= m c^2\n\\end{equation*}\n";

        let mut eq = Equation::new("E &= m c^2");
        eq.numbered = false;

        test_element(&[&eq], should_be);
    }
}
