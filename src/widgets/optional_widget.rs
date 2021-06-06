use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use either::{Either, Left, Right};

// ---

impl<Child: WidgetSequence> WidgetSequence for Option<Child> {
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        self.iter().flat_map(|child| child.widgets()).collect()
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.iter_mut()
            .flat_map(|child| child.widgets_mut())
            .collect()
    }
}

impl<ChildLeft: WidgetSequence, ChildRight: WidgetSequence> WidgetSequence
    for Either<ChildLeft, ChildRight>
{
    fn widgets(&self) -> Vec<&dyn FlexWidget> {
        // TODO - use iterator chain instead?
        match self {
            Left(child_left) => child_left.widgets(),
            Right(child_right) => child_right.widgets(),
        }
    }

    fn widgets_mut(&mut self) -> Vec<&mut dyn FlexWidget> {
        // TODO - use iterator chain instead?
        match self {
            Left(child_left) => child_left.widgets_mut(),
            Right(child_right) => child_right.widgets_mut(),
        }
    }
}
