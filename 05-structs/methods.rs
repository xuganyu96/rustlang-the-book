struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    /** the first parameter can be &self (immutable reference to the data),
     * &mut self (mutable reference to the data),
     * or self (memory move; will take ownership)
     */
    fn area(&self) -> u32 { // &self and self dictate different behavior
        return self.width * self.height;
    }

    fn integer_scale(&mut self, mul: u32) {
        self.width = self.width * mul;
        self.height = self.height * mul;
    }

    fn destroy(self) {
        println!("instance destroyed");
    }

    fn square(len: u32) -> Self { // "Self" can refer to the type
        return Self {
            width: len,
            height: len,
        };
    }
}

fn main() {
    let mut big_rec: Rectangle = Rectangle { width: 30, height: 50 };
    println!("{}", big_rec.area());

    big_rec.integer_scale(2);
    println!("{}", &big_rec.area()); // automatic referencing/dereferencing!

    big_rec.destroy();
    // will not compile because ownership moved
    // println!("{}", big_rec.area());

    let sqr: Rectangle = Rectangle::square(13);
    println!("{}", &sqr.area());
}

