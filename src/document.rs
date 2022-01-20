use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::slice::Iter;

use commands::Command;
use enviroment::Environment;
use equations::Align;
use lists::List;
use section::{
    Chapter, Container, Paragraph, Part, Section, Subparagraph, Subsection, Subsubsection,
};
use text::Text;

use crate::Writable;

/// The root Document node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    /// The document class.
    pub class: DocumentClass,
    /// The `Document`'s preamble.
    pub preamble: Preamble,
    /// Document arguments, like `[12pt,oneside,a4paper]`
    pub arguments: Vec<String>,
    /// The various elements inside this `Document`.
    elements: Vec<Element>,
}

impl Document {
    /// Create a new `Document` with the specified `DocumentClass`.
    pub fn new(document_class: DocumentClass) -> Self {
        Document {
            class: document_class,
            ..Default::default()
        }
    }

    /// Add an element to the `Document`.
    ///
    /// To make this work as seamlessly as possible, it will accept anything
    /// which can be converted into an `Element` using `into()` and supports
    /// the builder pattern with method chaining.
    pub fn push<E>(&mut self, element: E) -> &mut Self
    where
        E: Into<Element>,
    {
        self.elements.push(element.into());
        self
    }

    /// Iterate over the Elements in this document.
    pub fn iter(&self) -> Iter<Element> {
        self.elements.iter()
    }

    /// A convience method to include one document into
    /// another by cloning the individual nodes.
    pub fn push_doc(&mut self, doc: &Document) -> &mut Self {
        for element in doc.iter() {
            self.push(element.clone());
        }
        self
    }
}

impl Deref for Document {
    type Target = Vec<Element>;

    /// A shortcut to let you iterate over the elements in the `Document`.
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

/// The kind of Document being generated.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum DocumentClass {
    Article,
    Book,
    Report,
    /// A partial document comes without header and footer.
    /// It is intended to be included (`include{}`) in some other tex file.
    Part,
    Other(String),
}

impl Default for DocumentClass {
    fn default() -> Self {
        DocumentClass::Article
    }
}

impl Display for DocumentClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DocumentClass::Article => write!(f, "article"),
            DocumentClass::Book => write!(f, "book"),
            DocumentClass::Report => write!(f, "report"),
            DocumentClass::Part => write!(f, ""),
            DocumentClass::Other(ref s) => write!(f, "{}", *s),
        }
    }
}

impl Extend<Element> for Document {
    fn extend<T: IntoIterator<Item = Element>>(&mut self, iter: T) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl Writable for Document {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        match self.class {
            // only go through childs if we have a partial document
            DocumentClass::Part => {
                for element in self.iter() {
                    element.write_to(writer)?;
                }
            }
            // write a full document
            _ => {
                writeln!(
                    writer,
                    r"\documentclass[{}]{{{}}}",
                    self.arguments.join(","),
                    self.class
                )?;

                self.preamble.write_to(writer)?;

                writeln!(writer, r"\begin{{document}}")?;

                for element in self.iter() {
                    element.write_to(writer)?;
                }

                writeln!(writer, r"\end{{document}}")?;
            }
        }
        Ok(())
    }
}

/// An element of the document's preamble.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum PreambleElement {
    /// Use a package with an optional argument.  
    UsePackage {
        package: String,
        argument: Option<String>,
    },
    /// Create a `/newcommand` line in latex
    NewCommand {
        name: String,
        args_num: Option<usize>,
        default_arg: Option<String>,
        definition: String,
    },
    /// An escape hatch for including an arbitrary bit of TeX in a preamble.
    UserDefined(String),
}

/// A node representing the document's preamble.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preamble {
    /// The document's author.
    pub author: Option<String>,
    /// An optional title for the document.
    pub title: Option<String>,
    contents: Vec<PreambleElement>,
}

impl Preamble {
    /// Set the document's author.
    pub fn author(&mut self, name: &str) -> &mut Self {
        self.author = Some(name.to_string());
        self
    }

    /// Set the document title.
    pub fn title(&mut self, name: &str) -> &mut Self {
        self.title = Some(name.to_string());
        self
    }

    /// Add a package import to the preamble.
    pub fn use_package(&mut self, name: &str) -> &mut Self {
        self.contents.push(PreambleElement::UsePackage {
            package: name.to_string(),
            argument: None,
        });
        self
    }

    /// Interface of most commonly used way to write a `/newcommand` line in latex.  
    /// If you want to create `/newcommand` in
    /// other ways(like add default argument or do not assign the num of arguments),
    /// please use `push` method in `Preamble` struct.
    pub fn new_command(&mut self, name: &str, args_num: usize, definition: &str) -> &mut Self {
        self.contents.push(PreambleElement::NewCommand {
            name: String::from(name),
            args_num: Some(args_num),
            default_arg: None,
            definition: String::from(definition),
        });
        self
    }

    /// Iterate over each package used in the Preamble.
    pub fn iter(&self) -> Iter<PreambleElement> {
        self.contents.iter()
    }

    /// Is the preamble empty?
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Add a PreambleElement to the `Preamble`.
    ///
    /// To make this work as seamlessly as possible, it will accept anything
    /// which can be converted into an `PreambleElement` using `into()` and supports
    /// the builder pattern with method chaining.
    pub fn push<E>(&mut self, element: E) -> &mut Self
    where
        E: Into<PreambleElement>,
    {
        self.contents.push(element.into());
        self
    }
}

impl Extend<PreambleElement> for Preamble {
    fn extend<T: IntoIterator<Item = PreambleElement>>(&mut self, iter: T) {
        for elem in iter {
            self.push(elem);
        }
    }
}

