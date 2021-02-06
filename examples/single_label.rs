use capitaine::element_tree::{ElementTree, ElementTreeExt, NoEvent};
use capitaine::elements::{Button, ButtonPressed, Label};
use capitaine::glue::DruidAppData;
use capitaine::make_row;
use capitaine::root_handler::RootHandler;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

#[derive(Debug, Default, Clone)]
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

fn ui_builder() -> impl Widget<DruidAppData> {
    let state = AppState { count: 0 };

    RootHandler::new(&single_label, state)
}

fn main() -> Result<(), PlatformError> {
    capitaine::glue::init_tracing();

    let main_window = WindowDesc::new(ui_builder());
    let data = Default::default();
    AppLauncher::with_window(main_window).launch(data)
}
