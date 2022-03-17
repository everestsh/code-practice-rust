struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut first = Box::new(Node {
        value: 1,
        next: None,
    });
    let mut second = Box::new(Node {
        value: 2,
        next: None,
    });
    let third = Box::new(Node {
        value: 3,
        next: None,
    });

    second.next = Some(third);
    first.next = Some(second);

    let mut curr: Option<&Box<Node>> = Some(&first);
    while curr.is_some() {
        // we can unwrap the Option because we know for sure that curr is Some
        println!("{}", curr.unwrap().value);
        let next: &Option<Box<Node>> = &curr.unwrap().next;
        // Get an Option with a reference to the next element
        curr = next.as_ref();
    }
}