impl Writable for Preamble {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        for item in self.iter() {
            match item {
                PreambleElement::UsePackage {
                    package: pkg,
                    argument: None,
                } => writeln!(writer, r"\usepackage{{{}}}", pkg)?,
                PreambleElement::UsePackage {
                    package: pkg,
                    argument: Some(arg),
                } => writeln!(writer, r"\usepackage[{}]{{{}}}", arg, pkg)?,
                PreambleElement::NewCommand {
                    name,
                    args_num,
                    default_arg,
                    definition,
                } => {
                    write!(writer, r"\newcommand{{\{}}}", name)?;
                    if let Some(num) = args_num {
                        write!(writer, r"[{}]", num)?;
                    }
                    if let Some(arg) = default_arg {
                        write!(writer, r"[{}]", arg)?;
                    }
                    writeln!(writer, r"{{")?;
                    writeln!(writer, "{}", definition)?;
                    writeln!(writer, r"}}")?;
                }
                PreambleElement::UserDefined(s) => writeln!(writer, r"{}", s)?,
            }
        }

        if !self.is_empty() && (self.title.is_some() || self.author.is_some()) {
            writeln!(writer)?;
        }

        if let Some(ref title) = self.title {
            writeln!(writer, r"\title{{{}}}", title)?;
        }
        if let Some(ref author) = self.author {
            writeln!(writer, r"\author{{{}}}", author)?;
        }

        Ok(())
    }
}

pub trait Test: std::fmt::Debug + PartialEq {}

/// The major elements in a `Document`, representing each type of possible
/// node.
///
/// For convenience, any variant which wraps a struct will implement `From` for
/// that struct. Meaning you can create an `Element::Para` node just by using
/// `some_paragraph.into()`.
#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    /// A Part - one of sectioning elements, has -1 level.
    ///
    /// # Note
    ///
    /// Part is available only in `report` and `book` documents.
    Part(Part),
    /// A Chapter - one of sectioning elements, has 0 level.
    ///
    /// # Note
    ///
    /// Chapter is available only in `report` and `book` documents.
    Chapter(Chapter),
    /// A section - one of sectioning elements, has 1 level.
    Section(Section),
    /// A subsection - one of sectioning elements, has 2 level.
    Subsection(Subsection),
    /// A subsubsection - one of sectioning elements, has 3 level.
    Subsubsection(Subsubsection),
    /// A paragraph - one of sectioning elements, has 4 level.
    Paragraph(Paragraph),
    /// A subparagraph - one of sectioning elements, has 5 level.
    Subparagraph(Subparagraph),
    /// A bare text.
    ///
    /// # Note
    ///
    /// You probably don't want to add a text directly to your document,
    /// instead add it to a `Section` so that if you are walking the AST later
    /// on things make sense.
    Text(Text),
    /// Container for `Elements`. Container doesn't provide any formatting,
    /// it can be used just for storing Elements
    Container(Container),

    /// One-line command
    Command(Command),

    /// An `align` environment for containing a bunch of equations.
    Align(Align),

    /// A generic environment and its lines.
    Environment(Environment),

    /// Any other element.
    ///
    /// This can be used as an escape hatch if the particular element you want
    /// isn't directly supported or if you need to do something which isn't
    /// easily expressed any other way. You simply provide the raw string you
    /// want and it will be rendered unchanged in the final document.
    UserDefined(String),
    /// A list.
    List(List),
    /// A generic include statement
    Input(String),

    // Add a dummy element so we can expand later on without breaking stuff
    #[doc(hidden)]
    _Other,
}

impl From<Text> for Element {
    fn from(other: Text) -> Self {
        Element::Text(other)
    }
}

impl<'a> From<&'a str> for Element {
    /// Create an arbitrary unescaped element from a string.
    fn from(other: &'a str) -> Self {
        Element::Text(Text::from(other))
    }
}

impl From<List> for Element {
    fn from(other: List) -> Self {
        Element::List(other)
    }
}

impl From<Align> for Element {
    fn from(other: Align) -> Self {
        Element::Align(other)
    }
}

impl From<Section> for Element {
    fn from(other: Section) -> Self {
        Element::Section(other)
    }
}

impl From<Command> for Element {
    fn from(other: Command) -> Self {
        Element::Command(other)
    }
}

impl From<Environment> for Element {
    fn from(other: Environment) -> Self {
        Element::Environment(other)
    }
}

impl<S, I> From<(S, I)> for Element
where
    S: AsRef<str>,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    /// Converts a tuple of name and a list of lines into an
    /// `Element::Environment`.
    fn from(other: (S, I)) -> Self {
        let (name, lines) = other;
        Element::Environment(Environment::new(
            name.as_ref(),
            lines.into_iter().map(|s| s.as_ref().to_string()).collect(),
            None,
            None,
        ))
    }
}

impl Writable for Element {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        match *self {
            Element::Text(ref p) => p.write_to(writer)?,
            Element::Part(ref p) => p.write_to(writer)?,
            Element::Chapter(ref p) => p.write_to(writer)?,
            Element::Section(ref p) => p.write_to(writer)?,
            Element::Subsection(ref p) => p.write_to(writer)?,
            Element::Subsubsection(ref p) => p.write_to(writer)?,
            Element::Paragraph(ref p) => p.write_to(writer)?,
            Element::Subparagraph(ref p) => p.write_to(writer)?,
            Element::Container(ref p) => p.write_to(writer)?,

            Element::Command(ref command) => command.write_to(writer)?,
            Element::UserDefined(ref s) => writeln!(writer, "{}", s)?,
            Element::Align(ref p) => p.write_to(writer)?,

            Element::Environment(ref env) => env.write_to(writer)?,
            Element::List(ref list) => list.write_to(writer)?,
            Element::Input(ref s) => writeln!(writer, "\\input{{{}}}", s)?,

            Element::_Other => unreachable!(),
        }

        Ok(())
    }
}
