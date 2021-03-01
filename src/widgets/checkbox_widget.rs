use crate::element_tree::ReconcileCtx;
use crate::glue::Action;
use crate::glue::DruidAppData;
use crate::glue::Id;
use crate::widgets::SingleWidget;

use druid::widget::Checkbox;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point,
    Size, UpdateCtx, Widget, WidgetPod,
};

use tracing::trace;

// Note: We implement Widget instead of our FlexWidget, and wrap CheckboxWidget in a SingleWidget
// because otherwise  mouse events are filtered correctly (eg we get mouse events even if the
// cursor isn't over our checkbox)

pub struct CheckboxWidget {
    pub value: bool,
    pub pod: WidgetPod<bool, Checkbox>,
    pub id: Id,
}

impl CheckboxWidget {
    pub fn new(text: String, value: bool, id: Id) -> Self {
        // TODO - handle label in a more idiomatic way
        let checkbox = Checkbox::new(text);

        CheckboxWidget {
            value,
            pod: WidgetPod::new(checkbox),
            id,
        }
    }

    // TODO - merge with SingleWidget::request_druid_update ?
    pub fn request_druid_update(&mut self, ctx: &mut ReconcileCtx) {
        self.pod.with_event_context(
            ctx.event_ctx,
            |_widget: &mut Checkbox, ctx: &mut EventCtx| {
                trace!("request_druid_update: {:?}", ctx.widget_id());
                ctx.request_update();
            },
        );
    }
}

impl Widget<DruidAppData> for CheckboxWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        if let Event::MouseUp(_) = event {
            if ctx.is_hot() {
                trace!("Checkbox {:?} value changed: {}", self.id, self.value);
                data.queue_action(self.id, Action::Clicked);
            }
        }
        if let Event::MouseDown(_) = event {
            trace!("MouseDown");
        }
        self.pod.event(ctx, event, &mut self.value, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        _data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.lifecycle(ctx, event, &mut self.value, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &DruidAppData,
        _data: &DruidAppData,
        env: &Env,
    ) {
        self.pod.update(ctx, &mut self.value, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DruidAppData,
        env: &Env,
    ) -> Size {
        let size = self.pod.layout(ctx, bc, &mut self.value, env);
        self.pod.set_origin(ctx, &mut self.value, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &DruidAppData, env: &Env) {
        self.pod.paint(ctx, &mut self.value, env);
    }
}

pub type SingleCheckboxWidget = SingleWidget<CheckboxWidget>;
