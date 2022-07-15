/*
 * drop_trait is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/LeoMeinel/drop_trait/blob/main/LICENSE
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
