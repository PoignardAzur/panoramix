use crate::flex::FlexParams;
use crate::glue::Action;
use crate::glue::DruidAppData;
use crate::glue::Id;
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use druid::kurbo::{Rect, Size};
use druid::widget::{Button, Click, ControllerHost};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    WidgetPod,
};

pub struct ButtonWidget {
    pub pod: WidgetPod<DruidAppData, ControllerHost<Button<DruidAppData>, Click<DruidAppData>>>,
    pub flex: FlexParams,
    pub id: Id,
}

impl ButtonWidget {
    pub fn new(text: String, flex: FlexParams, id: Id) -> Self {
        let button = Button::new(text)
            .on_click(move |_, data: &mut DruidAppData, _| data.queue_action(id, Action::Clicked));

        ButtonWidget {
            pod: WidgetPod::new(button),
            flex,
            id,
        }
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

impl WidgetSequence for ButtonWidget {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
