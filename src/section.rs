extern crate doc_comment;

use self::doc_comment::doc_comment;

use document::Element;
use std::{io::Error, io::Write, slice::Iter};
use text::Text;

use crate::Writable;

/// A trait to represent all types of Section
trait SectionElement {
    /// Get if element is numbered
    fn numbered(&self) -> bool;
    /// Iterate throw elements
    fn iter(&self) -> Iter<Element>;
    /// Check if empty
    fn is_empty(&self) -> bool;
    /// Get name of element
    fn get_name(&self) -> &Text;
    /// Get name of section
    fn get_section_name(&self) -> &str;
    /// Get toctitle of section
    fn get_toctitle(&self) -> &Option<Text>;
}

fn visit_sectioning_element<T: SectionElement, W: Write>(
    writer: &mut W,
    section: &T,
) -> Result<(), Error> {
    // Write declaration of element
    match section.numbered() {
        true => {
            write!(writer, r"\{}", section.get_section_name())?;
            if let Some(toctitle) = section.get_toctitle() {
                write!(writer, "[")?;
                toctitle.write_to(writer)?;
                write!(writer, "]{{")?;
            } else {
                write!(writer, "{{")?;
            }
        }

        false => {
            write!(writer, r"\{}*{{", section.get_section_name())?;
        }
    }

    section.get_name().write_to(writer)?;
    writeln!(writer, "}}")?;

    for element in section.iter() {
        element.write_to(writer)?;

        // LaTeX needs an empty line between paragraphs/elements otherwise
        // it'll automatically concatenate them together
        writeln!(writer)?;
    }

    Ok(())
}

macro_rules! create_section_type {
    ($section_name:ident, $section_tex:literal) => {
        doc_comment! {

        concat!(
            "`", stringify!($section_name), "` - a sectioning element that rendered as - `\\", $section_tex,
            "[toctitle]{text}` or `\\", $section_tex, "*{text}`.\n\n", "Like the `Document` type, it is more or less just a collection of `Element`s. \n"
        ),

        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $section_name {
            /// The name of the section.
            pub name: Text,
            /// Elements inside
            elements: Vec<Element>,
            /// Type of section
            sectioning_name: String,
            /// `Toctitle` - ontains entry for the table of contents if different from text.
            toctitle: Option<Text>,
            /// Numbered section
            pub numbered: bool,
        }

        impl $section_name {
            /// Create a new section with the specified name.
            pub fn new(name: &str) -> Self {
                Self {
                    name: Text::from(name),
                    elements: Default::default(),
                    sectioning_name: $section_tex.to_owned(),
                    toctitle: None,
                    numbered: true,
                }
            }

            /// Create a new section with the specified and formatted name.
            pub fn new_formatted(name: Text) -> Self {
                Self {
                    name: name,
                    elements: Default::default(),
                    sectioning_name: $section_tex.to_owned(),
                    toctitle: None,
                    numbered: true,
                }
            }

            /// Set `toctitle` that contains entry for the table of contents
            pub fn set_toctitle<T: Into<Text>>(&mut self, titile: T) -> &mut Self
            {
                self.toctitle = Some(titile.into());
                self
            }

            /// Add an element to the Section.
            pub fn push<I>(&mut self, element: I) -> &mut Self
            where
                I: Into<Element>,
            {
                self.elements.push(element.into());
                self
            }
        }

        impl SectionElement for $section_name {
            fn numbered(&self) -> bool {
                self.numbered
            }

            fn get_name(&self) -> &Text {
                &self.name
            }

            fn get_section_name(&self) -> &str {
                &self.sectioning_name
            }

            fn get_toctitle(&self) -> &Option<Text>
            {
                &self.toctitle
            }

            /// Iterate over the elements in this list.
            fn iter(&self) -> Iter<Element> {
                self.elements.iter()
            }

            /// Is this section empty?
            fn is_empty(&self) -> bool {
                self.elements.is_empty()
            }
        }

        impl Writable for $section_name {
            fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                visit_sectioning_element(writer, self)
            }
        }
    }
    };
}

create_section_type!(Part, "part");
create_section_type!(Chapter, "chapter");
create_section_type!(Section, "section");
create_section_type!(Subsection, "subsection");
create_section_type!(Subsubsection, "subsubsection");
create_section_type!(Paragraph, "paragraph");
create_section_type!(Subparagraph, "subparagraph");

/// A document Container.
/// `Contaier` is using only for storing elements withput formatting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    /// Elements inside
    elements: Vec<Element>,
}

impl Container {
    /// Create a new section with the specified name.
    pub fn new() -> Self {
        Self {
            elements: Default::default(),
        }
    }

    /// Add an element to the Section.
    pub fn push<I>(&mut self, element: I) -> &mut Self
    where
        I: Into<Element>,
    {
        self.elements.push(element.into());
        self
    }

    /// Iterate over the elements in this list.
    pub fn iter(&self) -> Iter<Element> {
        self.elements.iter()
    }

    /// Is this section empty?
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Writable for Container {
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        for element in self.iter() {
            element.write_to(writer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{section::*, TextElement};
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn render_blank_section() {
        let should_be = "\\section{First Section}\n";
        let section = Section::new("First Section");

        test_element(&[&section], should_be)
    }

    #[test]
    fn render_blank_subsection() {
        let should_be = "\\subsection{First Section}\n";

        let section = Subsection::new("First Section");
        test_element(&[&section], should_be)
    }

    #[test]
    fn render_blank_part() {
        let should_be = "\\part{First Section}\n";

        let section = Part::new("First Section");
        test_element(&[&section], should_be)
    }

    #[test]
    fn render_blank_chapter() {
        let should_be = "\\chapter{First Section}\n";

        let section = Chapter::new("First Section");

        test_element(&[&section], should_be)
    }

    #[test]
    fn section_with_paragraphs() {
        let should_be = r#"\section{First Section}
Lorem Ipsum...
Hello World!
"#;

        let mut section = Section::new("First Section");
        section.push("Lorem Ipsum...").push("Hello World!");

        test_element(&[&section], should_be)
    }

    #[test]
    fn section_non_numbered() {
        let should_be = r#"\section*{First Section}
Lorem Ipsum...
Hello World!
"#;

        let mut section = Section::new("First Section");
        section.numbered = false;
        section.push("Lorem Ipsum...").push("Hello World!");

        test_element(&[&section], should_be)
    }

    #[test]
    fn section_with_toctitle() {
        let should_be = r#"\section[Not First Section]{First Section}
Lorem Ipsum...
Hello World!
"#;

        let mut section = Section::new("First Section");
        section
            .set_toctitle("Not First Section")
            .push("Lorem Ipsum...")
            .push("Hello World!");

        test_element(&[&section], should_be)
    }

    #[test]
    fn section_with_link() {
        let should_be = r#"\section{\href{some_link}{https:\\example.com}}
Lorem Ipsum...
Hello World!
"#;

        let mut header = Text::new();

        header.push(TextElement::Link((
            "some_link".to_owned(),
            "https:\\\\example.com".to_owned(),
        )));

        let mut section = Section::new_formatted(header);
        section.push("Lorem Ipsum...").push("Hello World!");

        test_element(&[&section], should_be)
    }
}
