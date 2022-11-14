use anyhow::Result;
use inflector::Inflector;

fn as_ident(name: &str) -> syn::Ident {
    let name = name.replace(['.', '+'], "-").to_class_case();
    if name.starts_with(|c: char| c.is_digit(10)) {
        quote::format_ident!("_{}", name)
    } else {
        quote::format_ident!("{}", name)
    }
}

fn get_media_type_from_url(url: &str) -> Result<Vec<(syn::Ident, String)>> {
    let csv = ureq::get(url).call()?.into_string()?;
    Ok(csv
        .lines()
        .filter_map(|line| {
            if line.contains("OBSOLETED") {
                return None;
            }
            let mut iter = line.split(',');
            match (iter.next(), iter.next()) {
                (Some(name), Some(media_type)) => {
                    Some((as_ident(name.trim()), media_type.trim().to_string()))
                }
                _ => None,
            }
        })
        .collect())
}

fn main() -> Result<()> {
    let application =
        get_media_type_from_url("https://www.iana.org/assignments/media-types/application.csv")?;
    dbg!(application);
    Ok(())
}
