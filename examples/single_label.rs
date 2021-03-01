use panoramix::elements::{Button, ButtonPressed, Label};
use panoramix::{component, make_row, CompCtx, Element, ElementExt, NoEvent, RootHandler};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HelloBoxState {
    count: i32,
}

#[component]
fn HelloBox(ctx: &CompCtx, _props: ()) -> impl Element<HelloBoxState, NoEvent> {
    let state = ctx.use_local_state::<HelloBoxState>();
    make_row!(
        Button::new("Say hello").on::<ButtonPressed, _>(|state: &mut HelloBoxState, _| {
            println!("Hello world - {}", state.count);
            state.count += 1;
        }),
        Label::new(format!("Hello count: {}", state.count)),
    )
}

fn main() -> Result<(), druid::PlatformError> {
    RootHandler::new(HelloBox::new(()))
        .with_tracing(true)
        .launch()
}
