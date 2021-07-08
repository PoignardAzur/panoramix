use panoramix::elements::{Button, Label};
use panoramix::{component, Column, CompCtx, Element, NoEvent, RootHandler};

#[component]
fn HelloBox(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, ()>();
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
