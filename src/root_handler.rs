use crate::element_tree::{ElementTree, VirtualDom};
use crate::glue::{DruidAppData, GlobalEventCx};
use crate::widgets::flex;

// TODO
use crate::elements::component_caller::ComponentCaller;

use druid::widget::prelude::*;
use druid::{widget, Point, Widget, WidgetPod};
use tracing::instrument;

pub type WidgetSeqOf<RootCompState, ReturnedTree> =
   <<ReturnedTree as ElementTree<RootCompState>>::BuildOutput as VirtualDom<RootCompState>>::TargetWidgetSeq;

pub struct RootHandler<
    RootCompState,
    ReturnedTree: ElementTree<RootCompState>,
    Comp: Fn(&RootCompState, ()) -> ReturnedTree,
> {
    pub root_component: ComponentCaller<RootCompState, (), ReturnedTree, Comp, ()>,
    pub component_state: (RootCompState, ReturnedTree::AggregateComponentState),
    pub vdom: Option<ReturnedTree::BuildOutput>,
    pub vdom_state: Option<<ReturnedTree::BuildOutput as VirtualDom<RootCompState>>::DomState>,
    pub default_widget: WidgetPod<DruidAppData, widget::Flex<DruidAppData>>,
    pub widget:
        Option<WidgetPod<DruidAppData, flex::Flex<WidgetSeqOf<RootCompState, ReturnedTree>>>>,
}

impl<
        RootCompState,
        ReturnedTree: ElementTree<RootCompState>,
        Comp: Fn(&RootCompState, ()) -> ReturnedTree,
    > RootHandler<RootCompState, ReturnedTree, Comp>
{
    pub fn new(
        root_component: Comp,
        root_state: RootCompState,
    ) -> RootHandler<RootCompState, ReturnedTree, Comp> {
        let default_widget = WidgetPod::new(widget::Flex::row());
        RootHandler {
            root_component: ComponentCaller {
                component: root_component,
                props: (),
                _state: Default::default(),
                _tree: Default::default(),
                _expl_state: Default::default(),
            },
            component_state: (root_state, Default::default()),
            vdom: None,
            vdom_state: None,
            default_widget,
            widget: None,
        }
    }

    #[instrument(level = "debug", skip(self, ctx, cx))]
    pub fn run(&mut self, ctx: &mut EventCtx, cx: &mut GlobalEventCx) {
        use tracing::debug_span;

        let (new_vdom, state) = debug_span!("build").in_scope(|| {
            (self.root_component.component)(&self.component_state.0, ())
                .build(std::mem::take(&mut self.component_state.1))
        });
        self.component_state.1 = state;

        let mut vdom_state;

        if let Some(prev_vdom) = self.vdom.as_mut() {
            let prev_vdom_state = self.vdom_state.take().unwrap();
            let flex_widget = self.widget.as_mut().unwrap().widget_mut();

            vdom_state = debug_span!("apply_diff").in_scope(|| {
                new_vdom.apply_diff(prev_vdom, prev_vdom_state, &mut flex_widget.children_seq)
            });
            debug_span!("update_value").in_scope(|| {
                prev_vdom.update_value(new_vdom);
            });

            let _span_process_event = debug_span!("process_event");
            let _span_process_event = _span_process_event.enter();
            // TODO - use process_event's return?
            let _ = prev_vdom.process_event(
                &mut self.component_state.0,
                &mut self.component_state.1,
                &mut vdom_state,
                cx,
            );
            std::mem::drop(_span_process_event);
        } else {
            let (widget_seq, vdom_data) =
                debug_span!("init_tree").in_scope(|| new_vdom.init_tree());
            let flex_widget = WidgetPod::new(flex::Flex {
                direction: flex::Axis::Vertical,
                cross_alignment: flex::CrossAxisAlignment::Center,
                main_alignment: flex::MainAxisAlignment::Start,
                fill_major_axis: false,
                children_seq: widget_seq,
            });
            ctx.children_changed();
            self.widget = Some(flex_widget);
            vdom_state = vdom_data;
            self.vdom = Some(new_vdom);
        }

        self.vdom_state = Some(vdom_state);
    }
}

impl<
        RootCompState,
        ReturnedTree: ElementTree<RootCompState>,
        Comp: Fn(&RootCompState, ()) -> ReturnedTree,
    > Widget<DruidAppData> for RootHandler<RootCompState, ReturnedTree, Comp>
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        if let Some(widget) = &mut self.widget {
            widget.event(ctx, event, data, env);
        } else {
            self.default_widget.event(ctx, event, data, env);
        }

        let mut cx = GlobalEventCx::new(data);
        self.run(ctx, &mut cx);
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
            widget.set_layout_rect(ctx, data, env, (Point::ZERO, size).into());
        } else {
            size = self.default_widget.layout(ctx, bc, data, env);
            self.default_widget
                .set_layout_rect(ctx, data, env, (Point::ZERO, size).into());
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
