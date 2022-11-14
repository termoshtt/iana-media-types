use anyhow::Result;
use inflector::Inflector;
use std::{fmt, fs, path::PathBuf, process::Command};

#[derive(Debug, Clone)]
struct MediaType {
    name: syn::Ident,
    member_idents: Vec<syn::Ident>,
    member_templates: Vec<String>,
}

impl MediaType {
    fn new(name: syn::Ident, url: &str) -> Result<Self> {
        let csv = ureq::get(url).call()?.into_string()?;
        let mut member_idents = Vec::new();
        let mut member_templates = Vec::new();
        for line in csv
            .lines()
            .skip(1 /* CSV header */)
            .filter(|line| !line.contains("OBSOLETE"))
        {
            let mut iter = line.split(',');
            match (iter.next(), iter.next()) {
                (Some(name), Some(media_type)) => {
                    member_idents.push(as_ident(name));
                    member_templates.push(media_type.trim().to_string());
                }
                _ => continue,
            }
        }
        Ok(Self {
            name,
            member_idents,
            member_templates,
        })
    }

    fn enum_definition(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let member_idents = &self.member_idents;
        quote::quote! {
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum #name {
                #(#member_idents,)*
                Other(String),
            }
        }
    }
}

fn as_ident(name: &str) -> syn::Ident {
    let name = name.replace(['.', '+'], "-").to_pascal_case();
    if name.starts_with(|c: char| c.is_digit(10)) {
        quote::format_ident!("_{}", name)
    } else {
        quote::format_ident!("{}", name)
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.enum_definition())?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let application = MediaType::new(
        quote::format_ident!("Application"),
        "https://www.iana.org/assignments/media-types/application.csv",
    )?;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../iana-media-types")
        .canonicalize()?;
    fs::write(root.join("src/application.rs"), format!("{}", application))?;

    Command::new("cargo").arg("fmt").spawn()?;

    Ok(())
}
