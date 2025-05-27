use super::*;

#[test]
fn it_works() {
    let (item, errors) = do_stdlib(
        quote! {
            name = "Adam",
        },
        quote! {
            /// This is the item upon which
            /// the macro is being called.
            fn foo() {}
        },
    )
    .unwrap();
    if !errors.is_empty() {
        panic!("{errors:?}");
    }
    expectorate::assert_contents("tests/it_works.gen", &get_text_fmt(&item));
}

fn get_text_fmt(out: &proc_macro2::TokenStream) -> String {
    dbg!(out.to_string());
    let syntax_tree = syn::parse_file(&out.to_string()).unwrap();
    prettyplease::unparse(&syntax_tree)
}
