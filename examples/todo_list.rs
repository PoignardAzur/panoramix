use panoramix::elements::{
    Button, Checkbox, ComponentOutput, ElementList, Label, TextBox, TextChanged, Toggled,
};
use panoramix::flex::{CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use panoramix::{component, Column, CompCtx, Element, ElementExt, NoEvent, RootHandler, Row};

use panoramix::PlatformError;

const ROW_FLEX_PARAMS: FlexContainerParams = FlexContainerParams {
    cross_alignment: CrossAxisAlignment::Center,
    main_alignment: MainAxisAlignment::Center,
    fill_major_axis: true,
};

#[derive(Debug, Default, Clone, PartialEq)]
struct TaskItem {
    text: String,
    is_completed: bool,
    id: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct AppState {
    tasks: Vec<TaskItem>,
    high_priority: bool,
    task_name: String,
    next_id: i32,
}

// We implement the initial state of the app
impl Default for AppState {
    fn default() -> Self {
        let tasks: Vec<_> = (0..4)
            .into_iter()
            .map(|i| TaskItem {
                text: format!("Task #{}", i),
                is_completed: false,
                id: i,
            })
            .collect();
        let next_id = tasks.len() as i32;

        AppState {
            tasks,
            high_priority: false,
            task_name: String::new(),
            next_id,
        }
    }
}

type ItemEvent = Toggled;

#[component]
fn TaskRow(ctx: &CompCtx, props: TaskItem) -> impl Element<Event = ItemEvent> {
    let md = ctx.use_metadata::<ItemEvent, ()>();
    let text = if props.is_completed {
        format!("{} (complete)", props.text)
    } else {
        props.text.clone()
    };

    let row = Row!(
        Checkbox::new("", props.is_completed).bubble_up::<ItemEvent, _, _>(md),
        Label::new(text),
    )
    .with_flex_container_params(ROW_FLEX_PARAMS);
    ComponentOutput::new(md, row)
}

#[component]
fn AwesomeEditableList(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, AppState>();
    let state = ctx.get_local_state(md);

    let checkbox_priority = Checkbox::new("High priority", state.high_priority).on(
        md,
        |state: &mut AppState, event: Toggled| {
            state.high_priority = event.new_value;
        },
    );
    // TODO - Add "validate on enter" feature
    let textbox_task_name = TextBox::new(state.task_name.clone()).on_text_changed(
        md,
        |state: &mut AppState, event: TextChanged| {
            state.task_name = event.new_content;
        },
    );

    let button_new_task = Button::new("New task").on_click(md, |state: &mut AppState, _| {
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
        TaskRow::new(task_item.clone()).on::<ItemEvent, _, _, _>(
            md,
            move |state: &mut AppState, event| {
                state.tasks[i].is_completed = event.new_value;
            },
        )
    });
    let list_view = ElementList::from_keys_elems(list_keys, list_rows);

    let button_delete =
        Button::new("Delete completed tasks").on_click(md, |state: &mut AppState, _| {
            state.tasks = std::mem::take(&mut state.tasks)
                .into_iter()
                .filter(|task| !task.is_completed)
                .collect();
        });

    ComponentOutput::new(
        md,
        Column!(
            Row!(checkbox_priority, textbox_task_name, button_new_task)
                .with_flex_container_params(ROW_FLEX_PARAMS),
            list_view,
            button_delete,
        ),
    )
}

fn main() -> Result<(), PlatformError> {
    RootHandler::new(AwesomeEditableList)
        .with_tracing(true)
        .launch()
}
