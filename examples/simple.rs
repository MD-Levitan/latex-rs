extern crate latex;

use latex::{print, Command, Document, DocumentClass, Section};

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Fancy Document");
    doc.preamble.author("Michael-F-Bryan");

    doc.push(Command::TitlePage)
        .push(Command::ClearPage)
        .push(Command::TableOfContents)
        .push(Command::ClearPage);

    let mut section_1 = Section::new("Section 1");
    section_1
        .push("Here is some text which will be put in paragraph 1.")
        .push("And here is some more text for paragraph 2.");
    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    section_2.push("More text...");
    doc.push(section_2);

    doc
}

fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    println!("{}", rendered);
}
