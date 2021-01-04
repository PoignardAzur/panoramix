use crate::element_tree::{ElementTree, VirtualDom};
use crate::elements::with_event::WithEvent;

pub trait ElementTreeExt<ExplicitState>: ElementTree<ExplicitState> + Sized {
    fn with_event<
        Cb: Fn(&mut ExplicitState, &<Self::BuildOutput as VirtualDom<ExplicitState>>::Event),
    >(
        self,
        callback: Cb,
    ) -> WithEvent<Self, Cb, ExplicitState> {
        WithEvent {
            element: self,
            callback,
            _state: Default::default(),
        }
    }
}

impl<ExplicitState, ET: ElementTree<ExplicitState>> ElementTreeExt<ExplicitState> for ET {}
