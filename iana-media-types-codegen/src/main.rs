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
            .filter(|line| !line.contains("OBSOLETE") && !line.contains("DEPRECATED"))
        {
            let mut iter = line.split(',');
            match (iter.next(), iter.next()) {
                (Some(name), Some(ty)) => {
                    let ty = ty.trim();
                    if ty.is_empty() {
                        continue;
                    }
                    member_idents.push(as_ident(name));
                    member_templates.push(ty.to_string());
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
        let member_templates = &self.member_templates;
        quote::quote! {
            #[derive(
                Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
                ::serde::Serialize, ::serde::Deserialize
            )]
            pub enum #name {
                #(
                #[doc = #member_templates]
                #[serde(alias = #member_templates)]
                #member_idents,
                )*
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
            impl ::std::str::FromStr for #name {
                type Err = ();
                fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
                    match input {
                        #(
                        #member_templates => Ok(#name::#member_idents),
                        )*
                        _ => Err(()),
                    }
                }
            }
        }
    }
}

fn as_ident(name: &str) -> syn::Ident {
    let mut escaped = name.replace(['.', '+'], "-").to_pascal_case();
    if escaped.starts_with(|c: char| c.is_ascii_digit()) {
        escaped = format!("_{}", escaped);
    }
    if name.ends_with('+') {
        escaped = format!("{}_", escaped);
    }
    quote::format_ident!("{}", escaped)
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
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../iana-media-types")
        .canonicalize()?;

    for (name, ident) in [
        ("application", "Application"),
        ("audio", "Audio"),
        ("font", "Font"),
        ("image", "Image"),
        ("message", "Message"),
        ("model", "Model"),
        ("multipart", "Multipart"),
        ("text", "Text"),
        ("video", "Video"),
    ] {
        let path = root.join(format!("src/{}.rs", name));
        let url = format!("https://www.iana.org/assignments/media-types/{}.csv", name);
        let media_type = MediaType::new(quote::format_ident!("{}", ident), &url)?;
        fs::write(path, format!("{}", media_type))?;
    }

    Command::new("cargo").arg("fmt").current_dir(root).spawn()?;
    Ok(())
}
