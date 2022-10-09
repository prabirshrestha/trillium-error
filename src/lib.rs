use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn handler(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    let output: syn::Result<TokenStream> = match item {
        Item::Fn(mut item_fn) => {
            let attrs = &item_fn.attrs;
            let vis = &item_fn.vis;
            let sig = &mut item_fn.sig;
            let name = &sig.ident;
            let body = &item_fn.block;
            let docs = item_fn
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("doc"))
                .cloned()
                .collect::<Vec<_>>();

            Ok(quote! {
                #(#docs)*
                #[allow(non_camel_case_types)]
                #[derive(Debug)]
                #vis struct #name;

                impl #name {
                    #(#attrs)*
                    #sig {
                        #body
                    }
                }

                #[trillium::async_trait]
                impl trillium::Handler for #name {
                    async fn run(&self, mut conn: trillium::Conn) -> trillium::Conn {
                        match Self::#name(&mut conn).await {
                            Ok(_) => conn,
                            Err(e) => e.run(conn).await
                        }
                    }
                }
            }
            .into())
        }
        _ => Err(syn::Error::new_spanned(
            item,
            "#[handler] must added to `impl` or `fn`",
        )),
    };

    match output {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
