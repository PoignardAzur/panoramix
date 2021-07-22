use panoramix::elements::{Button, Checkbox, ComponentOutput, ElementList, Label, Toggled};
use panoramix::flex::{CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use panoramix::{component, CompCtx, Element, ElementExt, NoEvent, RootHandler, Row, Tuple};

use panoramix::PlatformError;

const ROW_FLEX_PARAMS: FlexContainerParams = FlexContainerParams {
    cross_alignment: CrossAxisAlignment::Center,
    main_alignment: MainAxisAlignment::Center,
    fill_major_axis: true,
};

#[derive(Debug, Default, Clone, PartialEq)]
struct ListItem {
    text: String,
    id: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct AppState {
    data: Vec<ListItem>,
    selected_row: Option<usize>,
    next_id: i32,
}

// We implement the initial state of the app
impl Default for AppState {
    fn default() -> Self {
        AppState {
            data: (0..8_i32)
                .map(|i| ListItem {
                    text: "hello".to_string(),
                    id: i,
                })
                .collect(),
            selected_row: None,
            next_id: 8,
        }
    }
}

type RowEvent = Toggled;
#[derive(Debug, Default, Clone, PartialEq)]
struct RowProps {
    list_item: ListItem,
    is_selected: bool,
}

#[component]
fn MyListRow(ctx: &CompCtx, props: RowProps) -> impl Element<Event = RowEvent> {
    let md = ctx.use_metadata::<RowEvent, u16>();
    let age = ctx.get_local_state(md);

    let row = Row!(
        Checkbox::new("", props.is_selected).map_event(md, |state: &mut u16, event| {
            *state += 1;
            Some(event)
        }),
        Label::new(format!("{} - age={}", &props.list_item.text, age)),
        Label::new(format!("id={}", props.list_item.id)),
    )
    .with_flex_container_params(ROW_FLEX_PARAMS);
    ComponentOutput::new(md, row)
}

#[component]
fn AwesomeEditableList(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, AppState>();
    let state = ctx.get_local_state(md);

    let button_create = Button::new("Create").on_click(md, |state: &mut AppState, _| {
        state.data.push(ListItem {
            text: "new item".to_string(),
            id: state.next_id,
        });
        state.next_id += 1;
    });
    let button_insert = Button::new("Insert").on_click(md, |state: &mut AppState, _| {
        state.data.insert(
            0,
            ListItem {
                text: "inserted item".to_string(),
                id: state.next_id,
            },
        );
        state.next_id += 1;
    });
    let button_delete = Button::new("Delete").on_click(md, |state: &mut AppState, _| {
        if let Some(row) = state.selected_row {
            state.data.remove(row as usize);
            state.selected_row = None;
        }
    });
    let button_update = Button::new("Update").on_click(md, |state: &mut AppState, _| {
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

        MyListRow::new(row_props).on::<RowEvent, _, _, _>(md, move |state: &mut AppState, event| {
            if event.new_value {
                state.selected_row = Some(i);
            } else {
                state.selected_row = None;
            }
        })
    });
    let list_view = ElementList::from_keys_elems(list_keys, list_rows);

    ComponentOutput::new(
        md,
        Tuple!(
            Row!(button_create, button_insert, button_delete, button_update)
                .with_flex_container_params(ROW_FLEX_PARAMS),
            list_view,
        ),
    )
}

fn main() -> Result<(), PlatformError> {
    RootHandler::new(AwesomeEditableList)
        .with_tracing(true)
        .launch()
}
