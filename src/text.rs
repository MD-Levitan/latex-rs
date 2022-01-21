use crate::Writable;
use std::slice::Iter;

/// `Text` represents collection of elements like bold text and other without new line.
///
/// # Note
///
/// A text in previous version was `Paragraph`.
///
/// # Examples
///
/// Like most of the other types in this crate, the standard workflow is to
/// create an empty `Text` then incrementally add bits to it using method
/// chaining and the `push()` method.
///
/// ```rust
/// use latex::{Text, TextElement};
///
/// let mut p = Text::new();
/// p.push("Hello ")
///  .push(TextElement::italic("World"))
///  .push("!")
///  .push(" Here is an equation ")
///  .push(TextElement::InlineMath("y = mx + c".to_string()))
///  .push(".");
/// ```
///
/// The above Text would get rendered to something like this:
///
/// ```tex
/// Hello \textit{World}! Here is an equation $y = mx + c$.
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Text {
    /// A list of `TextElements` which make up the Text's contents.
    pub elements: Vec<TextElement>,
}

impl Text {
    /// Create a new Text.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a `TextElement` to the `Text`.
    pub fn push<P>(&mut self, elem: P) -> &mut Self
    where
        P: Into<TextElement>,
    {
        self.elements.push(elem.into());
        self
    }

    /// Add some raw text to the Text.
    pub fn push_text(&mut self, text: &str) -> &mut Self {
        self.push(TextElement::Plain(text.to_string()))
    }

    /// Iterate over the `TextElement`s in this `Text`.
    pub fn iter(&self) -> Iter<TextElement> {
        self.elements.iter()
    }
}

impl Writable for Text {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        for elem in self.iter() {
            elem.write_to(writer)?;
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for Text {
    fn from(other: &'a str) -> Text {
        let mut text = Text::new();
        text.push_text(other);
        text
    }
}

/// The various Text elements.
///
/// For convenience, you can convert from a string to a `TextElement`
/// using `into()`.
#[derive(Clone, Debug, PartialEq)]
pub enum TextElement {
    /// A plain string.
    Plain(String),
    /// Bolded text.
    Bold(Box<TextElement>),
    /// Italicized text.
    Italic(Box<TextElement>),
    /// Hyperlink - (description, link)
    Link((String, String)),
    /// An inline mathematical expression.
    InlineMath(String),
}

impl TextElement {
    /// Convenience method for wrapping a `TextElement` in an italics tag.
    pub fn italic<E>(elem: E) -> TextElement
    where
        E: Into<TextElement>,
    {
        TextElement::Italic(Box::new(elem.into()))
    }

    /// Convenience method for wrapping a `TextElement` in a bold tag.
    pub fn bold<E>(elem: E) -> TextElement
    where
        E: Into<TextElement>,
    {
        TextElement::Bold(Box::new(elem.into()))
    }
}

impl Default for TextElement {
    fn default() -> Self {
        TextElement::from("")
    }
}

impl<'a> From<&'a str> for TextElement {
    fn from(other: &'a str) -> Self {
        TextElement::Plain(other.to_string())
    }
}

impl Writable for TextElement {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        match *self {
            TextElement::Plain(ref s) => write!(writer, "{}", s)?,
            TextElement::Link(ref s) => write!(writer, "\\href{{{}}}{{{}}}", s.0, s.1)?,
            TextElement::InlineMath(ref s) => write!(writer, "${}$", s)?,
            TextElement::Bold(ref e) => write!(writer, r"\textbf{{{}}}", e)?,
            TextElement::Italic(ref e) => {
                write!(writer, r"\textit{{{}}}", e)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for TextElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextElement::Plain(ref s) => write!(f, "{}", s)?,
            TextElement::Link(ref s) => write!(f, "\\href{{{}}}{{{}}}", s.0, s.1)?,
            TextElement::InlineMath(ref s) => write!(f, "${}$", s)?,
            TextElement::Bold(ref e) => write!(f, r"\textbf{{{}}}", e)?,
            TextElement::Italic(ref e) => {
                write!(f, r"\textit{{{}}}", e)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.elements.iter() {
            match *item {
                TextElement::Plain(ref s) => write!(f, "{}", s)?,
                TextElement::Link(ref s) => write!(f, "\\href{{{}}}{{{}}}", s.0, s.1)?,
                TextElement::InlineMath(ref s) => write!(f, "${}$", s)?,
                TextElement::Bold(ref e) => write!(f, r"\textbf{{{}}}", e)?,
                TextElement::Italic(ref e) => {
                    write!(f, r"\textit{{{}}}", e)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use crate::text::*;
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn create_simple_text() {
        let should_be = "Hello World";
        let mut text = Text::new();
        text.push_text("Hello World");

        test_element(&[&text], should_be)
    }

    #[test]
    fn textgraph_with_bold_text() {
        let should_be = "Hello \\textbf{World}";
        let mut text = Text::new();
        text.push_text("Hello ");
        text.push(TextElement::Bold(Box::new(TextElement::Plain(
            "World".to_string(),
        ))));

        test_element(&[&text], should_be)
    }

    #[test]
    fn textgraph_with_italic_text() {
        let should_be = "Hello \\textit{World}";

        let mut text = Text::new();
        text.push_text("Hello ");
        text.push(TextElement::Italic(Box::new(TextElement::Plain(
            "World".to_string(),
        ))));

        test_element(&[&text], should_be)
    }

    #[test]
    fn inline_code() {
        let should_be = "Hello $\\lambda$ World!";

        let mut text = Text::new();
        text.push_text("Hello ")
            .push(TextElement::InlineMath(r"\lambda".to_string()))
            .push_text(" World!");

        test_element(&[&text], should_be)
    }
}
