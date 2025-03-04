use document::Element;
use paragraph::ParagraphElement;
use std::slice::Iter;

/// A trait to represent all types of Section
pub trait SectionElement {
    /// Get if element is numbered
    fn numbered(&self) -> bool;
    /// Iterate throw elements
    fn iter(&self) -> Iter<Element>;
    /// Check if empty
    fn is_empty(&self) -> bool;
    /// Get name of element
    fn get_name(&self) -> &ParagraphElement;
    /// Get name of section
    fn get_section_name(&self) -> &str;
}

macro_rules! create_section_type {
    ($section_name:ident, $section_tex:literal) => {
        /// A document SectionElement.
        ///
        /// Like the `Document` type, a `$section_tex` is more or less just a collection of
        /// `Element`s. When rendered it will start with `\$section_tex{Section Name}` and
        /// then each element will be rendered in turn.
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $section_name {
            /// The name of the section.
            pub name: ParagraphElement,
            /// Elements inside
            elements: Vec<Element>,
            /// Type of section
            sectioning_name: String,
            /// Numbered section
            pub numbered: bool,
        }

        impl $section_name {
            /// Create a new section with the specified name.
            pub fn new(name: &str) -> Self {
                Self {
                    name: ParagraphElement::from(name),
                    elements: Default::default(),
                    sectioning_name: $section_tex.to_owned(),
                    numbered: true,
                }
            }

            /// Create a new section with the specified and formatted name.
            pub fn new_formatted(name: ParagraphElement) -> Self {
                Self {
                    name: name,
                    elements: Default::default(),
                    sectioning_name: $section_tex.to_owned(),
                    numbered: true,
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
        }

        impl SectionElement for $section_name {
            fn numbered(&self) -> bool {
                self.numbered
            }

            fn get_name(&self) -> &ParagraphElement {
                &self.name
            }

            fn get_section_name(&self) -> &str {
                &self.sectioning_name
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
    };
}

create_section_type!(Part, "part");
create_section_type!(Chapter, "chapter");
create_section_type!(Section, "section");
create_section_type!(Subsection, "subsection");
create_section_type!(Subsubsection, "subsubsection");
// create_section_type!(Paragraph, "paragraph");
// create_section_type!(Subparagraph, "subparagraph");
