/*!
This crate adds support for error handling in [trillium](https://trillium.rs) web framework.

Due to limitations in Rust, error handling is currently not supported in trillium. When the language
adds capability to express bounds for `for<'a> Fn(&'a Conn) -> impl Future<Output=…> + 'a`, trillium
will add first class support for error handling. Until then `trillium-error` provides
a proc macro to help write handlers with error. For more details please refer to the discussion
[here](https://github.com/trillium-rs/trillium/discussions/31).

```ignore
use trillium_error::handler;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Custom error")]
    CustomError,
    #[error("IO error")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

#[async_trait]
impl Handler for AppError {
    async fn run(&self, conn: Conn) -> Conn {
        conn.with_status(500).with_body("Internal Server Error")
    }
}

#[handler]
async fn helloworld(conn: &mut Conn) -> Result<(), AppError> {
    conn.set_status(200);
    conn.set_body("hello world");
    // Ok(())
    Err(AppError::CustomError)
}

fn main() {
    trillium_tokio::run(helloworld);
}
```
*/
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
