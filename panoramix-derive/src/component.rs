use proc_macro2::TokenStream;
use quote::quote;
use tracing::trace;

pub fn component(attr: TokenStream, fn_item: syn::ItemFn) -> TokenStream {
    #![allow(non_snake_case)]
    assert!(attr.is_empty());

    // Types used:
    // - syn::ItemFn
    // - syn::Signature
    // - syn::FnArg

    // TODO
    // - fn_item.attr

    let fn_visibility = fn_item.vis;

    let fn_constness = fn_item.sig.constness;
    let fn_asyncness = fn_item.sig.asyncness;
    let fn_unsafety = fn_item.sig.unsafety;
    let fn_abi = fn_item.sig.abi;

    let component_name = fn_item.sig.ident;

    let fn_generics = fn_item.sig.generics;
    let fn_args = fn_item.sig.inputs;
    let fn_variadic = fn_item.sig.variadic;

    let fn_output = fn_item.sig.output;

    let fn_block = fn_item.block;

    assert!(fn_constness.is_none());
    assert!(fn_asyncness.is_none());
    assert!(fn_unsafety.is_none());
    assert!(fn_abi.is_none());

    assert!(fn_generics.params.is_empty());
    assert!(fn_generics.where_clause.is_none());
    assert!(fn_variadic.is_none());

    assert!(fn_args.len() == 2);
    let ctx_arg;
    let props_arg;
    if let (syn::FnArg::Typed(pattern1), syn::FnArg::Typed(pattern2)) =
        (fn_args.first().unwrap(), fn_args.last().unwrap())
    {
        ctx_arg = pattern1;
        props_arg = pattern2;
    } else {
        panic!("Argument cannot be self")
    };
    let props_ty = *props_arg.ty.clone();

    let fn_output = if let syn::ReturnType::Type(_, ty) = fn_output {
        *ty
    } else {
        panic!()
    };

    let (local_event_ty, local_state_ty) = parse_return_ty(fn_output.clone());

    // TODO
    // - Error message if user tries to do MyComponent(props) instead of MyComponent::new(props)

    // TODO - only pub when input declaration is pub

    let vis = fn_visibility;
    let ComponentName = component_name.clone();
    let ComponentName_literal = proc_macro2::Literal::string(&component_name.to_string());
    let PropsType = props_ty;
    let LocalEvent = local_event_ty;
    let LocalState = local_state_ty;

    quote! {
        #[derive(Debug, Default, Clone, PartialEq, Hash)]
        #vis struct #ComponentName;

        impl #ComponentName {
            #vis fn new<ParentCpEvent, ParentCpState>(
                props: #PropsType,
            ) -> impl panoramix::Element<ParentCpEvent, ParentCpState, Event=#LocalEvent> {
                <Self as panoramix::elements::Component>::new(props)
            }

            #vis fn render(
                #ctx_arg,
                #props_arg,
            ) -> #fn_output {
                #fn_block
            }
        }

        impl panoramix::elements::Component for #ComponentName {
            type Props = #PropsType;
            type LocalEvent = #LocalEvent;
            type LocalState = #LocalState;

            fn new<ParentCpEvent, ParentCpState>(
                props: Self::Props,
            ) -> panoramix::elements::backend::ComponentHolder<Self, ParentCpEvent, ParentCpState>
            {
                panoramix::elements::backend::ComponentHolder::new(#ComponentName, props)
            }

            fn name() -> &'static str {
                #ComponentName_literal
            }

            fn call_indirect<ParentCpEvent, ParentCpState>(
                &self,
                prev_state: (
                    Vec<Self::LocalEvent>,
                    Self::LocalState,
                    Option<panoramix::elements::any_element::AnyStateBox>,
                ),
                props: Self::Props,
            ) -> (
                panoramix::elements::backend::ComponentOutput<
                    Self::LocalEvent,
                    Self::LocalState,
                    panoramix::elements::any_element::VirtualDomBox<Self::LocalEvent, Self::LocalState>,
                    ParentCpEvent,
                    ParentCpState,
                >,
                (
                    Vec<Self::LocalEvent>,
                    Self::LocalState,
                    Option<panoramix::elements::any_element::AnyStateBox>,
                ),
            ) {
                panoramix::elements::backend::ComponentHolder::build_with(
                    #ComponentName,
                    &#ComponentName::render,
                    prev_state,
                    props,
                )
            }
        }
    }
}

#[allow(dead_code)]
fn get_arg_ident(pattern: syn::Pat) -> syn::Ident {
    trace!("pattern: {:?}", pattern);

    if let syn::Pat::Ident(ident) = pattern {
        ident.ident
    } else {
        panic!("Argument must be an identifier pattern")
    }
}

fn parse_return_ty(return_ty: syn::Type) -> (syn::Type, syn::Type) {
    // TODO - Handle return types `impl Element`, `impl Element<Event>`

    // Uses syn::TypeImplTrait
    let impl_trait = if let syn::Type::ImplTrait(impl_trait) = return_ty {
        impl_trait
    } else {
        panic!("Component must return impl Element")
    };

    let element_trait = if let syn::TypeParamBound::Trait(element_trait) =  impl_trait.bounds.first().unwrap() {
        element_trait
    } else {
        panic!("Component must return impl Element")
    };

    let last_segment = element_trait.path.segments.last().unwrap();
    let elements_ty_args = last_segment.arguments.clone();
    assert!(last_segment.ident.to_string() == "Element");

    // AngleBracketedGenericArguments
    let elements_ty_args: Vec<_> = match elements_ty_args {
        syn::PathArguments::None => {
            Vec::new()
        }
        syn::PathArguments::AngleBracketed(elements_ty_args) => {
            elements_ty_args.args.into_iter().collect()
        }
        _ => {
            panic!("Component must return impl Element<LocalState, LocalEvent>")
        }
    };

    use syn::parse_quote;
    let default_event_ty: syn::GenericArgument = parse_quote!( NoEvent );
    let default_state_ty: syn::GenericArgument = parse_quote!( () );

    assert!(elements_ty_args.len() <= 2);
    let local_event_ty = elements_ty_args.get(0).cloned().unwrap_or(default_event_ty);
    let local_state_ty = elements_ty_args.get(1).cloned().unwrap_or(default_state_ty);

    let local_state_ty = if let syn::GenericArgument::Type(local_state_ty) = local_state_ty {
        local_state_ty
    } else {
        panic!("Component must return impl Element<LocalEvent, LocalState>")
    };
    let local_event_ty = if let syn::GenericArgument::Type(local_event_ty) = local_event_ty {
        local_event_ty
    } else {
        panic!("Component must return impl Element<LocalEvent, LocalState>")
    };

    (local_event_ty.clone(), local_state_ty.clone())
}
