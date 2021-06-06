use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

pub struct WidgetList<Child: WidgetSequence> {
    pub children: Vec<Child>,
}

impl<Child: WidgetSequence> WidgetSequence for WidgetList<Child> {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        self.children
            .iter()
            .flat_map(|child| child.widgets())
            .collect()
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.children
            .iter_mut()
            .flat_map(|child| child.widgets_mut())
            .collect()
    }
}
