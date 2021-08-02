extern crate proc_macro;

mod component;

// ---

use proc_macro::TokenStream;
use syn::parse_macro_input;
use tracing::trace;

/// Attribute used to declare a component
///
/// The attribute applies to a function, but will transform that function into a type, which:
/// - Implements the [`Component`](panoramix::elements::Component) trait.
/// - Has an inherent `new` method of the form
///   ```
///   # use panoramix::{Element, NoEvent};
///   # type MyEventType = NoEvent;
///   # type MyPropsType = ();
///   fn new(props: MyPropsType) -> impl panoramix::Element<Event = MyEventType>
///   # { panoramix::elements::EmptyElement::new() }
///   ```
///
/// By convention, the name of your component should be in UpperCamelCase, and the compiler will warn you if it isn't.
///
/// ## Example:
///
/// ```rust
/// # use panoramix::{component, CompCtx, Element, NoEvent};
/// # type MyEventType = NoEvent;
/// # type MyPropsType = ();
/// # type LocalState = ();
/// #
/// #[component]
/// fn MyComponent(ctx: &CompCtx, props: MyPropsType) -> impl Element<Event = MyEventType> {
///     // ...
///     # panoramix::elements::EmptyElement::new()
/// }
///
/// // ...
///
/// # let my_props = ();
/// MyComponent::new(my_props)
/// # ;
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
