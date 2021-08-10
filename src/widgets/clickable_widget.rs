use crate::flex::FlexParams;
use crate::glue::{Action, DruidAppData, WidgetId};
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use crate::glue::DebugState;
use druid::kurbo::{Point, Rect, Size};
use druid::widget::{Click, ControllerHost, IdentityWrapper};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget, WidgetPod,
};

// ---

// FIXME
struct UnwrapSingleWidget<Children: WidgetSequence> {
    children: Children,
}

impl<Children: WidgetSequence> UnwrapSingleWidget<Children> {
    fn new(children: Children) -> Self {
        UnwrapSingleWidget { children }
    }

    #[allow(dead_code)]
    fn child(&self) -> &dyn FlexWidget {
        let mut children = self.children.widgets();
        assert_eq!(children.len(), 1);
        children.pop().unwrap()
    }

    #[allow(dead_code)]
    fn child_mut(&mut self) -> &mut dyn FlexWidget {
        let mut children = self.children.widgets_mut();
        assert_eq!(children.len(), 1);
        children.pop().unwrap()
    }
}

impl<Children: WidgetSequence> Widget<DruidAppData> for UnwrapSingleWidget<Children> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        self.child_mut().event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.child_mut().lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.child_mut().update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size {
        let size = self.child_mut().layout(ctx, bc, data, env);
        self.child_mut().set_layout_rect(
            ctx,
            data,
            env,
            Rect::from_origin_size(Point::ORIGIN, size),
        );
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        self.child_mut().paint(ctx, data, env);
    }

    fn debug_state(&self, _data: &DruidAppData) -> DebugState {
        todo!();
        //self.child_mut().widget().debug_state(&self.value)
    }
}

// ---

pub struct ClickableWidget<Children: WidgetSequence> {
    pod: WidgetPod<
        DruidAppData,
        IdentityWrapper<ControllerHost<UnwrapSingleWidget<Children>, Click<DruidAppData>>>,
    >,
    pub flex: FlexParams,
    id: WidgetId,
}

impl<Children: WidgetSequence> ClickableWidget<Children> {
    pub fn new(children: Children, id: WidgetId) -> Self {
        let clickable_widget = IdentityWrapper::wrap(
            ControllerHost::new(
                UnwrapSingleWidget::new(children),
                Click::new(move |_, data: &mut DruidAppData, _| {
                    data.queue_action(id, Action::Clicked)
                }),
            ),
            id,
        );

        ClickableWidget {
            pod: WidgetPod::new(clickable_widget),
            flex: Default::default(),
            id,
        }
    }

    pub fn children(&self) -> &Children {
        use druid::widget::WidgetWrapper as _;
        &self.pod.widget().wrapped().wrapped().children
    }

    pub fn children_mut(&mut self) -> &mut Children {
        use druid::widget::WidgetWrapper as _;
        &mut self.pod.widget_mut().wrapped_mut().wrapped_mut().children
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }
}

impl<Children: WidgetSequence> FlexWidget for ClickableWidget<Children> {
    fn flex_params(&self) -> FlexParams {
        self.flex
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        self.pod.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size {
        self.pod.layout(ctx, bc, data, env)
    }

    fn paint_rect(&self) -> Rect {
        self.pod.paint_rect()
    }

    fn set_layout_rect(&mut self, ctx: &mut LayoutCtx, data: &DruidAppData, env: &Env, rect: Rect) {
        self.pod.set_layout_rect(ctx, data, env, rect)
    }

    fn layout_rect(&self) -> Rect {
        self.pod.layout_rect()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        self.pod.paint(ctx, data, env);
    }

    fn debug_state(&self, data: &DruidAppData) -> DebugState {
        self.pod.widget().debug_state(data)
    }
}

impl<Children: WidgetSequence> WidgetSequence for ClickableWidget<Children> {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        vec![self]
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
