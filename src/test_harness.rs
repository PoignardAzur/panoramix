#![allow(unused_imports)]

use crate::internals::{ReconcileCtx, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::{DebugState, DruidAppData};
use crate::widgets::SingleWidget;
use crate::RootWidget;
use crate::{Element, NoEvent};

use derivative::Derivative;
use druid::keyboard_types::{Code, Key, KeyState};
use druid::tests::harness::Harness as DruidHarness;
use druid::{
    Command, Event, KeyEvent, Modifiers, MouseButton, MouseButtons, MouseEvent, Point, RawMods,
    Vec2, WidgetId, WidgetState,
};
use std::any::Any;
use tracing::instrument;

use druid::widget as druid_w;

pub struct Harness<'a, 'b, RootElem: Element + Any> {
    pub druid_harness: &'a mut DruidHarness<'b, DruidAppData>,
    pub mouse_state: MouseEvent,
    pub _markers: std::marker::PhantomData<RootElem>,
}

impl<RootElem: 'static + Element> Harness<'_, '_, RootElem> {
    pub fn run_test_window(
        element: RootElem,
        callback: impl FnMut(&mut Harness<'_, '_, RootElem>),
    ) {
        let root_widget = RootWidget::from_element(element);
        let data: DruidAppData = Default::default();
        let mut callback = callback;

        DruidHarness::create_simple(data, root_widget, move |druid_harness| {
            druid_harness.send_initial_events();
            druid_harness.just_layout();

            let mouse_state = MouseEvent {
                pos: Point::ZERO,
                window_pos: Point::ZERO,
                buttons: MouseButtons::default(),
                mods: Modifiers::default(),
                count: 0,
                focus: false,
                button: MouseButton::None,
                wheel_delta: Vec2::ZERO,
            };

            let mut harness = Harness {
                druid_harness,
                mouse_state,
                _markers: Default::default(),
            };

            callback(&mut harness);
        });
    }

    pub fn get_state(&mut self, widget_id: WidgetId) -> WidgetState {
        self.druid_harness.get_state(widget_id)
    }

    pub fn try_get_state(&mut self, widget_id: WidgetId) -> Option<WidgetState> {
        self.druid_harness.try_get_state(widget_id)
    }

    pub fn get_root_debug_state(&self) -> DebugState {
        self.druid_harness.get_root_debug_state()
    }

    pub fn get_debug_state(&mut self, widget_id: WidgetId) -> DebugState {
        self.druid_harness.get_debug_state(widget_id)
    }

    pub fn try_get_debug_state(&mut self, widget_id: WidgetId) -> Option<DebugState> {
        self.druid_harness.try_get_debug_state(widget_id)
    }

    /// Send a command to a target.
    pub fn submit_command(&mut self, cmd: impl Into<Command>) {
        self.druid_harness.submit_command(cmd)
    }

    pub fn update_root_element(&mut self, new_root: RootElem) {
        use druid::{Command, Selector, Target};

        let selector = Selector::new("update_root_element");
        let command = Command::new(selector, new_root, Target::Global);

        self.druid_harness.submit_command(command);
    }

    pub fn mouse_move(&mut self, pos: impl Into<Point>) {
        let pos = pos.into();
        self.mouse_state.pos = pos;
        self.mouse_state.window_pos = pos;
        self.mouse_state.button = MouseButton::None;

        self.druid_harness
            .event(Event::MouseMove(self.mouse_state.clone()));
    }

    pub fn mouse_button_press(&mut self, button: MouseButton) {
        self.mouse_state.buttons.insert(button);
        self.mouse_state.button = button;

        self.druid_harness
            .event(Event::MouseDown(self.mouse_state.clone()));
    }

    pub fn mouse_button_release(&mut self, button: MouseButton) {
        self.mouse_state.buttons.remove(button);
        self.mouse_state.button = button;

        self.druid_harness
            .event(Event::MouseUp(self.mouse_state.clone()));
    }

    pub fn mouse_click_on(&mut self, id: WidgetId) {
        let widget_rect = self.druid_harness.get_state(id).layout_rect();
        let widget_center = widget_rect.origin();

        self.mouse_move(widget_center);
        self.mouse_button_press(MouseButton::Left);
        self.mouse_button_release(MouseButton::Left);
    }

    pub fn mouse_move_to(&mut self, id: WidgetId) {
        let widget_rect = self.druid_harness.get_state(id).layout_rect();
        let widget_center = widget_rect.origin();

        self.mouse_move(widget_center);
    }

    pub fn keyboard_key(&mut self, key: &str) {
        let event = KeyEvent::for_test(RawMods::None, key);

        self.druid_harness.event(Event::KeyDown(event.clone()));
        self.druid_harness.event(Event::KeyUp(event.clone()));
    }
}
