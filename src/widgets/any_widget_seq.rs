use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use std::any::Any;

pub trait AnyWidgetSeq: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn any_widgets(&mut self) -> Vec<&mut dyn FlexWidget>;
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

    fn any_widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.widgets()
    }
}

// -

pub struct WidgetSeqBox {
    pub value: Box<dyn AnyWidgetSeq>,
}

impl WidgetSequence for WidgetSeqBox {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.value.any_widgets()
    }
}
