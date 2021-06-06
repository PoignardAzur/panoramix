#![allow(unused_attributes)]

use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

pub struct WidgetTuple<
    WS0: WidgetSequence,
    WS1: WidgetSequence,
    WS2: WidgetSequence,
    WS3: WidgetSequence,
    WS4: WidgetSequence,
    WS5: WidgetSequence,
    WS6: WidgetSequence,
    WS7: WidgetSequence,
    WS8: WidgetSequence,
    WS9: WidgetSequence,
    WS10: WidgetSequence,
    WS11: WidgetSequence,
>(
    pub WS0,
    pub WS1,
    pub WS2,
    pub WS3,
    pub WS4,
    pub WS5,
    pub WS6,
    pub WS7,
    pub WS8,
    pub WS9,
    pub WS10,
    pub WS11,
);

impl<
        WS0: WidgetSequence,
        WS1: WidgetSequence,
        WS2: WidgetSequence,
        WS3: WidgetSequence,
        WS4: WidgetSequence,
        WS5: WidgetSequence,
        WS6: WidgetSequence,
        WS7: WidgetSequence,
        WS8: WidgetSequence,
        WS9: WidgetSequence,
        WS10: WidgetSequence,
        WS11: WidgetSequence,
    > WidgetSequence for WidgetTuple<WS0, WS1, WS2, WS3, WS4, WS5, WS6, WS7, WS8, WS9, WS10, WS11>
{
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        let mut all_widgets = Vec::new();
        all_widgets.append(&mut self.0.widgets());
        all_widgets.append(&mut self.1.widgets());
        all_widgets.append(&mut self.2.widgets());
        all_widgets.append(&mut self.3.widgets());
        all_widgets.append(&mut self.4.widgets());
        all_widgets.append(&mut self.5.widgets());
        all_widgets.append(&mut self.6.widgets());
        all_widgets.append(&mut self.7.widgets());
        all_widgets.append(&mut self.8.widgets());
        all_widgets.append(&mut self.9.widgets());
        all_widgets.append(&mut self.10.widgets());
        all_widgets.append(&mut self.11.widgets());
        all_widgets
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        let mut all_widgets = Vec::new();
        all_widgets.append(&mut self.0.widgets_mut());
        all_widgets.append(&mut self.1.widgets_mut());
        all_widgets.append(&mut self.2.widgets_mut());
        all_widgets.append(&mut self.3.widgets_mut());
        all_widgets.append(&mut self.4.widgets_mut());
        all_widgets.append(&mut self.5.widgets_mut());
        all_widgets.append(&mut self.6.widgets_mut());
        all_widgets.append(&mut self.7.widgets_mut());
        all_widgets.append(&mut self.8.widgets_mut());
        all_widgets.append(&mut self.9.widgets_mut());
        all_widgets.append(&mut self.10.widgets_mut());
        all_widgets.append(&mut self.11.widgets_mut());
        all_widgets
    }
}
