use crate::flex::FlexParams;
use crate::glue::{Action, DruidAppData, WidgetId};
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use crate::glue::DebugState;
use druid::kurbo::{Point, Rect, Size};
use druid::widget::{Button, Click, ControllerHost, IdentityWrapper};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget, WidgetExt, WidgetPod,
};

pub struct ButtonWidget {
    pub pod: WidgetPod<
        DruidAppData,
        IdentityWrapper<ControllerHost<Button<DruidAppData>, Click<DruidAppData>>>,
    >,
    pub flex: FlexParams,
    id: WidgetId,
}

impl ButtonWidget {
    pub fn new(text: String, flex: FlexParams, id: WidgetId) -> Self {
        let button = Button::new(text)
            .on_click(move |_, data: &mut DruidAppData, _| data.queue_action(id, Action::Clicked))
            .with_id(id);

        ButtonWidget {
            pod: WidgetPod::new(button),
            flex,
            id,
        }
    }

    pub fn id(&self) -> WidgetId {
        self.id
    }
}

impl FlexWidget for ButtonWidget {
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

    fn set_origin(&mut self, ctx: &mut LayoutCtx, data: &DruidAppData, env: &Env, origin: Point) {
        self.pod.set_origin(ctx, data, env, origin)
    }

    fn layout_rect(&self) -> Rect {
        self.pod.layout_rect()
    }

    fn baseline_offset(&self) -> f64 {
        self.pod.baseline_offset()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        self.pod.paint(ctx, data, env);
    }

    fn debug_state(&self, data: &DruidAppData) -> DebugState {
        self.pod.widget().debug_state(data)
    }
}

impl WidgetSequence for ButtonWidget {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        vec![self]
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
