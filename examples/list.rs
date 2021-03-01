use panoramix::elements::{Button, Checkbox, ElementList, Label, Toggled};
use panoramix::widgets::flex::{CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use panoramix::{component, CompCtx, Element, ElementExt, NoEvent, RootHandler, Row, Tuple};

use druid::PlatformError;

const ROW_FLEX_PARAMS: FlexContainerParams = FlexContainerParams {
    cross_alignment: CrossAxisAlignment::Center,
    main_alignment: MainAxisAlignment::Center,
    fill_major_axis: true,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListItem {
    text: String,
    id: i32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AppState {
    data: Vec<ListItem>,
    selected_row: Option<usize>,
    next_id: i32,
}

type RowEvent = Toggled;
// TODO - private type leak?
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RowProps {
    list_item: ListItem,
    is_selected: bool,
}

#[component]
fn MyListRow(ctx: &CompCtx, props: RowProps) -> impl Element<u16, RowEvent> {
    let age = ctx.use_local_state::<u16>();
    Row!(
        Checkbox::new("", props.is_selected).map_event(|state: &mut u16, event| {
            *state += 1;
            Some(event)
        }),
        Label::new(format!("{} - age={}", &props.list_item.text, age)),
        Label::new(format!("id={}", props.list_item.id)),
    )
    .with_flex_container_params(ROW_FLEX_PARAMS)
}

#[component]
fn AwesomeEditableList(ctx: &CompCtx, _props: ()) -> impl Element<AppState, NoEvent> {
    let state = ctx.use_local_state::<AppState>();

    let button_create = Button::new("Create").on_click(|state: &mut AppState, _| {
        state.data.push(ListItem {
            text: "new item".to_string(),
            id: state.next_id,
        });
        state.next_id += 1;
    });
    let button_insert = Button::new("Insert").on_click(|state: &mut AppState, _| {
        state.data.insert(
            0,
            ListItem {
                text: "inserted item".to_string(),
                id: state.next_id,
            },
        );
        state.next_id += 1;
    });
    let button_delete = Button::new("Delete").on_click(|state: &mut AppState, _| {
        if let Some(row) = state.selected_row {
            state.data.remove(row as usize);
            state.selected_row = None;
        }
    });
    let button_update = Button::new("Update").on_click(|state: &mut AppState, _| {
        if let Some(row) = state.selected_row {
            state.data[row as usize].text = "updated".to_string();
        }
    });

    // TODO - Find a syntax that looks more hierachical
    let list_keys = state.data.iter().map(|list_item| list_item.id.to_string());
    let list_rows = state.data.iter().enumerate().map(|(i, list_item)| {
        let row_props = RowProps {
            list_item: list_item.clone(),
            is_selected: state.selected_row == Some(i),
        };

        MyListRow::new(row_props).on::<RowEvent, _>(move |state: &mut AppState, event| {
            if event.0 {
                state.selected_row = Some(i);
            } else {
                state.selected_row = None;
            }
        })
    });
    let list_view = ElementList::from_keys_elems(list_keys, list_rows);

    Tuple!(
        Row!(button_create, button_insert, button_delete, button_update)
            .with_flex_container_params(ROW_FLEX_PARAMS),
        list_view,
    )
}

fn main() -> Result<(), PlatformError> {
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

    RootHandler::new(AwesomeEditableList::new(()))
        .with_initial_state(state)
        .with_tracing(true)
        .launch()
}
