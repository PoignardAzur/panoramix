use panoramix::elements::{Button, Label};
use panoramix::{component, Column, CompCtx, Element, Metadata, NoEvent, RootHandler};

#[component]
fn HelloBox(_ctx: &CompCtx, _props: ()) -> impl Element {
    let md = Metadata::<NoEvent, ()>::new();
    Column!(
        Label::new("Hello world!"),
        Button::new("Say hello").on_click(md, |_, _| {
            println!("Hello world");
        })
    )
}

fn main() -> Result<(), panoramix::PlatformError> {
    RootHandler::new(HelloBox).with_tracing(true).launch()
}
