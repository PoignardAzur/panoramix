use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

pub struct WidgetTuple<
    WS0: WidgetSequence,
    WS1: WidgetSequence,
    WS2: WidgetSequence,
    WS3: WidgetSequence,
>(pub WS0, pub WS1, pub WS2, pub WS3);

impl<WS0: WidgetSequence, WS1: WidgetSequence, WS2: WidgetSequence, WS3: WidgetSequence>
    WidgetSequence for WidgetTuple<WS0, WS1, WS2, WS3>
{
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        let mut all_widgets = Vec::new();
        all_widgets.append(&mut self.0.widgets());
        all_widgets.append(&mut self.1.widgets());
        all_widgets.append(&mut self.2.widgets());
        all_widgets.append(&mut self.3.widgets());
        all_widgets
    }
}
