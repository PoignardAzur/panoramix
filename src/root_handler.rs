use crate::glue::{DruidAppData, GlobalEventCx};

use crate::element_tree::{ElementTree, VirtualDom};
use crate::widgets::flex;

// TODO
use crate::elements::component_caller::ComponentCaller;

use druid::widget::prelude::*;
use druid::{widget, Point, Widget, WidgetPod};

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

    pub fn run(&mut self, ctx: &mut EventCtx, cx: &mut GlobalEventCx) {
        let (vdom, component_state) = (self.root_component.component)(&self.component_state.0, ())
            .build(std::mem::take(&mut self.component_state.1));
        self.component_state.1 = component_state;

        let mut vdom_state;

        if let Some(prev_vdom) = self.vdom.as_mut() {
            let prev_vdom_state = self.vdom_state.take().unwrap();
            let flex_widget = self.widget.as_mut().unwrap().widget_mut();
            vdom_state = vdom.apply_diff(prev_vdom, prev_vdom_state, &mut flex_widget.children_seq);
            prev_vdom.update_value(vdom);

            if let Some(_event) = prev_vdom.process_event(
                &mut self.component_state.0,
                &mut self.component_state.1,
                &mut vdom_state,
                cx,
            ) {
                // callback(&event, &mut self.state);
            }
        } else {
            let (widget_seq, vdom_data) = vdom.init_tree();
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
            self.vdom = Some(vdom);
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
