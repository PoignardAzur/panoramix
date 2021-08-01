extern crate proc_macro;

mod component;

// ---

use proc_macro::TokenStream;
use syn::parse_macro_input;
use tracing::trace;

/// Attribute used to declare a component
///
/// The attribute applies to a function, but will transform that function into a type, with a `new()` method. By convention, the name of your component should be in UpperCamelCase, and the compiler will warn you if it isn't.
///
/// Example:
///
/// ```rust
/// # use panoramix::{component, CompCtx, Element, NoEvent};
/// # type EventType = NoEvent;
/// # type PropsType = ();
/// # type LocalState = ();
/// #
/// #[component]
/// fn MyComponent(ctx: &CompCtx, props: PropsType) -> impl Element<EventType, LocalState> {
///     // ...
///     # panoramix::elements::EmptyElement::new()
/// }
/// ```
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    trace!("attr: {:?}", attr);
    trace!("item: {:?}", item);

    let attr = proc_macro2::TokenStream::from(attr);
    let fn_item = parse_macro_input!(item as syn::ItemFn);

    trace!("fn_item: {:?}", fn_item);

    proc_macro::TokenStream::from(
        component::component(attr, fn_item).unwrap_or_else(|error| error.to_compile_error()),
    )
}
