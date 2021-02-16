use capitaine::element_tree::{ElementTree, ElementTreeExt, NoEvent};
use capitaine::elements::{Button, ButtonPressed, Label};
use capitaine::make_row;
use capitaine::root_handler::RootHandler;

use druid::PlatformError;

#[derive(Debug, Default, Clone, PartialEq)]
struct AppState {
    count: i32,
}

fn single_label(state: &AppState, _props: ()) -> impl ElementTree<AppState, NoEvent> {
    make_row!(
        Button::new("Increase").on::<ButtonPressed, _>(|state: &mut AppState, _| {
            state.count += 1;
        }),
        Label::new(format!("Count: {}", state.count)),
    )
}

fn main() -> Result<(), PlatformError> {
    let state = AppState { count: 0 };

    RootHandler::new(&single_label, state)
        .with_tracing(true)
        .launch()
}
