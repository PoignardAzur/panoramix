use crate::glue::DruidAppData;
use crate::metadata::{Metadata, NoState};
use druid::{Env, EventCtx};
use std::any::{type_name, Any, TypeId};

/// Context type passed to all components when building them.
pub struct CompCtx<'a> {
    pub(crate) local_state: &'a dyn Any,
}

impl<'a> CompCtx<'a> {
    /// Returns the local state of the current component instance.
    ///
    /// Panics if the generic type doesn't match the component's local state type.
    pub fn use_local_state<T: 'static>(&self) -> &'a T {
        if (*self.local_state).type_id() == TypeId::of::<NoState>() {
            panic!("error: 'use_local_state' cannot be called for a component whose root element isn't ComponentOutput")
        }
        self.local_state.downcast_ref::<T>().unwrap()
    }

    // TODO - add methods
    // get metadata
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
        // TODO - Add one failing test for this
        self.state.downcast_mut::<ComponentState>().expect(&format!(
            "internal type error: event handler expected {:?} ({}), parent component gave {:?}",
            TypeId::of::<ComponentState>(),
            type_name::<ComponentState>(),
            type_id,
        ))
    }
}
