use panoramix::elements::{Button, ComponentOutput, EmptyElement};
use panoramix::internals::WidgetId;
use panoramix::Metadata;
use panoramix::{component, CompCtx, Element, ElementExt, NoEvent};

#[derive(Debug, Default, Clone, PartialEq)]
struct MyEvent;

#[component]
fn UseMetadataTwice(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let _md = ctx.use_metadata::<NoEvent, i32>();
    let _md = ctx.use_metadata::<NoEvent, i32>();
    EmptyElement::new()
}

#[component]
fn NoComponentOutput(ctx: &CompCtx, _props: ()) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<NoEvent, i32>();
    let _local_state = ctx.get_local_state(md);
    EmptyElement::new()
}

#[component]
fn WrongState(ctx: &CompCtx, _props: ()) -> impl Element<Event = MyEvent> {
    let md = ctx.use_metadata::<MyEvent, ()>();
    let _local_state = ctx.get_local_state(md);

    let empty = EmptyElement::new();
    let wrong_md: Metadata<MyEvent, i32> = Default::default();
    ComponentOutput::new(wrong_md, empty)
}

#[component]
fn WrongEvent(ctx: &CompCtx, id: WidgetId) -> impl Element<Event = NoEvent> {
    let md = ctx.use_metadata::<MyEvent, u16>();
    let button = Button::new("Hello")
        .with_reserved_id(id)
        .map_event(md, |_, _event| Some(MyEvent));
    let wrong_md: Metadata<NoEvent, u16> = Default::default();
    ComponentOutput::new(wrong_md, button)
}

#[component]
fn CorrectMetadata(ctx: &CompCtx, id: WidgetId) -> impl Element<Event = MyEvent> {
    let md = ctx.use_metadata::<MyEvent, u16>();
    let button = Button::new("Hello")
        .with_reserved_id(id)
        .map_event(md, |_, _event| Some(MyEvent));
    ComponentOutput::new(md, button)
}

use panoramix::test_harness::Harness;
use test_env_log::test;

#[test]
#[should_panic]
fn use_metadata_twice() {
    let element = UseMetadataTwice::new(());
    element.build(Default::default());
}

#[test]
#[should_panic]
fn no_component_output() {
    let element = NoComponentOutput::new(());
    element.build(Default::default());
}

#[test]
#[should_panic]
fn wrong_state() {
    let element = WrongState::new(());
    element.build(Default::default());
}

#[test]
#[should_panic]
fn wrong_event() {
    let button_id = WidgetId::reserved(1);
    let element = WrongEvent::new(button_id);

    Harness::run_test_window(element, |harness| {
        harness.mouse_click_on(button_id);
    });
}

#[test]
fn correct_metadata() {
    let button_id = WidgetId::reserved(1);
    let element = CorrectMetadata::new(button_id);

    Harness::run_test_window(element, |harness| {
        harness.mouse_click_on(button_id);
    });
}
