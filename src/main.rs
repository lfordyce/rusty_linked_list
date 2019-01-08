extern crate linked_list;

use linked_list::third::List;

fn main() {
    let list = List::new();
    let list = list.append(1).append(2).append(3);
    println!("{:?}", list.head().unwrap());
}
