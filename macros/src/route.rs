use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

enum RequestType {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
}

#[derive(Debug, FromMeta)]
struct RouteArgs {
    #[darling(rename = "GET")]
    #[darling(default)]
    is_get: bool,
    #[darling(rename = "POST")]
    #[darling(default)]
    is_post: bool,
    #[darling(rename = "PUT")]
    #[darling(default)]
    is_put: bool,
    #[darling(rename = "DELETE")]
    #[darling(default)]
    is_delete: bool,
    #[darling(rename = "HEAD")]
    #[darling(default)]
    is_head: bool,
    #[darling(rename = "OPTIONS")]
    #[darling(default)]
    is_options: bool,
    #[darling(rename = "TRACE")]
    #[darling(default)]
    is_trace: bool,

    url: String,
}

pub(crate) fn route_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let statements = block.stmts;

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match RouteArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let RouteArgs {
        is_get,
        is_post,
        is_put,
        is_delete,
        is_head,
        is_options,
        is_trace,

        url,
    } = args;

    quote! {
        #(#attrs)*
        #vis #sig {
            #(#statements)*

           println!("{} {} {}", #is_get, #is_post, #url);
        }
    }
    .into()
}
