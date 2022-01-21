extern crate doc_comment;

use self::doc_comment::doc_comment;
use crate::Writable;

macro_rules! create_commands {
    ($(($command_name:ident, $command_tex:literal)), +) => {
        doc_comment! {

        concat!("`Command` - represent all one-line commands without params.\n\n",
        $(
    "`Command::", stringify!($command_name), "`- command for `\\", $command_tex ,"` latex.\n\n",
        )+
    ),

        #[allow(missing_docs)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum Command {
            $(
                $command_name,
            )+

        }

        impl Writable for Command {
            fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                match &self
                {
                    $( Command::$command_name => write!(writer, "\\{}\n", $command_tex), )+
                }
            }
        }
    }
    };
}

create_commands!(
    (TableOfContents, "tableofcontents"),
    (TitlePage, "maketitle"),
    (ClearPage, "clearpage"),
    (BigSkip, "bigskip"),
    (NewPage, "newpage")
);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn render_table_contents() {
        let should_be = "\\tableofcontents\n";
        let command: Command = Command::TableOfContents;

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_table_maketitle() {
        let should_be = "\\maketitle\n";
        let command: Command = Command::TitlePage;

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_table_clearpage() {
        let should_be = "\\clearpage\n";
        let command: Command = Command::ClearPage;

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_table_bigskip() {
        let should_be = "\\bigskip\n";
        let command: Command = Command::BigSkip;

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_table_newpage() {
        let should_be = "\\newpage\n";
        let command: Command = Command::NewPage;

        test_element(&[&command], should_be)
    }
}
