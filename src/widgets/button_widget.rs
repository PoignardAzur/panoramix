use crate::glue::Action;
use crate::glue::DruidAppData;
use crate::glue::Id;
use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;
use crate::widgets::flex::FlexParams;

use druid::kurbo::{Rect, Size};
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, UpdateCtx, WidgetPod,
};
use druid::widget::{Button, Click, ControllerHost};

pub struct ButtonWidget(
    pub WidgetPod<DruidAppData, ControllerHost<Button<DruidAppData>, Click<DruidAppData>>>,
    pub Id,
);

impl ButtonWidget {
    pub fn new(text: String, id: Id) -> Self {
        let button = Button::new(text)
            .on_click(move |_, data: &mut DruidAppData, _| data.queue_action(id, Action::Clicked));

        ButtonWidget(WidgetPod::new(button), id)
    }
}

impl FlexWidget for ButtonWidget {
    fn flex_params(&self) -> FlexParams {
        FlexParams {
            flex: 1.0,
            alignment: None,
        }
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        self.0.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.0.lifecycle(ctx, event, data, env);
    }   fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    ) {
        self.0.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size {
        self.0.layout(ctx, bc, data, env)
    }

    fn paint_rect(&self) -> Rect {
        self.0.paint_rect()
    }

    fn set_layout_rect(&mut self, ctx: &mut LayoutCtx, data: &DruidAppData, env: &Env, rect: Rect) {
        self.0.set_layout_rect(ctx, data, env, rect)
    }

    fn layout_rect(&self) -> Rect {
        self.0.layout_rect()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        self.0.paint(ctx, data, env);
    }
}

impl WidgetSequence for ButtonWidget {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}
