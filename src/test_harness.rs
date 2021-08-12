//! Harness used to mock a druid-and-panoramix environment on a headless target.

use crate::glue::{DebugState, DruidAppData};
use crate::Element;
use crate::RootWidget;

use druid::tests::harness::Harness as DruidHarness;
use druid::{
    Command, Event, KeyEvent, Modifiers, MouseButton, MouseButtons, MouseEvent, Point, RawMods,
    Vec2, WidgetId, WidgetState,
};
use std::any::Any;

// TODO
// use tracing::instrument;

/// Harness used to create a mock test environment.
///
/// A typical panoramix test will look like:
///
/// ```no_run
/// # use panoramix::elements::Button;
/// # use panoramix::test_harness::Harness;
/// # type MyElement = Button;
/// fn my_unit_test() {
///     let element = MyElement::new("Hello");
///
///     Harness::run_test_window(element, |harness| {
///         let window_state = harness.get_root_debug_state();
///         insta::assert_debug_snapshot!(window_state);
///
///         // Use Harness methods to interact with test window,
///         // then check window state
///
///         // ...
///     });
/// }
/// ```
pub struct Harness<'a, 'b, RootElem: Element + Any> {
    pub druid_harness: &'a mut DruidHarness<'b, DruidAppData>,
    pub mouse_state: MouseEvent,
    pub _markers: std::marker::PhantomData<RootElem>,
}

impl<RootElem: 'static + Element> Harness<'_, '_, RootElem> {
    /// Create the harness, and pass it to a callback function.
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

    /// Retrieve a copy of this widget's `WidgetState`, or die trying.
    pub fn get_state(&mut self, widget_id: WidgetId) -> WidgetState {
        self.druid_harness.get_state(widget_id)
    }

    /// Attempt to retrieve a copy of this widget's `WidgetState`.
    pub fn try_get_state(&mut self, widget_id: WidgetId) -> Option<WidgetState> {
        self.druid_harness.try_get_state(widget_id)
    }

    /// Retrieve a copy of the root widget's `DebugState` (and by recursion, all others)
    pub fn get_root_debug_state(&self) -> DebugState {
        self.druid_harness.get_root_debug_state()
    }

    /// Retrieve a copy of this widget's `DebugState`, or die trying.
    pub fn get_debug_state(&mut self, widget_id: WidgetId) -> DebugState {
        self.druid_harness.get_debug_state(widget_id)
    }

    /// Attempt to retrieve a copy of this widget's `DebugState`.
    pub fn try_get_debug_state(&mut self, widget_id: WidgetId) -> Option<DebugState> {
        self.druid_harness.try_get_debug_state(widget_id)
    }

    /// Send a druid command through the widget tree.
    pub fn submit_command(&mut self, cmd: impl Into<Command>) {
        self.druid_harness.submit_command(cmd)
    }

    /// Update the entire harness with a new element, which replaces the one passed to [`Harness::run_test_window`].
    ///
    /// This is especially useful to test implementations of [`VirtualDom::reconcile`](crate::internals::VirtualDom::reconcile)
    pub fn update_root_element(&mut self, new_root: RootElem) {
        use druid::{Selector, Target};

        let selector = Selector::new("update_root_element");
        let command = Command::new(selector, new_root, Target::Global);

        self.druid_harness.submit_command(command);
    }

    /// Move an internal mouse state, and send a MouseMove event to the window.
    pub fn mouse_move(&mut self, pos: impl Into<Point>) {
        let pos = pos.into();
        self.mouse_state.pos = pos;
        self.mouse_state.window_pos = pos;
        self.mouse_state.button = MouseButton::None;

        self.druid_harness
            .event(Event::MouseMove(self.mouse_state.clone()));
    }

    /// Send a MouseDown event to the window.
    pub fn mouse_button_press(&mut self, button: MouseButton) {
        self.mouse_state.buttons.insert(button);
        self.mouse_state.button = button;

        self.druid_harness
            .event(Event::MouseDown(self.mouse_state.clone()));
    }

    /// Send a MouseUp event to the window.
    pub fn mouse_button_release(&mut self, button: MouseButton) {
        self.mouse_state.buttons.remove(button);
        self.mouse_state.button = button;

        self.druid_harness
            .event(Event::MouseUp(self.mouse_state.clone()));
    }

    /// Send events that lead to a given widget being clicked.
    ///
    /// Combines [`mouse_move`](Self::mouse_move), [`mouse_button_press`](Self::mouse_button_press), and [`mouse_button_release`](Self::mouse_button_release).
    pub fn mouse_click_on(&mut self, id: WidgetId) {
        let widget_rect = self.druid_harness.get_state(id).layout_rect();
        let widget_center = widget_rect.center();

        self.mouse_move(widget_center);
        self.mouse_button_press(MouseButton::Left);
        self.mouse_button_release(MouseButton::Left);
    }

    /// Use [`mouse_move`](Self::mouse_move) to set the internal mouse pos to the center of the given widget.
    pub fn mouse_move_to(&mut self, id: WidgetId) {
        let widget_rect = self.druid_harness.get_state(id).layout_rect();
        let widget_center = widget_rect.center();

        self.mouse_move(widget_center);
    }

    /// Send a KeyDown and a KeyUp event to the window.
    pub fn keyboard_key(&mut self, key: &str) {
        let event = KeyEvent::for_test(RawMods::None, key);

        self.druid_harness.event(Event::KeyDown(event.clone()));
        self.druid_harness.event(Event::KeyUp(event.clone()));
    }
}
