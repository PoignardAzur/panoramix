use capitaine::element_tree::{ElementTree, ElementTreeExt, NoEvent};
use capitaine::elements::{Button, ButtonPressed, ComponentCaller, ElementList, Label};
use capitaine::glue::DruidAppData;
use capitaine::root_handler::RootHandler;
use capitaine::widgets::flex::{CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use capitaine::{make_group, make_row};

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

const ROW_FLEX_PARAMS: FlexContainerParams = FlexContainerParams {
    cross_alignment: CrossAxisAlignment::Center,
    main_alignment: MainAxisAlignment::End,
    fill_major_axis: false,
};

#[derive(Debug, Default, Clone)]
struct ListItem {
    text: String,
    id: i32,
}

#[derive(Debug, Default, Clone)]
struct AppState {
    data: Vec<ListItem>,
    selected_row: Option<usize>,
    next_id: i32,
}

type RowEvent = ButtonPressed;
struct RowProps {
    list_item: ListItem,
    is_selected: bool,
}

fn list_row(state: &u16, props: RowProps) -> impl ElementTree<u16, RowEvent> {
    let age = *state;
    make_row!(
        Button::new("Select").map_event(|state: &mut u16, event| {
            *state += 1;
            Some(event)
        }),
        Label::new(if props.is_selected { " [*]" } else { " [ ]" }),
        Label::new(format!("{} - age={}", &props.list_item.text, age)),
        Label::new(format!("id={}", props.list_item.id)),
    )
    .with_flex_container_params(ROW_FLEX_PARAMS)
}

fn some_component(state: &AppState, _props: ()) -> impl ElementTree<AppState, NoEvent> {
    let button_create = Button::new("Create").on::<ButtonPressed, _>(|state: &mut AppState, _| {
        state.data.push(ListItem {
            text: "new item".to_string(),
            id: state.next_id,
        });
        state.next_id += 1;
    });
    let button_insert = Button::new("Insert").on::<ButtonPressed, _>(|state: &mut AppState, _| {
        state.data.insert(
            0,
            ListItem {
                text: "inserted item".to_string(),
                id: state.next_id,
            },
        );
        state.next_id += 1;
    });
    let button_delete = Button::new("Delete").on::<ButtonPressed, _>(|state: &mut AppState, _| {
        if let Some(row) = state.selected_row {
            state.data.remove(row as usize);
            state.selected_row = None;
        }
    });
    let button_update = Button::new("Update").on::<ButtonPressed, _>(|state: &mut AppState, _| {
        if let Some(row) = state.selected_row {
            state.data[row as usize].text = "updated".to_string();
        }
    });

    let list_view_data = state
        .data
        .iter()
        .enumerate()
        .map(|(i, list_item)| {
            let row_props = RowProps {
                list_item: list_item.clone(),
                is_selected: state.selected_row == Some(i),
            };

            let comp_builder = ComponentCaller::prepare(list_row, row_props);
            let comp_builder = comp_builder.on::<RowEvent, _>(move |state: &mut AppState, _| {
                state.selected_row = Some(i);
            });

            (list_item.id.to_string(), comp_builder)
        })
        .collect();
    let list_view = ElementList {
        children: list_view_data,
        _comp_state: Default::default(),
        _comp_event: Default::default(),
    };

    make_group!(
        make_row!(button_create, button_insert, button_delete, button_update,)
            .with_flex_container_params(ROW_FLEX_PARAMS),
        list_view,
    )
}

fn ui_builder() -> impl Widget<DruidAppData> {
    let state = AppState {
        data: (0..8_i32)
            .map(|i| ListItem {
                text: "hello".to_string(),
                id: i,
            })
            .collect(),
        selected_row: None,
        next_id: 8,
    };

    RootHandler::new(&some_component, state)
}

fn main() -> Result<(), PlatformError> {
    capitaine::glue::init_tracing();

    let main_window = WindowDesc::new(ui_builder);
    let data = Default::default();
    AppLauncher::with_window(main_window)
        .launch(data)
}
