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

/// If the macro encountered a fatal error, return Err.
/// If the macro encountered some errors, but was also able to construct some source code,
/// returns Ok with a non-empty vec of errors.
fn do_stdlib_inner(
    metadata: Metadata,
    _attr: proc_macro2::TokenStream,
    _item: proc_macro2::TokenStream,
) -> Result<(proc_macro2::TokenStream, Vec<Error>), Error> {
    let name = metadata.name;
    // Try to return errors in this vec, rather than returning them with ? or `return Err`.
    // This way Rust Analyzer still gets some valid Rust source code to work with.
    // E.g. if you're looping over some user input, prefer to append errors and `continue` rather than
    // using ? or stopping the loop.
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

            // Put all the output we received first,
            // then emit compiler errors about whatever the user got wrong.
            let output = quote! {
                #out
                #( #compiler_errors )*
            };

            output.into()
        }
    }
}
