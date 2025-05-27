use quote::quote;
use serde::Deserialize;
use serde_tokenstream::from_tokenstream;
use syn::Error;

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
pub fn stdlib(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    do_output(do_stdlib(attr.into(), item.into()))
}

#[derive(Deserialize)]
struct Metadata {
    name: String,
}

fn do_stdlib(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> Result<(proc_macro2::TokenStream, Vec<Error>), Error> {
    let metadata = from_tokenstream(&attr)?;
    do_stdlib_inner(metadata, attr, item)
}

fn do_stdlib_inner(
    metadata: Metadata,
    _attr: proc_macro2::TokenStream,
    _item: proc_macro2::TokenStream,
) -> Result<(proc_macro2::TokenStream, Vec<Error>), Error> {
    let name = metadata.name;
    let errors = Vec::new();
    let tokens = quote! {
        const NAME: &'static str = #name;
    };
    Ok((tokens, errors))
}

fn do_output(
    res: Result<(proc_macro2::TokenStream, Vec<Error>), Error>,
) -> proc_macro::TokenStream {
    match res {
        Err(err) => err.to_compile_error().into(),
        Ok((out, errors)) => {
            let compiler_errors = errors.iter().map(|err| err.to_compile_error());

            let output = quote! {
                #out
                #( #compiler_errors )*
            };

            output.into()
        }
    }
}
