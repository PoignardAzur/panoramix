#![allow(unused_imports)]

use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::flex::FlexParams;
use crate::glue::DebugState;
use crate::glue::DruidAppData;
use crate::widgets::SingleWidget;
use crate::RootWidget;
use druid::tests::harness::Harness as DruidHarness;
use druid::Command;
use druid::WidgetId;
use druid::WidgetState;
use std::any::Any;

use crate::element_tree::ReconcileCtx;
use druid::widget as druid_w;

use derivative::Derivative;
use tracing::instrument;

pub struct Harness<'a, 'b, RootElem: Element + Clone + Any> {
    pub druid_harness: &'a mut DruidHarness<'b, DruidAppData>,
    pub _markers: std::marker::PhantomData<RootElem>,
}

impl<RootElem: 'static + Element + Clone> Harness<'_, '_, RootElem> {
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

            let mut harness = Harness {
                druid_harness,
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
}
