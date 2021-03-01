use crate::element_tree::{CompCtx, Element, ReconcileCtx, VirtualDom};
use crate::elements::{Component, ComponentHolder};
use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widgets::flex;

use druid::widget::prelude::*;
use druid::{widget, AppLauncher, PlatformError, Point, Widget, WidgetPod, WindowDesc};
use std::fmt::Debug;
use tracing::{debug_span, info, instrument, trace};

type WidgetSeqOf<RootCpState, RootCpEvent, ReturnedTree> = <<ReturnedTree as Element<
    RootCpState,
    RootCpEvent,
>>::BuildOutput as VirtualDom<RootCpState, RootCpEvent>>::TargetWidgetSeq;

// FIXME - RootComponent must be Clone to be able to clone the props
// Not intuitive, find different abstraction?

pub struct RootWidget<
    RootCpState: Clone + Default + Debug + PartialEq + 'static,
    RootCpEvent,
    ReturnedTree: Element<RootCpState, RootCpEvent>,
    Comp: Component<RootCpState, RootCpEvent, Output = ReturnedTree>,
> {
    pub root_component: Comp,
    pub component_state: (RootCpState, ReturnedTree::AggregateChildrenState),
    pub vdom: Option<ReturnedTree::BuildOutput>,
    pub default_widget: WidgetPod<DruidAppData, widget::Flex<DruidAppData>>,
    pub widget: Option<
        WidgetPod<
            DruidAppData,
            flex::FlexWidget<WidgetSeqOf<RootCpState, RootCpEvent, ReturnedTree>>,
        >,
    >,
}

impl<
        RootCpState: Clone + Default + Debug + PartialEq + 'static,
        RootCpEvent,
        ReturnedTree: Element<RootCpState, RootCpEvent>,
        Comp: Component<RootCpState, RootCpEvent, Output = ReturnedTree>,
    > RootWidget<RootCpState, RootCpEvent, ReturnedTree, Comp>
{
    pub fn new(root_component: ComponentHolder<Comp>) -> Self {
        let default_widget = WidgetPod::new(widget::Flex::row());
        RootWidget {
            root_component: root_component.0,
            component_state: Default::default(),
            vdom: None,
            default_widget,
            widget: None,
        }
    }

    pub fn with_state(self, root_state: RootCpState) -> Self {
        RootWidget {
            component_state: (root_state, Default::default()),
            ..self
        }
    }

    #[instrument(level = "debug", skip(self, ctx))]
    pub fn init(&mut self, ctx: &mut EventCtx) {
        let (new_vdom, state) = debug_span!("build").in_scope(|| {
            // FIXME - clone
            let ctx = CompCtx {
                local_state: Box::new(self.component_state.0.clone()),
            };
            self.root_component
                .clone()
                .call(&ctx)
                .build(Default::default())
        });
        self.component_state.1 = state;

        info!("Initial aggregate app state: {:?}", self.component_state);

        let widget_seq = debug_span!("init_tree").in_scope(|| new_vdom.init_tree());
        // FIXME - Fix alignment to be consistent
        // (eg "Root(Button)" and "Root(Row(Button))" should be the same)
        let flex_widget = WidgetPod::new(flex::FlexWidget {
            direction: flex::Axis::Vertical,
            flex_params: flex::FlexContainerParams {
                cross_alignment: flex::CrossAxisAlignment::Center,
                main_alignment: flex::MainAxisAlignment::Start,
                fill_major_axis: false,
            },
            children_seq: widget_seq,
        });
        ctx.children_changed();
        self.widget = Some(flex_widget);
        self.vdom = Some(new_vdom);

        ctx.request_paint();
    }

    #[instrument(level = "debug", skip(self, ctx, data, env))]
    pub fn run(
        &mut self,
        ctx: &mut EventCtx,
        data: &mut DruidAppData,
        env: &Env,
    ) -> Option<RootCpEvent> {
        // The high-level workflow is:
        // - Make a copy of the app state.
        // - Run events that can change app state.
        //  -> If app state is unchanged, return early.
        // - Generate new vdom from new app state.
        // - Reconcile new vdom with previous vdom.
        let prev_component_state = self.component_state.clone();

        let event = debug_span!("process_event").in_scope(|| {
            let prev_vdom = self.vdom.as_mut().unwrap();
            let flex_widget = self.widget.as_mut().unwrap().widget_mut();
            let mut cx = GlobalEventCx::new(data);

            prev_vdom.process_event(
                &mut self.component_state.0,
                &mut self.component_state.1,
                &mut flex_widget.children_seq,
                &mut cx,
            )
        });

        if self.component_state == prev_component_state {
            trace!("State is unchanged. Skipping virtual DOM update.");
            return event;
        }

        info!("New aggregate app state: {:?}", self.component_state);

        let (new_vdom, state) = debug_span!("build").in_scope(|| {
            // FIXME - clone
            let ctx = CompCtx {
                local_state: Box::new(self.component_state.0.clone()),
            };
            self.root_component
                .clone()
                .call(&ctx)
                .build(std::mem::take(&mut self.component_state.1))
        });
        self.component_state.1 = state;

        let flex_widget = self.widget.as_mut().unwrap().widget_mut();
        let prev_vdom = self.vdom.as_mut().unwrap();
        let mut reconcile_ctx = ReconcileCtx {
            event_ctx: ctx,
            data,
            env,
        };

        debug_span!("reconcile").in_scope(|| {
            new_vdom.reconcile(
                &prev_vdom,
                &mut flex_widget.children_seq,
                &mut reconcile_ctx,
            );
        });
        debug_span!("update_value").in_scope(|| {
            prev_vdom.update_value(new_vdom);
        });

        ctx.request_update();
        ctx.request_paint();

        event
    }
}

