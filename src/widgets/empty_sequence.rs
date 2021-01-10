use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

pub struct EmptySequence;

impl WidgetSequence for EmptySequence {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        Vec::new()
    }
}
