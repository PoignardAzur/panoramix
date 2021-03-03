use panoramix::elements::{Button, Label};
use panoramix::{component, Column, CompCtx, Element, NoEvent, RootHandler};

#[component]
fn HelloBox(_ctx: &CompCtx, _props: ()) -> impl Element {
    Column!(
        Label::new("Hello world!"),
        Button::new("Say hello").on_click(|_, _| {
            println!("Hello world");
        })
    )
}

fn main() -> Result<(), druid::PlatformError> {
    RootHandler::new(HelloBox::new(()))
        .with_tracing(true)
        .launch()
}
