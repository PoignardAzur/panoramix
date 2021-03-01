use panoramix::elements::{Label, TextBox, TextChanged};
use panoramix::{component, make_column, CompCtx, Element, ElementExt, NoEvent, RootHandler};

use druid::PlatformError;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HelloState {
    name: String,
}

#[component]
fn HelloComponent(ctx: &CompCtx, _props: ()) -> impl Element<HelloState, NoEvent> {
    let state = ctx.use_local_state::<HelloState>();

    let message = if state.name.is_empty() {
        String::from("Enter your name")
    } else {
        format!("Hello, {}!", state.name)
    };

    let textbox = TextBox::new(state.name.clone()).on::<TextChanged, _>(
        |state: &mut HelloState, event: TextChanged| {
            state.name = event.0;
        },
    );

    make_column!(textbox, Label::new(message))
}

fn main() -> Result<(), PlatformError> {
    RootHandler::new(HelloComponent::new(()))
        .with_tracing(true)
        .launch()
}
