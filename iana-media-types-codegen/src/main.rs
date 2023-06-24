use anyhow::Result;
use inflector::Inflector;
use std::{collections::HashMap, fmt, fs, path::PathBuf, process::Command};

#[derive(Debug, Clone)]
struct MediaType {
    name: syn::Ident,
    member_idents: Vec<syn::Ident>,
    member_templates: Vec<String>,
    member_extensions: Vec<Vec<String>>,
}

impl MediaType {
    fn new(name: syn::Ident, url: &str, extensions_map: &ExtensionsMap) -> Result<Self> {
        let csv = ureq::get(url).call()?.into_string()?;
        let mut member_idents = Vec::new();
        let mut member_templates = Vec::new();
        let mut member_extensions = Vec::new();
        for line in csv
            .lines()
            .skip(1 /* CSV header */)
            .filter(|line| !line.contains("OBSOLETE") && !line.contains("DEPRECATED"))
        {
            let mut iter = line.split(',');
            match (iter.next(), iter.next()) {
                (Some(name), Some(ty)) => {
                    member_idents.push(as_ident(name));

                    let mime_type = ty.trim();
                    if mime_type.is_empty() {
                        continue;
                    }

                    let mut extensions_vec = Vec::new();
                    if let Some(extenstions) = extensions_map.get(mime_type) {
                        extensions_vec = extenstions
                            .into_iter()
                            .map(|extension_str| extension_str.to_string())
                            .collect();
                    }

                    member_templates.push(mime_type.to_string());
                    member_extensions.push(extensions_vec)
                }
                _ => continue,
            }
        }
        Ok(Self {
            name,
            member_idents,
            member_templates,
            member_extensions,
        })
    }

    fn enum_definition(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let member_idents = &self.member_idents;
        let member_templates = &self.member_templates;
        let member_extensions = &self.member_extensions;
        quote::quote! {
            #[derive(
                Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
                ::serde::Serialize, ::serde::Deserialize
            )]
            pub enum #name {
                #(
                #[doc = #member_templates]
                #[serde(rename = #member_templates)]
                #(
                #[serde(alias = #member_extensions)]
                )*
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
        let member_extensions = &self.member_extensions;
        quote::quote! {
            impl ::std::str::FromStr for #name {
                type Err = ();
                fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
                    match input {
                        #(
                        #member_templates #(| #member_extensions)* => Ok(#name::#member_idents),
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

    let etc_mime_types_string = ureq::get("https://pagure.io/mailcap/raw/master/f/mime.types")
        .call()?
        .into_string()?;
    let extentions_map = expentions_map(&etc_mime_types_string)?;

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
        let media_type = MediaType::new(quote::format_ident!("{}", ident), &url, &extentions_map)?;
        fs::write(path, format!("{}", media_type))?;
    }

    Command::new("cargo").arg("fmt").current_dir(root).spawn()?;
    Ok(())
}

type ExtensionsMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn expentions_map<'a>(etc_mime_types_string: &str) -> Result<ExtensionsMap<'_>> {
    let mut extensions_map: ExtensionsMap = HashMap::new();
    let delimeters = [' ', '\t'];

    etc_mime_types_string
        .lines()
        .filter(|line| !line.starts_with('#') && line.contains(delimeters))
        .map(|line| {
            line.split(delimeters)
                .filter(|str| !str.is_empty())
                .collect::<Vec<&str>>()
        })
        .for_each(|line| {
            let mut iter = line.into_iter();
            extensions_map.insert(iter.next().expect("MIMI type"), iter.collect());
        });

    Ok(extensions_map)
}
