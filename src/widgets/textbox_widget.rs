use crate::flex::FlexParams;
use crate::glue::{Action, DruidAppData, WidgetId};
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use crate::glue::DebugState;
use druid::kurbo::{Point, Rect, Size};
use druid::widget::{IdentityWrapper, TextBox};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget, WidgetExt, WidgetPod,
};

use tracing::trace;

pub struct TextBoxWidget {
    pub text: String,
    pub pod: WidgetPod<String, IdentityWrapper<TextBox<String>>>,
    pub flex: FlexParams,
    id: WidgetId,
}

impl TextBoxWidget {
    pub fn new(text: String, flex: FlexParams, id: WidgetId) -> Self {
        let textbox = TextBox::new().with_id(id);

        TextBoxWidget {
            text,
            pod: WidgetPod::new(textbox),
            flex,
            id,
        }
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }

    // TODO - merge with SingleWidget::request_druid_update ?
    pub fn request_druid_update(&mut self, ctx: &mut EventCtx) {
        self.pod
            .with_event_context(ctx, |_widget, ctx: &mut EventCtx| {
                trace!("request_druid_update: {:?}", ctx.widget_id());
                ctx.request_update();
            });
    }
}

impl FlexWidget for TextBoxWidget {
    fn flex_params(&self) -> FlexParams {
        self.flex
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        if let Event::KeyUp(_) = event {
            trace!("TextBox {:?} content changed: {}", self.id(), self.text);
            data.queue_action(self.id(), Action::TextChanged(self.text.clone()));
        }
        self.pod.event(ctx, event, &mut self.text, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.lifecycle(ctx, event, &mut self.text, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DruidAppData,
        _data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.update(ctx, &mut self.text, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DruidAppData,
        env: &Env,
    ) -> Size {
        self.pod.layout(ctx, bc, &mut self.text, env)
    }

    fn paint_rect(&self) -> Rect {
        self.pod.paint_rect()
    }

    fn set_origin(&mut self, ctx: &mut LayoutCtx, _data: &DruidAppData, env: &Env, origin: Point) {
        self.pod.set_origin(ctx, &mut self.text, env, origin)
    }

    fn layout_rect(&self) -> Rect {
        self.pod.layout_rect()
    }

    fn baseline_offset(&self) -> f64 {
        self.pod.baseline_offset()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &DruidAppData, env: &Env) {
        self.pod.paint(ctx, &mut self.text, env);
    }

    fn debug_state(&self, _data: &DruidAppData) -> DebugState {
        self.pod.widget().debug_state(&self.text)
    }
}

impl WidgetSequence for TextBoxWidget {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        vec![self]
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
