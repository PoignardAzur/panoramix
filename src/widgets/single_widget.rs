use crate::element_tree::ReconcileCtx;
use crate::glue::DruidAppData;
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;
use crate::widgets::flex::FlexParams;
use druid::kurbo::{Rect, Size};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget, WidgetPod,
};
use tracing::trace;

pub struct SingleWidget<W: Widget<DruidAppData>> {
    pub pod: WidgetPod<DruidAppData, W>,
    pub flex: FlexParams,
}

impl<W: Widget<DruidAppData>> SingleWidget<W> {
    pub fn new(widget: W, flex: FlexParams) -> Self {
        SingleWidget {
            pod: WidgetPod::new(widget),
            flex,
        }
    }

    pub fn request_druid_update(&mut self, ctx: &mut ReconcileCtx) {
        self.pod
            .with_event_context(ctx.event_ctx, |_widget: &mut W, ctx: &mut EventCtx| {
                trace!("request_druid_update: {:?}", ctx.widget_id());
                ctx.request_update();
            });
    }
}

impl<W: Widget<DruidAppData>> FlexWidget for SingleWidget<W> {
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
}

impl<W: Widget<DruidAppData>> WidgetSequence for SingleWidget<W> {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
