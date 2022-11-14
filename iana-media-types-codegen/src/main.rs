use anyhow::Result;
use inflector::Inflector;

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
            .filter(|line| !line.contains("OBSOLETED"))
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
            pub enum #name {
                #(#member_idents),*
            }
        }
    }
}

fn as_ident(name: &str) -> syn::Ident {
    let name = name.replace(['.', '+'], "-").to_class_case();
    if name.starts_with(|c: char| c.is_digit(10)) {
        quote::format_ident!("_{}", name)
    } else {
        quote::format_ident!("{}", name)
    }
}

fn main() -> Result<()> {
    let application = MediaType::new(
        quote::format_ident!("MediaType"),
        "https://www.iana.org/assignments/media-types/application.csv",
    )?;
    println!("{}", application.enum_definition());
    Ok(())
}