impl<
        RootCpState: Clone + Default + Debug + PartialEq + 'static,
        RootCpEvent,
        ReturnedTree: Element<RootCpState, RootCpEvent>,
        Comp: Component<RootCpState, RootCpEvent, Output = ReturnedTree>,
    > Widget<DruidAppData> for RootWidget<RootCpState, RootCpEvent, ReturnedTree, Comp>
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        if let Some(widget) = &mut self.widget {
            widget.event(ctx, event, data, env);
        } else {
            self.default_widget.event(ctx, event, data, env);
        }

        if self.vdom.is_none() {
            self.init(ctx);
        } else {
            // We ignore the root event for now.
            // This might change in cases where the
            // user controls when RootWidget::run() is called.
            let _ = self.run(ctx, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    ) {
        if let Some(widget) = &mut self.widget {
            widget.lifecycle(ctx, event, data, env);
        } else {
            self.default_widget.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    ) {
        if let Some(widget) = &mut self.widget {
            widget.update(ctx, data, env);
        } else {
            self.default_widget.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size {
        let size;

        if let Some(widget) = &mut self.widget {
            size = widget.layout(ctx, bc, data, env);
            widget.set_origin(ctx, data, env, Point::ZERO);
        } else {
            size = self.default_widget.layout(ctx, bc, data, env);
            self.default_widget.set_origin(ctx, data, env, Point::ZERO);
        }

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        if let Some(widget) = &mut self.widget {
            widget.paint(ctx, data, env);
        } else {
            self.default_widget.paint(ctx, data, env);
        }
    }
}

pub struct RootHandler<
    RootCpState: Clone + Default + Debug + PartialEq + 'static,
    RootCpEvent,
    ReturnedTree: Element<RootCpState, RootCpEvent>,
    Comp: Component<RootCpState, RootCpEvent, Output = ReturnedTree>,
> {
    pub root_widget: RootWidget<RootCpState, RootCpEvent, ReturnedTree, Comp>,
    pub init_tracing: bool,
}

impl<
        RootCpState: 'static + Clone + Default + Debug + PartialEq,
        RootCpEvent: 'static,
        ReturnedTree: 'static + Element<RootCpState, RootCpEvent>,
        Comp: 'static + Component<RootCpState, RootCpEvent, Output = ReturnedTree>,
    > RootHandler<RootCpState, RootCpEvent, ReturnedTree, Comp>
{
    pub fn new(root_component: ComponentHolder<Comp>) -> Self {
        RootHandler {
            root_widget: RootWidget::new(root_component),
            init_tracing: false,
        }
    }

    pub fn with_state(self, root_state: RootCpState) -> Self {
        RootHandler {
            root_widget: self.root_widget.with_state(root_state),
            ..self
        }
    }

    pub fn with_tracing(self, init_tracing: bool) -> Self {
        RootHandler {
            init_tracing,
            ..self
        }
    }

    pub fn launch(self) -> Result<(), PlatformError> {
        if self.init_tracing {
            crate::glue::init_tracing();
        }

        let widget = self.root_widget;
        let main_window = WindowDesc::new(widget);
        let data = Default::default();

        AppLauncher::with_window(main_window).launch(data)
    }
}
