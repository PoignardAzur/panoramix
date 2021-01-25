use crate::element_tree::{ElementTree, VirtualDom};
use crate::elements::with_event::WithEvent;

pub trait ElementTreeExt<ComponentState, ComponentEvent>:
    ElementTree<ComponentState, ComponentEvent> + Sized
{
    fn with_event<
        Cb: Fn(
            &mut ComponentState,
            &<Self::BuildOutput as VirtualDom<ComponentState, ComponentEvent>>::Event,
        ),
    >(
        self,
        callback: Cb,
    ) -> WithEvent<ComponentState, ComponentEvent, Self, Cb> {
        WithEvent {
            element: self,
            callback,
            _comp_state: Default::default(),
            _comp_event: Default::default(),
        }
    }
}

impl<ComponentState, ComponentEvent, ET: ElementTree<ComponentState, ComponentEvent>>
    ElementTreeExt<ComponentState, ComponentEvent> for ET
{
}
