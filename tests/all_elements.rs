use panoramix::elements::{
    Button, Checkbox, ComponentOutput, ElementBox, ElementList, Label, TextBox, TextChanged,
    Toggled,
};
use panoramix::internals::WidgetId;
use panoramix::{component, Column, CompCtx, Element, ElementExt, NoEvent, Row};

use either::{Left, Right};

#[derive(Debug, Default, Clone, PartialEq)]
struct TaskItem {
    text: String,
    is_completed: bool,
    id: u16,
}

#[derive(Debug, Clone, PartialEq)]
struct AppState {
    tasks: Vec<TaskItem>,
    high_priority: bool,
    task_name: String,
    next_id: u16,
}

// We implement the initial state of the app
impl Default for AppState {
    fn default() -> Self {
        let tasks: Vec<_> = (0..4)
            .into_iter()
            .map(|i| TaskItem {
                text: format!("Task #{}", i),
                is_completed: i >= 2,
                id: i,
            })
            .collect();
        let next_id = tasks.len() as u16;

        AppState {
            tasks,
            high_priority: false,
            task_name: String::from("Next task"),
            next_id,
        }
    }
}

type ItemEvent = Toggled;

#[component]
fn TaskRow(ctx: &CompCtx, props: TaskItem) -> impl Element<Event = ItemEvent> {
    let md = ctx.use_metadata::<ItemEvent, ()>();
    let text = props.text.clone();

    let checkbox = ElementBox::new(
        Checkbox::new("", props.is_completed).with_reserved_id(WidgetId::reserved(props.id + 10)),
    )
    .bubble_up::<ItemEvent, _, _>(md);
    let row = if props.is_completed {
        Left(Row!(checkbox, Label::new(text)))
    } else {
        Right(Row!(checkbox, Label::new(text), Label::new("(complete)")))
    };

    ComponentOutput::new(md, row)
}

#[component]
fn AwesomeEditableList(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, AppState>();
    let state = ctx.get_local_state(md);

    // TODO - Add "validate on enter" feature
    let textbox_task_name = TextBox::new(state.task_name.clone()).on_text_changed(
        md,
        |state: &mut AppState, event: TextChanged| {
            state.task_name = event.new_content;
        },
    );

    let button_new_task = Button::new("New task")
        .with_reserved_id(WidgetId::reserved(1))
        .on_click(md, |state: &mut AppState, _| {
            if state.task_name == "" {
                return;
            }

            let new_task = TaskItem {
                text: state.task_name.clone(),
                is_completed: false,
                id: state.next_id,
            };

            state.tasks.push(new_task);
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

    let button_delete = Button::new("Delete completed tasks")
        .with_reserved_id(WidgetId::reserved(2))
        .on_click(md, |state: &mut AppState, _| {
            state.tasks = std::mem::take(&mut state.tasks)
                .into_iter()
                .filter(|task| !task.is_completed)
                .collect();
        });

    ComponentOutput::new(
        md,
        Column!(
            Row!(textbox_task_name, button_new_task),
            list_view,
            button_delete,
        ),
    )
}

use insta::assert_debug_snapshot;
use panoramix::test_harness::Harness;
use test_env_log::test;

#[test]
fn test_all_widgets() {
    let list = AwesomeEditableList::new(());

    let button_new_task_id = WidgetId::reserved(1);
    let button_delete_id = WidgetId::reserved(2);
    //let checkbox_0_id = WidgetId::reserved(10);
    //let checkbox_2_id = WidgetId::reserved(12);

    Harness::run_test_window(list, |harness| {
        let root_state = harness.get_root_debug_state();
        assert_debug_snapshot!(root_state);

        harness.mouse_click_on(button_new_task_id);

        let root_state_2 = harness.get_root_debug_state();
        assert_debug_snapshot!(root_state_2);

        // FIXME - Add "edit textbox" event
        // FIXME - Add "click checkbox" event

        //harness.mouse_click_on(checkbox_0_id);
        //harness.mouse_click_on(checkbox_2_id);

        //let root_state_3 = harness.get_root_debug_state();
        //assert_debug_snapshot!(root_state_3);

        harness.mouse_click_on(button_delete_id);

        let root_state_4 = harness.get_root_debug_state();
        assert_debug_snapshot!(root_state_4);
    });
}
