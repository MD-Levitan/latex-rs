extern crate doc_comment;

use self::doc_comment::doc_comment;
use crate::{Text, Writable};

macro_rules! create_commands {
        // format params: param
        (@format ($writter:expr, $self:expr) $name:ident) => {
            write!($writter, "{}", $self.$name)?;
        };
        // format params: {param}
        (@format ($writter:expr, $self:expr) {$name:ident}) => {
            write!($writter, "{{{}}}", $self.$name)?;
        };

        // format params: [param]
        (@format ($writter:expr, $self:expr) [$name:ident]) => {
            if let Some(value) = &$self.$name
            {
                write!($writter, "[{}]", value)?;
            }
        };

        // format params: [param1, param2]
        (@format ($writter:expr, $self:expr) [$($name:ident),+]) => {
            write!($writter, "[")?;
            $(
                if let Some(value) = &$self.$name
                {
                    write!($writter, "{},", value)?;
                }
            )+
            write!($writter, "]")?;
        };

        // format params: {param1, param2}
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

            impl From<$command_name> for Command {
                fn from(other: $command_name) -> Self {
                    Command::$command_name(other)
                }
            }


            )+
        };
}

create_commands!(
    (Label, "label", { label: String }, {label}),
    (Ref, "ref", { text: String }, {text}),
    (Bibitem, "bibitem", {text: Text, cite: String}, {cite}text),
    (Bibliographystyle, "bibliographystyle", {style: String}, {style}),
    (Bibliography, "bibliography", {file: String}, {file}),
    (Cite, "cite", { reference: String, subcit: Option<Text> }, [subcit]{reference}),
    (Framebox, "framebox", {text:Text, size:Option<String>, pos:Option<String>}, [size][pos]{text});
    (TableOfContents, "tableofcontents"),
    (TitlePage, "maketitle"),
    (ClearPage, "clearpage"),
    (BigSkip, "bigskip"),
    (NewPage, "newpage"),
    (Appendix, "appendix")
);

impl Label {
    /// Generate `Label` from `object_type` and `object_name`.
    ///
    /// # Example
    ///
    /// ```
    /// use latex::Label;
    /// let label = Label::generate_default_label("subsection", "Introduction");
    ///
    /// ```
    ///
    /// Result in latex:
    ///
    /// ```tex
    /// \label{subsection:introduction}
    /// ```
    pub fn generate_default_label(object_type: &str, object_name: &str) -> Label {
        Label::new(format!("{}:{}", object_type, object_name.to_lowercase()))
    }

    /// Get reference(`Ref`) from `Label`
    pub fn get_ref(&self) -> Ref {
        Ref::new(self.label.clone())
    }
}

impl Bibitem {
    /// Get reference(`Cite`) from `Bibitem`
    pub fn get_ref(&self) -> Cite {
        Cite::new(self.cite.clone(), None)
    }
}

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
    fn render_bibliography() {
        let should_be = "\\bibliography{some_file.txt}\n";

        let command = Bibliography::new("some_file.txt".to_owned());

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_bibliographystyle() {
        let should_be = "\\bibliographystyle{plain}\n";

        let command = Bibliographystyle::new("plain".to_owned());

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_bibitem() {
        let should_be = "\\bibitem{key1}\\textbf{Some Book}\n";

        let command = Bibitem::new(TextElement::bold("Some Book").into(), "key1".to_string());

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_get_cite() {
        let should_be = "\\cite{key1}\n";
        let command: Command =
            Bibitem::new(TextElement::bold("Some Book").into(), "key1".to_string())
                .get_ref()
                .into();

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_simple_label() {
        let should_be = "\\label{some label}\n";
        let command: Command = Command::Label(Label::new("some label".into()));

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_gen_label() {
        let should_be = "\\label{subsection:introduction}\n";
        let command: Command =
            Command::Label(Label::generate_default_label("subsection", "Introduction"));

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_get_refer() {
        let should_be = "\\ref{subsection:introduction}\n";
        let command: Command = Label::generate_default_label("subsection", "Introduction")
            .get_ref()
            .into();

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_simple_framebox() {
        let should_be = "\\framebox[30pc][l]{some framebox}\n";
        let command: Command = Command::Framebox(Framebox::new(
            "some framebox".into(),
            Some("30pc".to_owned()),
            Some("l".to_owned()),
        ));

        test_element(&[&command], should_be)
    }

    #[test]
    fn render_framebox() {
        let should_be = "\\framebox[30pc][l]{\\textbf{some bold framebox}}\n";
        let mut text = Text::new();
        text.push(TextElement::bold("some bold framebox"));

        let command: Command = Command::Framebox(Framebox::new(
            text,
            Some("30pc".to_owned()),
            Some("l".to_owned()),
        ));

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

    #[test]
    fn render_table_appendix() {
        let should_be = "\\appendix\n";
        let command: Command = Command::Appendix;

        test_element(&[&command], should_be)
    }
}
