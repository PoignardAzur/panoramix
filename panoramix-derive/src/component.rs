use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Display;
use syn::Error;

pub fn component(attr: TokenStream, fn_item: syn::ItemFn) -> Result<TokenStream, Error> {
    #![allow(non_snake_case)]

    fn error(tokens: impl ToTokens, message: impl Display) -> Result<TokenStream, Error> {
        Err(Error::new_spanned(tokens, message))
    }

    if !attr.is_empty() {
        return error(attr, "#[component] attribute doesn't take parameters");
    }

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

    if fn_constness.is_some() {
        return error(
            fn_constness,
            "#[component] attribute doesn't support const functions",
        );
    }
    if fn_asyncness.is_some() {
        return error(
            fn_asyncness,
            "#[component] attribute doesn't support async functions",
        );
    }
    if fn_unsafety.is_some() {
        return error(
            fn_unsafety,
            "#[component] attribute doesn't support unsafe functions",
        );
    }
    if fn_abi.is_some() {
        return error(
            fn_abi,
            "#[component] attribute doesn't support non-standard ABIs",
        );
    }

    if !fn_generics.params.is_empty() {
        return error(
            fn_generics.params,
            "#[component] attribute doesn't support generic parameters",
        );
    }
    if fn_generics.where_clause.is_some() {
        return error(
            fn_generics.where_clause,
            "#[component] attribute doesn't support where clauses",
        );
    }
    if fn_variadic.is_some() {
        return error(
            fn_variadic,
            "#[component] attribute doesn't support variadic arguments",
        );
    }

    if fn_args.len() != 2 {
        let len = fn_args.len();
        return error(
            fn_args,
            format!(
                "error in #[component] function: expected 2 arguments, found {}",
                len
            ),
        );
    }

    let ctx_arg;
    let props_arg;
    match fn_args.first().unwrap() {
        syn::FnArg::Typed(pattern) => ctx_arg = pattern,
        syn::FnArg::Receiver(receiver) => {
            return error(
                receiver,
                "error in #[component] function: argument cannot be self",
            );
        }
    }
    match fn_args.last().unwrap() {
        syn::FnArg::Typed(pattern) => props_arg = pattern,
        syn::FnArg::Receiver(receiver) => {
            return error(
                receiver,
                "error in #[component] function: argument cannot be self",
            );
        }
    }
    let props_ty = *props_arg.ty.clone();

    let fn_output = match fn_output {
        syn::ReturnType::Type(_, ty) => *ty,
        syn::ReturnType::Default => {
            return error(
                component_name,
                "error in #[component] function: missing return type",
            );
        }
    };

    let local_event_ty = if let Some(local_event_ty) = parse_return_ty(fn_output.clone())? {
        local_event_ty
    } else {
        return error(
            fn_output,
            "error in #[component] function: missing bound for associated type `Event`",
        );
    };

    // TODO
    // - Error message if user tries to do MyComponent(props) instead of MyComponent::new(props)

    let vis = fn_visibility;
    let ComponentName = component_name.clone();
    let ComponentName_literal = proc_macro2::Literal::string(&component_name.to_string());
    let PropsType = props_ty;
    let LocalEvent = local_event_ty;

    Ok(quote! {
        #[derive(Debug, Default, Clone, PartialEq, Hash)]
        #vis struct #ComponentName;

        impl #ComponentName {
            #vis fn new(
                props: #PropsType,
            ) -> impl panoramix::Element<Event=#LocalEvent> {
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

            fn new(
                props: Self::Props,
            ) -> panoramix::elements::ElementBox<#LocalEvent>
            {
                panoramix::elements::ElementBox::new(
                    panoramix::elements::internals::ComponentHolder::<Self, _, _>::new(&#ComponentName::render, props)
                )
            }

            fn name() -> &'static str {
                #ComponentName_literal
            }
        }
    })
}

fn parse_return_ty(return_ty: syn::Type) -> Result<Option<syn::Type>, Error> {
    fn error(tokens: impl ToTokens, message: impl Display) -> Result<Option<syn::Type>, Error> {
        Err(Error::new_spanned(tokens, message))
    }

    let impl_trait = if let syn::Type::ImplTrait(impl_trait) = return_ty {
        impl_trait
    } else {
        // Possible error case: '-> i32'
        return error(
            return_ty,
            "error in #[component] function: return type must be `impl Element</* ... */>`",
        );
    };
    if impl_trait.bounds.len() != 1 {
        // Possible error case: '-> impl Element + Default'
        return error(impl_trait, "error in #[component] function: return type must be `impl Element</* ... */>` with no other bounds");
    }
    let element_trait = if let syn::TypeParamBound::Trait(element_trait) =
        impl_trait.bounds.first().unwrap()
    {
        element_trait
    } else {
        // This error case should be impossible
        return error(impl_trait, "error in #[component] function: return type must be `impl Element</* ... */>` with no other bounds");
    };

    let last_segment = element_trait.path.segments.last().unwrap();
    let elements_ty_args = last_segment.arguments.clone();

    // AngleBracketedGenericArguments
    let elements_ty_args: Vec<_> = match elements_ty_args {
        syn::PathArguments::None => {
            return Ok(None);
        }
        syn::PathArguments::AngleBracketed(elements_ty_args) => {
            elements_ty_args.args.into_iter().collect()
        }
        syn::PathArguments::Parenthesized(_) => {
            return Ok(None);
        }
    };

    if elements_ty_args.len() == 0 {
        return Ok(None);
    }

    let local_event_ty = elements_ty_args.into_iter().find_map(|arg| match arg {
        syn::GenericArgument::Binding(local_event_bind)
            if local_event_bind.ident.to_string() == "Event" =>
        {
            Some(local_event_bind.ty)
        }
        _ => None,
    });

    Ok(local_event_ty)
}
