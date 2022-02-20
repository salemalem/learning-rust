// A partially defined struct type
struct BagOfHolding<T> {
    // Our parameter type T can be handed to others
    item: Option<T>,
}

fn main() {
    // Note: A bag for i32, holding nothing! We have to specify the type
    // because otherwise Rust would not know what type of bag it is.
    // you can put any type as long as item: None
    // try putting <bool> and nothing will change :D
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!")
    } else {
        println!("there's something in the bag!")
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(42) };

    if i32_bag.item.is_some() {
        println!("there's something in the bag!")
    } else {
        println!("there's nothing in the bag!")
    }

    // match lets us deconstruct Option elegantly and ensure we handle all cases!
    match i32_bag.item {
        Some(v) => println!("found {} in bag!", v),
        None => println!("found nothing"),
    }
}
