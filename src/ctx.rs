use crate::glue::DruidAppData;
use crate::metadata::{Metadata, NoState};
use druid::{Env, EventCtx};
use std::any::{type_name, Any, TypeId};

/// Context type passed to all components when building them.
pub struct CompCtx<'a> {
    // Yeah, we're using a cell, sorry. It's very local, though.
    pub(crate) called_use_metadata: std::cell::Cell<bool>,
    pub(crate) local_state: &'a dyn Any,
}

impl<'a> CompCtx<'a> {
    pub fn use_metadata<ComponentEvent: 'static, ComponentState: 'static>(
        &self,
    ) -> Metadata<ComponentEvent, ComponentState> {
        if self.called_use_metadata.get() {
            panic!("error: 'use_metadata' can only be called once per component")
        }
        self.called_use_metadata.set(true);
        Default::default()
    }

    /// Returns the local state of the current component instance.
    ///
    /// Panics if the generic type doesn't match the component's local state type.
    pub fn get_local_state<ComponentEvent: 'static, ComponentState: 'static>(
        &self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> &'a ComponentState {
        #![allow(unused_variables)]
        if (*self.local_state).type_id() == TypeId::of::<NoState>() {
            panic!("error: 'get_local_state' cannot be called for a component whose root element isn't ComponentOutput")
        }
        self.local_state
            .downcast_ref::<ComponentState>()
            .expect(&format!(
            "internal type error: get_local_state expected {:?} ({}), parent component gave {:?}",
            TypeId::of::<ComponentState>(),
            type_name::<ComponentState>(),
            (*self.local_state).type_id(),
        ))
    }

    // TODO - add methods
    // use_lifecycle
    // get_vdom_context
}

/// Context required by [`VirtualDom::reconcile`]
pub struct ReconcileCtx<'a, 'b, 'c, 'd, 'e> {
    pub event_ctx: &'a mut EventCtx<'d, 'e>,
    pub data: &'b mut DruidAppData,
    pub env: &'c Env,
}

pub struct ProcessEventCtx<'e, 's> {
    pub event_queue: &'e mut dyn Any,
    pub state: &'s mut dyn Any,
}

impl<'e, 's> ProcessEventCtx<'e, 's> {
    pub fn event_queue<ComponentEvent: 'static, ComponentState: 'static>(
        &mut self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> &mut Vec<ComponentEvent> {
        #![allow(unused_variables)]
        let type_id = (*self.event_queue).type_id();
        self.event_queue
            .downcast_mut::<Vec<ComponentEvent>>()
            .expect(&format!(
                "internal type error: event handler expected {:?} ({}), parent component gave {:?}",
                TypeId::of::<Vec<ComponentEvent>>(),
                type_name::<Vec<ComponentEvent>>(),
                type_id,
            ))
    }

    pub fn state<ComponentEvent: 'static, ComponentState: 'static>(
        &mut self,
        md: Metadata<ComponentEvent, ComponentState>,
    ) -> &mut ComponentState {
        #![allow(unused_variables)]
        let type_id = (*self.event_queue).type_id();
        self.state.downcast_mut::<ComponentState>().expect(&format!(
            "internal type error: event handler expected {:?} ({}), parent component gave {:?}",
            TypeId::of::<ComponentState>(),
            type_name::<ComponentState>(),
            type_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::NoEvent;
    use test_env_log::test;

    #[derive(Debug, PartialEq, Eq)]
    struct MyEvent(i32);

    #[test]
    fn event_queue() {
        let md: Metadata<MyEvent, NoState> = Default::default();

        let mut event_queue = Vec::<MyEvent>::new();
        let mut ctx = ProcessEventCtx {
            event_queue: &mut event_queue,
            state: &mut NoState,
        };
        ctx.event_queue(md).push(MyEvent(42));

        assert_eq!(event_queue, vec![MyEvent(42)],);
    }

    #[test]
    fn state() {
        let md: Metadata<NoEvent, i64> = Default::default();

        let mut state = 12345_i64;
        let mut ctx = ProcessEventCtx {
            event_queue: &mut Vec::<NoEvent>::new(),
            state: &mut state,
        };

        assert_eq!(*ctx.state(md), 12345_i64,);

        *ctx.state(md) = 123_i64;
        assert_eq!(state, 123_i64,);
    }
}
