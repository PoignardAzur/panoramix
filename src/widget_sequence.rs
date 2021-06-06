use druid::kurbo::{Rect, Size};

use crate::flex::FlexParams;
use crate::glue::DruidAppData;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
};

pub trait WidgetSequence {
    // TODO - This is horribly inefficient. We'd like to have
    //     -> impl Iterator<&mut dyn FlexWidget>
    // instead, but this would require both GATs and trait-method existential types to be stable
    fn widgets(&self) -> Vec<&dyn FlexWidget>;
    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget>;
}

// Essentially a boilerplate trait for SingleWidget
pub trait FlexWidget {
    fn flex_params(&self) -> FlexParams;

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env);
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    );
    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    );

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size;
    fn paint_rect(&self) -> Rect;
    fn set_layout_rect(&mut self, ctx: &mut LayoutCtx, data: &DruidAppData, env: &Env, rect: Rect);
    fn layout_rect(&self) -> Rect;
    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env);
}
