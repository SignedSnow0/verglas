mod route;

use proc_macro::TokenStream;
use route::route_impl;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    route_impl(args, input)
}
