//! A crate for generating LaTeX documents programatically.
//!
//! The main purpose of this library is to make the job of programatically
//! generating LaTeX reports and documents (which will probably then be
//! compiled to PDF) as easy as possible.
//!
//! This library tries to use Rust's powerful type system to give your document
//! additional semantic meaning and compile-time typesafety. For example,
//! [`Align`] *could* easily be implemented with `Element::Environment`, where
//! each equation is written in as-is and appended to the list of lines.
//! However by pulling it into its own type you gain the ability to do
//! equation-specific manipulations and have nice abstractions like an
//! `Equation`'s [`label()`] method.
//!
//!
//! # Examples
//!
//! ## Creating A Document
//!
//! Here's how to create a reasonably complex document containing a title page,
//! a table of contents, some equations, and two sections.
//!
//! ```rust
//! use latex::{DocumentClass, Element, Document, Section, Align, Command};
//!
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let mut doc = Document::new(DocumentClass::Article);
//!
//! // Set some metadata for the document
//! doc.preamble.title("My Fancy Document");
//! doc.preamble.author("Michael-F-Bryan");
//!
//! doc.push(Command::TitlePage)
//!     .push(Command::ClearPage)
//!     .push(Command::TableOfContents)
//!     .push(Command::ClearPage);
//!
//! let mut section_1 = Section::new("Section 1");
//! section_1.push("Here is some text which will be put in paragraph 1.")
//!          .push("And here is some more text for paragraph 2.");
//! doc.push(section_1);
//!
//! let mut section_2 = Section::new("Section 2");
//!
//! section_2.push("More text...")
//!          .push(Align::from("y &= mx + c"));
//!
//! doc.push(section_2);
//!
//! let rendered = latex::print(&doc)?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! This will generate the LaTeX source for you, so all you need to do now is
//! write it to a file and then run your favourite tex build tool on it (I
//! personally use [latexmk]).
//!
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::Write;
//! use std::process::Command;
//!
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! # let rendered = String::new();
//! // Write our rendered text to a file
//! let mut f = File::open("report.tex")?;
//! write!(f, "{}", rendered)?;
//!
//! // Then call latexmk on it
//! let exit_status = Command::new("latexmk").arg("report.tex").status()?;
//!
//! assert!(exit_status.success());
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! ## Traversing A Document
//!
//! Once you have created a document, you have the ability to walk it and do
//! any transformation you want using the [`Visitor`] trait. All methods on the
//! trait come with default `impls` which will recursively visit the various
//! nodes in your `Document`. This means if you only care about the `Paragraph`
//! nodes you can implement just the [`visit_paragraph()`] method and then
//! inspect all `Paragraph` nodes in the document. Everything else should *Just
//! Work*.
//!
//! If you want to see how you can write your own `Visitor`, check out the
//! source code for the [`Printer`] struct.
//!
//!
//! [latexmk]: http://mg.readthedocs.io/latexmk.html
//! [`Align`]: struct.Align.html
//! [`label()`]: struct.Equation.html#method.label
//! [`Visitor`]: visitor/trait.Visitor.html
//! [`visit_paragraph()`]: visitor/trait.Visitor.html#method.visit_paragraph
//! [`Printer`]: visitor/struct.Printer.html

#![deny(missing_docs)]

mod commands;
mod document;
mod enviroment;
mod equations;
mod lists;
mod section;
mod text;

pub use commands::*;
pub use document::{Document, DocumentClass, Element, Preamble, PreambleElement};
pub use enviroment::Environment;
pub use equations::{Align, AlignEquation, Equation};
pub use lists::{Item, List, ListKind};
pub use section::{Chapter, Container, Part, Section, Subsection, Subsubsection};
pub use text::{Text, TextElement};

use std::io::{Error, Write};

/// Trait for Latex objections to print/write document
/// To replace `Visitor` conception.
pub trait Writable {
    /// Writes `self` as latex to `writer`
    ///
    /// # Arguments
    /// * `writer` - destination writer
    ///
    /// # Returns
    /// `()` or `std::io::Error` if an error occurred during writing
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

/// Struct for creating Latex
pub struct Latex<W: Write> {
    writer: W,
}

impl<W: Write> Latex<W> {
    /// Creates a new `Latex` struct
    ///
    /// # Arguments
    ///
    /// * `writer` - Destination for Latex data
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Returns the underlying `writer` and consumes the object
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Writes a `Writable` elemnt to the document
    ///
    /// # Returns
    /// `()` or `std::io::Error` if an error occurred during writing to the underlying writer
    pub fn write<T: Writable>(&mut self, element: &T) -> Result<(), Error> {
        element.write_to(&mut self.writer)?;
        Ok(())
    }
}

/// Print a document to a string.
pub fn print(doc: &Document) -> Result<String, anyhow::Error> {
    let mut buffer = Vec::new();
    let mut latex = Latex::new(&mut buffer);
    latex.write(doc)?;
    let rendered = String::from_utf8(buffer)?;
    Ok(rendered)
}
