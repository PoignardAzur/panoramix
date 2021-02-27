use proc_macro2::TokenStream;
use quote::quote;
use tracing::trace;

pub fn component(attr: TokenStream, fn_item: syn::ItemFn) -> TokenStream {
    assert!(attr.is_empty());

    // Types used:
    // - syn::ItemFn
    // - syn::Signature
    // - syn::FnArg

    // TODO
    // - fn_item.attr
    // - fn_item.vis

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
    let props_ty = props_arg.ty.clone();
    let props_ident = get_arg_ident(*props_arg.pat.clone());

    let fn_output = if let syn::ReturnType::Type(_, ty) = fn_output {
        *ty
    } else {
        panic!()
    };

    // TODO
    // - Error message if user tries to do MyComponent(props) instead of MyComponent::new(props)

    quote! {
        #[derive(Debug)]
        pub struct #component_name(
            #props_ty
        );

        impl #component_name {
            pub fn new(#props_arg) -> Self {
                Self(#props_ident)
            }

            pub fn render(#ctx_arg, #props_arg) -> #fn_output {
                #fn_block
            }
        }
    }
}

fn get_arg_ident(pattern: syn::Pat) -> syn::Ident {
    trace!("pattern: {:?}", pattern);

    if let syn::Pat::Ident(ident) = pattern {
        ident.ident
    } else {
        panic!("Argument must be an identifier pattern")
    }
}
