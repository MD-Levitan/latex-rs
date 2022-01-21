extern crate doc_comment;

use self::doc_comment::doc_comment;
use crate::{Text, Writable};

macro_rules! create_commands {
    (@format ($writter:expr, $self:expr) {$name:ident}) => {
        write!($writter, "{{{}}}", $self.$name)?;
        };

        (@format ($writter:expr, $self:expr) [$name:ident]) => {
            write!($writter, "[{}]", $self.$name)?;
        };

        (@format ($writter:expr, $self:expr) [$($name:ident),+]) => {
            write!($writter, "[")?;
            $(
                write!($writter, "{},", $self.$name)?;
            )+
            write!($writter, "]")?;
        };

        (@format ($writter:expr, $self:expr) {$($name:ident),+}) => {
            write!($writter, "{")?;
            $(
                write!($writter, "{},", $self.$name)?;
            )+
            write!($writter, "}")?;
        };


        (
            $( ($command_name:ident, $command_tex:literal, {$($param_name:ident:$param_type:ty),*}, $($formatter:tt)*) ),+;

            $( ($command_name_line:ident, $command_tex_line:literal) ),+
        ) =>
        {

            doc_comment! {

                concat!("`Command` - represent all one-line commands.\n\n",
                $(
                    "`Command::", stringify!($command_name_line), "`- command for `\\", $command_tex_line ,"` latex.\n\n",
                )+

                $(
                    "`Command::", stringify!($command_name), "`- command for `\\", $command_tex , stringify!($($formatter)*), "` latex.\n\n",
                )+
                        ),

                #[allow(missing_docs)]
                #[derive(Debug, Clone, PartialEq)]
                pub enum Command {
                    $(
                        $command_name($command_name),
                    )+

                    $(
                        $command_name_line,
                    )+

                }

            }

            #[allow(non_snake_case)]
            impl Writable for Command {
                fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
                    match &self
                    {
                        $( Command::$command_name(ref $command_name) => $command_name.write_to(writer), )+

                        $( Command::$command_name_line => write!(writer, "\\{}\n", $command_tex_line), )+
                    }
                }
            }

            $(

            doc_comment! {
                concat!("`Command::", stringify!($command_name), "`- command for `\\", $command_tex , stringify!($($formatter)*), "` latex.\n\n",),

                #[derive(Default, Debug, Clone, PartialEq)]
                pub struct $command_name
                {
                    $(
                        $param_name: $param_type,
                    )*
                }
            }


            impl $command_name
            {
                /// Create new command
                pub fn new(
                    $(
                        $param_name: $param_type,
                    )*
                ) -> Self
                {
                    Self{
                        $(
                            $param_name: $param_name,
                        )*
                    }
                }

            }

            impl Writable for $command_name {
                fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {

                write!(writer, "\\{}", $command_tex)?;
                $(
                    create_commands!(@format (writer, self) $formatter);
                )*

                write!(writer, "\n")

                }
            }


            )+
        };
}

create_commands!(
    (Label, "label", { label: String }, {label}),
    (Ref, "ref", { text: String }, {text}),
    (Framebox, "framebox", {text:Text, size:String, pos:String}, [size][pos]{text});
    (TableOfContents, "tableofcontents"),
    (TitlePage, "maketitle"),
    (ClearPage, "clearpage"),
    (BigSkip, "bigskip"),
    (NewPage, "newpage")
);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Latex, TextElement, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn render_simple_label() {
        let should_be = "\\label{some label}\n";
        let command: Command = Command::Label(Label::new("some label".into()));

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_simple_framebox() {
        let should_be = "\\framebox[1][2]{some framebox}\n";
        let command: Command = Command::Framebox(Framebox::new(
            "some framebox".into(),
            "1".to_owned(),
            "2".to_owned(),
        ));

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_framebox() {
        let should_be = "\\framebox[30pc][l]{\\textbf{some bold framebox}}\n";
        let mut text = Text::new();
        text.push(TextElement::bold("some bold framebox"));

        let command: Command =
            Command::Framebox(Framebox::new(text, "30pc".to_owned(), "l".to_owned()));

        test_element(&[&command], should_be)
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
