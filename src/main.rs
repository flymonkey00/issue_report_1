use crate::protos::proto::example::ExampleMessage;

mod protos;

fn main() {
    let msg = ExampleMessage::new();
    println!("{}", msg.field1);
}
