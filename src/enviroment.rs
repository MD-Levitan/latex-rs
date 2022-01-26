use crate::Writable;

use document::Element;
use text::Text;

// macro_rules! create_enviroment_type {
//     ($env_name:ident, $env_tex:literal) => {
//         /// Add Envoriment.
//         ///
//         /// # Note
//         ///
//         /// It's contains generic params, and not for
//         /// all enviroments they are working
//         #[derive(Clone, Debug, Default, PartialEq)]
//         pub struct $env_name {
//             /// List of params in `{}` separate by `,`
//             ///
//             /// # Example
//             ///
//             /// vec!["param1", "param2"] -> {param1, param2}
//             params: Vec<String>,

//             /// List of params in `[]` separated by `,`
//             ///
//             /// # Example
//             ///
//             /// vec!["param1", "param2"] -> [param1, param2]
//             optional_params: Vec<String>,

//             /// Elements inside
//             elements: Vec<Element>,

//             /// Type of env
//             enving_name: String,

//             /// Numbered env
//             pub numbered: bool,
//         }

//         impl $env_name {
//             /// Create a new env with the specified name.
//             pub fn new(name: &str) -> Self {
//                 Self {
//                     name: Text::from(name),
//                     elements: Default::default(),
//                     enving_name: $env_tex.to_owned(),
//                     numbered: true,
//                 }
//             }

//             /// Create a new env with the specified and formatted name.
//             pub fn new_formatted(name: Text) -> Self {
//                 Self {
//                     name: name,
//                     elements: Default::default(),
//                     enving_name: $env_tex.to_owned(),
//                     numbered: true,
//                 }
//             }

//             /// Add an element to the Section.
//             pub fn push<I>(&mut self, element: I) -> &mut Self
//             where
//                 I: Into<Element>,
//             {
//                 self.elements.push(element.into());
//                 self
//             }
//         }

//         impl Writable for $env_name {
//             fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
//                 visit_enving_element(writer, self)
//             }
//         }
//     };
// }

/// Add Envoriment.
///
/// # Note
///
/// It's contains generic params, and not for
/// all enviroments they are working
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Environment {
    /// List of params. Each param in own `{}`
    ///
    /// # Example
    ///
    /// vec!["param1", "param2"] -> {enving_name}{param1}{param2}
    params: Vec<String>,

    /// List of params in `[]` separated by `,`
    ///
    /// # Example
    ///
    /// vec!["param1", "param2"] -> [param1, param2]
    optional_params: Vec<String>,

    /// Elements inside
    elements: Vec<Element>,

    /// Type of env
    enving_name: String,

    /// Numbered env
    pub numbered: bool,
}

impl Environment {
    /// Create a new enviroment with the specified name.
    pub fn new_empty(name: &str) -> Self {
        Self {
            elements: Default::default(),
            optional_params: Default::default(),
            params: Default::default(),
            enving_name: name.to_owned(),
            numbered: true,
        }
    }

    /// Create a new enviroment with params and content.
    pub fn new(
        name: &str,
        text: Vec<String>,
        params: Option<Vec<String>>,
        optional_params: Option<Vec<String>>,
    ) -> Self {
        Self {
            elements: text
                .iter()
                .map(|x| Element::Text(x.as_str().into()))
                .collect(),
            optional_params: optional_params.unwrap_or_default(),
            params: params.unwrap_or_default(),
            enving_name: name.to_owned(),
            numbered: true,
        }
    }

    /// Create a new enviroment with the params
    pub fn with_params(
        name: &str,
        params: Option<Vec<String>>,
        optional_params: Option<Vec<String>>,
    ) -> Self {
        Self {
            elements: Default::default(),
            optional_params: optional_params.unwrap_or_default(),
            params: params.unwrap_or_default(),
            enving_name: name.to_owned(),
            numbered: true,
        }
    }

    /// Add an element to the Enviroment.
    pub fn push<I>(&mut self, element: I) -> &mut Self
    where
        I: Into<Element>,
    {
        self.elements.push(element.into());
        self
    }

    /// Add an text as `Element::Text` to the list.
    pub fn push_text<I>(&mut self, element: I) -> &mut Self
    where
        I: Into<Text>,
    {
        self.elements.push(Element::Text(element.into()));
        self
    }
}

impl Writable for Environment {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        write!(writer, r"\begin{{{}}}", self.enving_name)?;
        if !self.params.is_empty() {
            self.params
                .iter()
                .try_for_each(|item| write!(writer, "{{{}}}", item))?;
        }

        if !self.optional_params.is_empty() {
            write!(writer, "[{}]", self.optional_params.join(","))?;
        }
        writeln!(writer)?;

        for elem in &self.elements {
            elem.write_to(writer)?;
            writeln!(writer)?;
        }
        writeln!(writer, r"\end{{{}}}", self.enving_name)
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;
    use crate::{Latex, Writable};

    fn test_element<W: Writable>(elements: &[&W], real: &str) {
        let mut generator = Latex::new(Vec::new());
        elements.iter().for_each(|&e| generator.write(e).unwrap());
        assert_eq!(String::from_utf8(generator.into_inner()).unwrap(), real);
    }

    #[test]
    fn render_empty_env() {
        let should_be = "\\begin{test}\n\\end{test}\n";
        let env = Environment::new_empty("test");

        test_element(&[&env], should_be)
    }

    #[test]
    fn render_with_optional_params() {
        let should_be =
            "\\begin{lstlisting}[language=Python]\nimport requests\n\\end{lstlisting}\n";
        let env = Environment::new(
            "lstlisting",
            vec!["import requests".to_owned()],
            None,
            Some(vec!["language=Python".to_owned()]),
        );

        test_element(&[&env], should_be)
    }

    #[test]
    fn render_with_params() {
        let should_be = "\\begin{tabular}{|p{2cm}|p{2cm}|p{8.5cm}|}\nsome\n\\end{tabular}\n";
        let env = Environment::new(
            "tabular",
            vec!["some".to_owned()],
            Some(vec!["|p{2cm}|p{2cm}|p{8.5cm}|".to_owned()]),
            None,
        );

        test_element(&[&env], should_be)
    }
}
