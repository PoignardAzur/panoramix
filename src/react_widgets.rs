use druid::kurbo::{Rect, Size};

use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, UpdateCtx, Widget, WidgetPod,
};
use druid::widget::Button;
use crate::flex2::FlexParams;
use crate::glue::DruidAppData;


pub trait FlexWidget {
    fn widget(&mut self) -> &mut dyn Widget<DruidAppData>;
    fn flex_params(&self) -> FlexParams;

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env);
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DruidAppData, env: &Env);
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &DruidAppData, data: &DruidAppData, env: &Env);

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &DruidAppData, env: &Env) -> Size;
    fn paint_rect(&self) -> Rect;
    fn set_layout_rect(&mut self, ctx: &mut LayoutCtx, data: &DruidAppData, env: &Env, rect: Rect);
    fn layout_rect(&self) -> Rect;
    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env);
}

pub trait WidgetSequence {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget>;
}


pub struct SingleWidget<W: Widget<DruidAppData>>(pub WidgetPod<DruidAppData, W>);

impl<W: Widget<DruidAppData>> FlexWidget for SingleWidget<W> {
    fn widget(&mut self) -> &mut dyn Widget<DruidAppData> {
        self.0.widget_mut()
    }

    fn flex_params(&self) -> FlexParams {
        FlexParams {
            flex: 1.0,
            alignment: None,
        }
    }


    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        self.0.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DruidAppData, env: &Env) {
        self.0.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &DruidAppData, data: &DruidAppData, env: &Env) {
        self.0.update(ctx, data, env);
    }


    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &DruidAppData, env: &Env) -> Size {
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

impl<W: Widget<DruidAppData>> WidgetSequence for SingleWidget<W> {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        vec![self]
    }
}

use crate::glue::Id;
use crate::glue::Action;
use druid::widget::ControllerHost;
use druid::widget::Click;
pub fn make_button(text: String, id: Id) -> SingleWidget<ControllerHost<Button<DruidAppData>, Click<DruidAppData>>> {
    let button = Button::new(text)
        .on_click(move |_, data: &mut DruidAppData, _| {
            data.queue_action(id, Action::Clicked)
        });

    SingleWidget(WidgetPod::new(button))
}



pub struct WidgetTuple<
    WS0: WidgetSequence,
    WS1: WidgetSequence,
    WS2: WidgetSequence,
    WS3: WidgetSequence,
>(
    pub WS0,
    pub WS1,
    pub WS2,
    pub WS3,
);

impl<
    WS0: WidgetSequence,
    WS1: WidgetSequence,
    WS2: WidgetSequence,
    WS3: WidgetSequence,
> WidgetSequence for WidgetTuple<WS0, WS1, WS2, WS3> {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        let mut all_widgets = Vec::new();
        all_widgets.append(&mut self.0.widgets());
        all_widgets.append(&mut self.1.widgets());
        all_widgets.append(&mut self.2.widgets());
        all_widgets.append(&mut self.3.widgets());
        all_widgets
    }
}


pub struct WidgetList<Child: WidgetSequence> {
    pub children: Vec<Child>,
}

impl<Child: WidgetSequence> WidgetSequence for WidgetList<Child> {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.children.iter_mut().flat_map(|child| child.widgets()).collect()
    }
}
