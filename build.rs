
use std::io::Write;

fn main() {
    let stream: proc_macro2::TokenStream = std::fs::read_to_string("src/views.rs").expect("Unable to read views.rs").parse().expect("Unable to parse views.rs");
    let item: syn::File = syn::parse2(stream).unwrap();
    let mut f = std::fs::File::create("test.txt").unwrap();
    for item in item.items {
        match item {
            syn::Item::Enum(enm) => {
                writeln!(&mut f, "{:?}: {{", enm.ident.to_string()).unwrap();
                for variant in enm.variants {
                    writeln!(&mut f, "\t{:?}: {{", variant.ident.to_string()).unwrap();
                    for field in variant.fields {
                        writeln!(&mut f, "\t\t{:?}: {:?}", field.ident.and_then(|f| Some(f.to_string())).or(Some("".to_string())).unwrap(), "").unwrap();
                    }
                    writeln!(&mut f, "\t}}").unwrap();
                }
                writeln!(&mut f, "}}").unwrap();
            },
            _ => {},
        }
    }
}