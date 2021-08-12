use crate::elements::{Component, ElementBox};
use crate::flex;
use crate::glue::{DruidAppData, GlobalEventCx};
use crate::internals::{ProcessEventCtx, ReconcileCtx, VirtualDom};
use crate::widgets::FlexWidget;
use crate::{Element, NoEvent};

use crate::glue::DebugState;

use druid::widget::prelude::*;
use druid::{widget, AppLauncher, Point, Widget, WidgetPod, WindowDesc};
use std::fmt::Debug;
use tracing::{debug_span, info, instrument, trace};

pub use druid::PlatformError;

// TODO - trait Element: 'static

/// Implements [`druid::Widget`] from a component
///
/// You should probably use [`RootHandler`] directly instead.
pub struct RootWidget<RootElem: Element> {
    pub root_element: RootElem,
    pub root_state: RootElem::AggregateChildrenState,
    pub vdom: Option<RootElem::BuildOutput>,
    pub default_widget: WidgetPod<DruidAppData, widget::Flex<DruidAppData>>,
    pub widget: Option<
        WidgetPod<
            DruidAppData,
            widget::Scroll<
                DruidAppData,
                FlexWidget<<RootElem::BuildOutput as VirtualDom>::TargetWidgetSeq>,
            >,
        >,
    >,
}

impl<LocalEvent: Clone + Debug + PartialEq + 'static> RootWidget<ElementBox<LocalEvent>> {
    pub fn new<Comp: Component<Props = (), LocalEvent = LocalEvent>>(
        _root_component: Comp,
    ) -> Self {
        RootWidget {
            root_element: Comp::new(()),
            root_state: Default::default(),
            vdom: None,
            default_widget: WidgetPod::new(widget::Flex::row()),
            widget: None,
        }
    }
}

impl<RootElem: Element> RootWidget<RootElem> {
    pub fn from_element(elem: RootElem) -> Self {
        RootWidget {
            root_element: elem,
            root_state: Default::default(),
            vdom: None,
            default_widget: WidgetPod::new(widget::Flex::row()),
            widget: None,
        }
    }
}

impl<RootElem: Element> RootWidget<RootElem> {
    #[instrument(level = "debug", skip(self, ctx))]
    pub fn init(&mut self, ctx: &mut EventCtx) {
        let (new_vdom, state) =
            debug_span!("build").in_scope(|| self.root_element.clone().build(Default::default()));
        self.root_state = state;

        info!("Initial aggregate app state: {:?}", self.root_state);

        let widget_seq = debug_span!("init_tree").in_scope(|| new_vdom.init_tree());
        // FIXME - Fix alignment to be consistent
        // (eg "Root(Button)" and "Root(Row(Button))" should be the same)
        let flex_widget = WidgetPod::new(
            widget::Scroll::new(FlexWidget {
                direction: flex::Axis::Vertical,
                flex_params: flex::FlexContainerParams {
                    cross_alignment: flex::CrossAxisAlignment::Center,
                    main_alignment: flex::MainAxisAlignment::Start,
                    fill_major_axis: false,
                },
                children_seq: widget_seq,
            })
            .vertical(),
        );
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
        force_update: bool,
    ) {
        // The high-level workflow is:
        // - Make a copy of the app state.
        // - Run events that can change app state.
        //  -> If app state is unchanged, return early.
        // - Generate new vdom from new app state.
        // - Reconcile new vdom with previous vdom.

        let prev_root_state = self.root_state.clone();

        debug_span!("process_event").in_scope(|| {
            let prev_vdom = self.vdom.as_mut().unwrap();
            let flex_widget = self.widget.as_mut().unwrap().widget_mut();
            let mut cx = GlobalEventCx::new(data);

            // We ignore the root event for now.
            // This might change in cases where we want the user to control
            // when RootWidget::run() is called.
            // Types are default types, because ProcessEventCtx will
            // normally be unused in root element.
            let mut ctx = ProcessEventCtx {
                event_queue: &mut Vec::<NoEvent>::new(),
                state: &mut (),
            };
            prev_vdom.process_event(
                &mut ctx,
                &mut self.root_state,
                &mut flex_widget.child_mut().children_seq,
                &mut cx,
            );
        });

        if !force_update && self.root_state == prev_root_state {
            trace!("State is unchanged. Skipping virtual DOM update.");
            return;
        }

        info!("New aggregate app state: {:?}", self.root_state);

        let (new_vdom, state) = debug_span!("build").in_scope(|| {
            self.root_element
                .clone()
                .build(std::mem::take(&mut self.root_state))
        });
        self.root_state = state;

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
                &mut flex_widget.child_mut().children_seq,
                &mut reconcile_ctx,
            );
        });
        *prev_vdom = new_vdom;

        ctx.request_update();
        ctx.request_paint();
    }
}

impl<RootElem: Element> Widget<DruidAppData> for RootWidget<RootElem> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        let mut force_update = false;

        if let Event::Command(command) = event {
            let selector = druid::Selector::new("update_root_element");
            if let Some(new_root) = command.get::<RootElem>(selector) {
                self.root_element = new_root.clone();
                force_update = true;
            }
        };
        if let Some(widget) = &mut self.widget {
            widget.event(ctx, event, data, env);
        } else {
            self.default_widget.event(ctx, event, data, env);
        }

        if self.vdom.is_none() {
            self.init(ctx);
        } else {
            self.run(ctx, data, env, force_update);
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

    fn debug_state(&self, data: &DruidAppData) -> DebugState {
        let child: &dyn Widget<_> = if let Some(widget) = &self.widget {
            widget.widget()
        } else {
            self.default_widget.widget()
        };

        DebugState {
            display_name: self.short_type_name().to_string(),
            children: vec![child.debug_state(data)],
            ..Default::default()
        }
    }
}

/// Creates a GUI application from a component
pub struct RootHandler<RootElem: Element> {
    pub root_widget: RootWidget<RootElem>,
    pub init_tracing: bool,
}

impl<LocalEvent: Clone + Debug + PartialEq + 'static> RootHandler<ElementBox<LocalEvent>> {
    /// Creates the data to start the application.
    ///
    /// The `root_component` parameter should be roughly `YourRootComponent::new(some_props)`.
    ///
    /// Call [`launch`](RootHandler::launch) to actually start the application.
    pub fn new(root_component: impl Component<Props = (), LocalEvent = LocalEvent>) -> Self {
        RootHandler {
            root_widget: RootWidget::new(root_component),
            init_tracing: false,
        }
    }
}

impl<RootElem: Element> RootHandler<RootElem> {
    pub fn with_tracing(self, init_tracing: bool) -> Self {
        RootHandler {
            init_tracing,
            ..self
        }
    }

    /// Start the application.
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
