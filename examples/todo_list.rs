use panoramix::elements::{
    Button, ButtonPressed, Checkbox, ElementList, Label, TextBox, TextChanged, Toggled,
};
use panoramix::widgets::flex::{CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use panoramix::{
    component, make_column, make_row, CompCtx, Element, ElementExt, NoEvent, RootHandler,
};

use druid::PlatformError;

const ROW_FLEX_PARAMS: FlexContainerParams = FlexContainerParams {
    cross_alignment: CrossAxisAlignment::Center,
    main_alignment: MainAxisAlignment::Center,
    fill_major_axis: true,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TaskItem {
    text: String,
    is_completed: bool,
    id: i32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AppState {
    tasks: Vec<TaskItem>,
    high_priority: bool,
    task_name: String,
    next_id: i32,
}

type ItemEvent = Toggled;

#[component]
fn TodoItem(_ctx: &CompCtx, props: TaskItem) -> impl Element<(), ItemEvent> {
    let text = if props.is_completed {
        format!("{} (complete)", props.text)
    } else {
        props.text.clone()
    };

    make_row!(
        Checkbox::new("", props.is_completed).bubble_up::<ItemEvent>(),
        Label::new(text),
    )
    .with_flex_container_params(ROW_FLEX_PARAMS)
}

#[component]
fn AwesomeEditableList(ctx: &CompCtx, _props: ()) -> impl Element<AppState, NoEvent> {
    let state = ctx.use_local_state::<AppState>();

    let checkbox_priority = Checkbox::new("High priority", state.high_priority).on(
        |state: &mut AppState, event: Toggled| {
            state.high_priority = event.0;
        },
    );
    // TODO - Add "validate on enter" feature
    let textbox_task_name = TextBox::new(state.task_name.clone()).on::<TextChanged, _>(
        |state: &mut AppState, event: TextChanged| {
            state.task_name = event.0;
        },
    );

    let button_new_task =
        Button::new("New task").on::<ButtonPressed, _>(|state: &mut AppState, _| {
            if state.task_name == "" {
                return;
            }

            let new_task = TaskItem {
                text: state.task_name.clone(),
                is_completed: false,
                id: state.next_id,
            };

            // If it's high priority, insert it at the beginning
            if state.high_priority {
                state.tasks.insert(0, new_task);
            } else {
                state.tasks.push(new_task);
            }

            state.task_name = String::new();
            state.next_id += 1;
        });

    // TODO - Find a syntax that looks more hierachical
    let list_keys = state.tasks.iter().map(|task_item| task_item.id.to_string());
    let list_rows = state.tasks.iter().enumerate().map(|(i, task_item)| {
        TodoItem::new(task_item.clone()).on::<ItemEvent, _>(move |state: &mut AppState, event| {
            state.tasks[i].is_completed = event.0;
        })
    });
    let list_view = ElementList::from_keys_elems(list_keys, list_rows);

    let button_delete =
        Button::new("Delete completed tasks").on::<ButtonPressed, _>(|state: &mut AppState, _| {
            state.tasks = std::mem::take(&mut state.tasks)
                .into_iter()
                .filter(|task| !task.is_completed)
                .collect();
        });

    make_column!(
        make_row!(checkbox_priority, textbox_task_name, button_new_task)
            .with_flex_container_params(ROW_FLEX_PARAMS),
        list_view,
        button_delete,
    )
}

fn main() -> Result<(), PlatformError> {
    let tasks: Vec<_> = (0..4)
        .into_iter()
        .map(|i| TaskItem {
            text: format!("Task #{}", i),
            is_completed: false,
            id: i,
        })
        .collect();
    let next_id = tasks.len() as i32;

    let state = AppState {
        tasks,
        high_priority: false,
        task_name: String::new(),
        next_id,
    };

    RootHandler::new(AwesomeEditableList::new(()))
        .with_state(state)
        .with_tracing(true)
        .launch()
}
