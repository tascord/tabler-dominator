use std::{collections::HashMap, path::Path, sync::Mutex};

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

lazy_static! {
    static ref ICONS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[proc_macro]
pub fn icon(item: TokenStream) -> TokenStream {
    icon_impl(item).unwrap_or_else(|e| e)
}

fn icon_impl(item: TokenStream) -> Result<TokenStream, TokenStream> {
    let item = item.into_iter().next().expect("No icon specified");
    let input = item
        .to_string()
        .strip_prefix('"')
        .expect("No leading quote?")
        .strip_suffix('"')
        .expect("No trailing quote?")
        .to_string();

    if !ICONS.lock().unwrap().contains_key(&input) {
        let path = Path::new(file!().strip_suffix("lib.rs").unwrap())
            .join("../svg/")
            .join(&format!("{input}.svg"))
            .canonicalize()
            .unwrap();

        let file = std::fs::read(path).map_err(|_| {
            return quote_spanned! {
                item.span().into() =>
                compile_error!("Invalid Tabler Icon");
            };
        })?;

        ICONS
            .lock()
            .unwrap()
            .insert(input.clone(), String::from_utf8_lossy(&file).to_string());
    }

    let icon = ICONS.lock().unwrap();
    let icon = icon.get(&input).unwrap();
    let exp = quote! {
        html!("div", { .with_node!(e => {
                .apply(|d| {
                    e.set_inner_html(&#icon);
                    d
                })
            })
        })
    };

    Ok(exp.into())
}
