use capitaine::elements::{Button, ButtonPressed, Label};
use capitaine::{make_row, ElementTree, ElementTreeExt, NoEvent, RootHandler};

#[derive(Debug, Default, Clone, PartialEq)]
struct HelloBoxState {
    count: i32,
}

fn hello_box(state: &HelloBoxState, _props: ()) -> impl ElementTree<HelloBoxState, NoEvent> {
    make_row!(
        Button::new("Say hello").on::<ButtonPressed, _>(|state: &mut HelloBoxState, _| {
            println!("Hello world - {}", state.count);
            state.count += 1;
        }),
        Label::new(format!("Hello count: {}", state.count)),
    )
}

fn main() -> Result<(), druid::PlatformError> {
    let state = HelloBoxState { count: 0 };

    RootHandler::new(&hello_box, state)
        .with_tracing(true)
        .launch()
}
