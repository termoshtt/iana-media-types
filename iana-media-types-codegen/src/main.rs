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

    fn impl_display(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let member_idents = &self.member_idents;
        let member_templates = &self.member_templates;
        quote::quote! {
            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    match self {
                        #(#name::#member_idents => write!(f, #member_templates)?,)*
                        #name::Other(template) => write!(f, "{}", template)?,
                    }
                    Ok(())
                }
            }
        }
    }

    fn impl_from_str(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let member_idents = &self.member_idents;
        let member_templates = &self.member_templates;
        quote::quote! {
            impl From<&str> for #name {
                fn from(input: &str) -> Self {
                    match input {
                        #(
                        #member_templates => #name::#member_idents,
                        )*
                        _ => #name::Other(input.to_string()),
                    }
                }
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
        write!(f, "{}", self.impl_display())?;
        write!(f, "{}", self.impl_from_str())?;
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
