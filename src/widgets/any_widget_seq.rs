use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use std::any::Any;

pub trait AnyWidgetSeq: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn any_widgets(&self) -> Vec<&dyn FlexWidget>;
    fn any_widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget>;
}

impl<T> AnyWidgetSeq for T
where
    T: WidgetSequence + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn any_widgets(&self) -> Vec<&dyn FlexWidget> {
        self.widgets()
    }

    fn any_widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.widgets_mut()
    }
}

// -

pub struct WidgetSeqBox {
    pub value: Box<dyn AnyWidgetSeq>,
}

impl WidgetSequence for WidgetSeqBox {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        self.value.any_widgets()
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.value.any_widgets_mut()
    }
}
