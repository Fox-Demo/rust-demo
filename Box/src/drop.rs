struct CustomSP {
    data: String,
}

impl Drop for CustomSP {
    //Execute this fn when the value goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSP with data: {}", self.data);
    }
}

#[cfg(test)]

mod tests {
    #[test]
    fn test_drop() {
        let c = super::CustomSP {
            data: String::from("test"),
        };

        drop(c); // manually drop the value by `std::mem::drop`
    }
}
