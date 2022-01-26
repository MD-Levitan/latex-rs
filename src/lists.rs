use std::ops::Deref;
use std::slice::Iter;

use section::Container;
use text::Text;

use crate::document::Element;
use crate::Writable;

/// Wrapper around a single list item.
///
/// An item will usually be rendered with `\item` followed by the item's text.
#[derive(Clone, Debug, PartialEq)]
pub struct Item(pub Container);

impl Deref for Item {
    type Target = Container;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Which kind of list should be used?
#[derive(Clone, Debug, PartialEq)]
pub enum ListKind {
    /// A numbered list.
    Enumerate,
    /// An un-numbered list.
    Itemize,
}

impl ListKind {
    /// Get the `ListKind`'s environment name.
    pub fn environment_name(&self) -> &str {
        match *self {
            ListKind::Enumerate => "enumerate",
            ListKind::Itemize => "itemize",
        }
    }
}

/// A list (either dot points or numbered).
///
/// # Examples
///
/// A list can be used like so:
///
/// ```rust
/// use latex::{List, ListKind};
///
/// let mut list = List::new(ListKind::Itemize);
/// list.push_text("Hello").push_text("From").push_text("Some").push_text("Dot-points");
/// ```
///
/// Calling the `render()` method on the list will then give something like
/// this:
///
/// ```tex
/// \begin{itemize}
/// \item Hello
/// \item From
/// \item Some
/// \item Dot-points
/// \end{itemize}
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct List {
    /// The kind of list this is.
    pub kind: ListKind,
    items: Vec<Item>,
}

impl List {
    /// Create an empty list of the specified type.
    pub fn new(kind: ListKind) -> List {
        List {
            kind,
            items: Vec::new(),
        }
    }

    /// Add an element to the list.
    pub fn push<P>(&mut self, item: P) -> &mut Self
    where
        P: Into<Container>,
    {
        self.items.push(Item(item.into()));
        self
    }

    /// Add an element to the list.
    pub fn push_element<E>(&mut self, item: E) -> &mut Self
    where
        E: Into<Element>,
    {
        let mut container = Container::new();
        container.push(item);

        self.items.push(Item(container));
        self
    }

    /// Add a text as `Element::Text` to the list.
    pub fn push_text<P>(&mut self, item: P) -> &mut Self
    where
        P: Into<Text>,
    {
        let mut container = Container::new();
        container.push(Element::Text(item.into()));

        self.items.push(Item(container));
        self
    }

    /// Iterate over the items in the list.
    pub fn iter(&self) -> Iter<Item> {
        self.items.iter()
    }
}

impl Writable for List {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        let env = self.kind.environment_name();

        writeln!(writer, r"\begin{{{}}}", env)?;

        for item in self.iter() {
            write!(writer, "\\item ")?;
            (**item).write_to(writer)?;
            writeln!(writer)?;
        }

        writeln!(writer, r"\end{{{}}}", env)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Container, Element, List, ListKind, Text};
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn push_item_to_list() {
        let mut list = List::new(ListKind::Itemize);

        let mut item = Container::new();
        item.push(Element::Text(Text::from("Hello, world!")));
        assert_eq!(list.items.len(), 0);
        list.push(item);
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn render_empty_itemize_list() {
        let should_be = "\\begin{itemize}\n\\end{itemize}\n";
        let list = List::new(ListKind::Itemize);
        test_element(&[&list], should_be)
    }

    #[test]
    fn render_enumerated_list() {
        let should_be = "\\begin{enumerate}\n\\end{enumerate}\n";
        let list = List::new(ListKind::Enumerate);
        test_element(&[&list], should_be)
    }

    #[test]
    fn render_itemize_list_simple() {
        let should_be =
            "\\begin{itemize}\n\\item Apple\n\\item Orange\n\\item Cherry\n\\end{itemize}\n";
        let mut list = List::new(ListKind::Itemize);
        list.push_text("Apple");
        list.push_text("Orange");
        list.push_text("Cherry");

        test_element(&[&list], should_be)
    }

    #[test]
    fn render_enumerated_list_simple() {
        let should_be =
            "\\begin{enumerate}\n\\item Apple\n\\item Orange\n\\item Cherry\n\\end{enumerate}\n";
        let mut list = List::new(ListKind::Enumerate);
        list.push_text("Apple");
        list.push_text("Orange");
        list.push_text("Cherry");

        test_element(&[&list], should_be)
    }
}
