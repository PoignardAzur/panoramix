use crate::widget_sequence::FlexWidget;
use crate::widget_sequence::WidgetSequence;

use either::{Either, Left, Right};

// ---

impl<Child: WidgetSequence> WidgetSequence for Option<Child> {
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        self.iter_mut().flat_map(|child| child.widgets()).collect()
    }
}

impl<ChildLeft: WidgetSequence, ChildRight: WidgetSequence> WidgetSequence
    for Either<ChildLeft, ChildRight>
{
    fn widgets(&mut self) -> Vec<&mut dyn FlexWidget> {
        // TODO - use iterator chain instead?
        match self {
            Left(child_left) => child_left.widgets(),
            Right(child_right) => child_right.widgets(),
        }
    }
}
