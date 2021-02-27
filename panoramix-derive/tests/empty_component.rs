use panoramix::elements::EmptyElement;
use panoramix::{Element, NoEvent};

use panoramix_derive::component;

#[component]
fn EmptyComponent(_state: &(), _my_props: ()) -> impl Element<(), NoEvent> {
    EmptyElement::new()
}

fn main() {
    let element = EmptyComponent::new(());
    println!("element = {:?}", element);
}
