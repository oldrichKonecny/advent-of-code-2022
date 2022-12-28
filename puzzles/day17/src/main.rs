use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    let map = Rc::new(RefCell::new(map));

    // Borrow the value inside the RefCell mutably
    let mut borrowed_map = map.borrow_mut();

    // Mutate the value
    borrowed_map.insert("new_key".to_string(), "new_value".to_string());
    let x = borrowed_map.get_mut("key").unwrap();
    x.push('x');
    println!("x: {:?}", x);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        assert_eq!(2, 1 + 1);
    }
}
