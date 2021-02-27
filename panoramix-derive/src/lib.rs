extern crate proc_macro;

mod component;

// ---

use proc_macro::TokenStream;
use syn::parse_macro_input;
use tracing::trace;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    trace!("attr: {:?}", attr);
    trace!("item: {:?}", item);

    let attr = proc_macro2::TokenStream::from(attr);
    let fn_item = parse_macro_input!(item as syn::ItemFn);

    trace!("fn_item: {:?}", fn_item);

    proc_macro::TokenStream::from(component::component(attr, fn_item))
}
