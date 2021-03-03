use panoramix::elements::EmptyElement;
use panoramix::{CompCtx, Element, NoEvent};

use panoramix_derive::component;

#[component]
fn EmptyComponent(_ctx: &CompCtx, _my_props: ()) -> impl Element<NoEvent> {
    EmptyElement::new()
}

fn main() {
    let element = EmptyComponent::new::<NoEvent, ()>(());
    println!("element = {:?}", element);
}
