/*
 * File: main.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

use std::fmt::{Display, Formatter};

fn main() {
    use_my_smart_pointer();
}

struct MySmartPointer<T: Display> {
    data: T,
}

impl<T: Display> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data: {}", self.data);
    }
}

impl<T: Display> Display for MySmartPointer<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn use_my_smart_pointer() {
    let read = MySmartPointer {
        data: "read".to_string(),
    };
    let _not_read = MySmartPointer { data: 98.23 };
    let manually_dropped = MySmartPointer { data: true };
    println!("read is: {}", read);
    // explicit destructor calls are not allowed here -> could result in a double-free
    drop(manually_dropped);
    println!();
}
