use crate::element_tree::ReconcileCtx;
use crate::element_tree::{Element, NoEvent, VirtualDom};
use crate::widgets::WidgetSeqBox;

use derivative::Derivative;
use std::any::Any;
use std::fmt::Debug;

// --- STATE ---

pub trait AnyState: Any + Debug {
    fn dyn_clone(&self) -> Box<dyn AnyState>;
    fn dyn_eq(&self, other: &Box<dyn AnyState>) -> bool;
}

impl<T> AnyState for T
where
    T: Clone + Default + Debug + PartialEq + 'static,
{
    fn dyn_clone(&self) -> Box<dyn AnyState> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &Box<dyn AnyState>) -> bool {
        // TODO unwrap
        Any::downcast_ref::<Self>(other).unwrap().eq(self)
    }
}

pub struct AnyStateBox {
    value: Box<dyn AnyState>,
}

impl AnyStateBox {
    pub fn new(state: impl AnyState) -> Self {
        AnyStateBox {
            value: Box::new(state),
        }
    }
}

impl Clone for AnyStateBox {
    fn clone(&self) -> Self {
        AnyStateBox {
            value: self.value.dyn_clone(),
        }
    }
}

impl Debug for AnyStateBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.value.fmt(f)
    }
}

impl PartialEq for AnyStateBox {
    fn eq(&self, other: &Self) -> bool {
        self.value.dyn_eq(&other.value)
    }
}

// --- ELEMENT ---

trait AnyElement: Any + Debug {
    fn build(
        &mut self,
        prev_state: Option<AnyStateBox>,
    ) -> (Box<dyn AnyVirtualDom>, Option<AnyStateBox>);
}

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Default(bound = "Child: Default"), Clone(bound = "Child: Clone"))]
struct ErasedElement<Child: Element<CpEvent, CpState>, CpEvent, CpState> {
    child: Option<Child>,
    _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

impl<Child: Element<CpEvent, CpState>, CpEvent, CpState> Debug
    for ErasedElement<Child, CpEvent, CpState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.as_ref().unwrap().fmt(f)
    }
}

impl<Child: Element<CpEvent, CpState> + 'static, CpEvent: 'static, CpState: 'static> AnyElement
    for ErasedElement<Child, CpEvent, CpState>
{
    fn build(
        &mut self,
        prev_state: Option<AnyStateBox>,
    ) -> (Box<dyn AnyVirtualDom>, Option<AnyStateBox>) {
        let child = self.child.take().unwrap();

        let prev_state = if let Some(mut prev_state) = prev_state {
            std::mem::take(
                Any::downcast_mut::<Child::AggregateChildrenState>(&mut prev_state.value).unwrap(),
            )
        } else {
            Default::default()
        };

        let (output, state) = child.build(prev_state);

        (
            Box::new(ErasedVirtualDom {
                child: output,
                _markers: Default::default(),
            }),
            Some(AnyStateBox {
                value: Box::new(state),
            }),
        )
    }
}

// -

pub struct ElementBox<CpEvent, CpState> {
    child: Box<dyn AnyElement>,
    _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

impl<CpEvent: 'static, CpState: 'static> ElementBox<CpEvent, CpState> {
    pub fn new(child: impl Element<CpEvent, CpState> + 'static) -> Self {
        ElementBox {
            child: Box::new(ErasedElement {
                child: Some(child),
                _markers: Default::default(),
            }),
            _markers: Default::default(),
        }
    }
}

impl<CpEvent, CpState> Debug for ElementBox<CpEvent, CpState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

impl<CpEvent, CpState> Element<CpEvent, CpState> for ElementBox<CpEvent, CpState> {
    type Event = NoEvent;
    type AggregateChildrenState = Option<AnyStateBox>;
    type BuildOutput = VirtualDomBox<CpEvent, CpState>;

    fn build(
        self,
        prev_state: Option<AnyStateBox>,
    ) -> (VirtualDomBox<CpEvent, CpState>, Option<AnyStateBox>) {
        let mut child = self.child;
        let (output, state) = child.build(prev_state);

        (
            VirtualDomBox {
                child: output,
                _markers: Default::default(),
            },
            state,
        )
    }
}

// --- VIRTUAL_DOM ---

#[derive(Derivative, PartialEq, Eq, Hash)]
#[derivative(Default(bound = "Child: Default"), Clone(bound = "Child: Clone"))]
pub struct ErasedVirtualDom<Child: VirtualDom<CpEvent, CpState>, CpEvent, CpState> {
    child: Child,
    _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

impl<Child: VirtualDom<CpEvent, CpState>, CpEvent, CpState> Debug
    for ErasedVirtualDom<Child, CpEvent, CpState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

pub trait AnyVirtualDom: Any + Debug {
    fn init_tree(&self) -> WidgetSeqBox;

    fn reconcile(
        &self,
        other: &Box<dyn AnyVirtualDom>,
        widget_seq: &mut WidgetSeqBox,
        ctx: &mut ReconcileCtx,
    );
}

impl<Child: VirtualDom<CpEvent, CpState> + 'static, CpEvent: 'static, CpState: 'static>
    AnyVirtualDom for ErasedVirtualDom<Child, CpEvent, CpState>
{
    fn init_tree(&self) -> WidgetSeqBox {
        WidgetSeqBox {
            value: Box::new(self.child.init_tree()),
        }
    }

    fn reconcile(
        &self,
        other: &Box<dyn AnyVirtualDom>,
        widget_seq: &mut WidgetSeqBox,
        ctx: &mut ReconcileCtx,
    ) {
        let other = Any::downcast_ref::<Self>(other).unwrap();
        let widget_seq =
            Any::downcast_mut::<Child::TargetWidgetSeq>(&mut widget_seq.value).unwrap();
        self.child.reconcile(&other.child, widget_seq, ctx);
    }
}

// -

pub struct VirtualDomBox<CpEvent, CpState> {
    child: Box<dyn AnyVirtualDom>,
    _markers: std::marker::PhantomData<(CpState, CpEvent)>,
}

impl<CpEvent, CpState> Debug for VirtualDomBox<CpEvent, CpState> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.child.fmt(f)
    }
}

impl<CpEvent, CpState> VirtualDom<CpEvent, CpState> for VirtualDomBox<CpEvent, CpState> {
    type Event = NoEvent;
    type AggregateChildrenState = Option<AnyStateBox>;
    type TargetWidgetSeq = WidgetSeqBox;

    fn init_tree(&self) -> Self::TargetWidgetSeq {
        self.child.init_tree()
    }

    fn reconcile(
        &self,
        other: &Self,
        widget_seq: &mut Self::TargetWidgetSeq,
        ctx: &mut ReconcileCtx,
    ) {
        self.child.reconcile(&other.child, widget_seq, ctx);
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_tree::assign_empty_state_type;
    use crate::elements::label::Label;
    use insta::assert_debug_snapshot;
    use test_env_log::test;

    #[test]
    fn new_element() {
        let label = ElementBox::new(Label::new("Hello"));
        assert_debug_snapshot!(label);

        assign_empty_state_type(&label);

        let (label_data, _state) = label.build(None);
        assert_debug_snapshot!(label_data);

        // TODO - check state
    }

    // TODO
    // - Event test
    // - Widget test
}
