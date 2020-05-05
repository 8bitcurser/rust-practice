#[derive(Debug)]
struct Square {
    width: u32,
    height: u32
}


// impl keyword for the Square struct
// self as the first parameter used to invoke the fields
// all within the context of the instance. 
// ~~
// We could have had each method and associated function at different impl
// blocks. This is useful when considering traits and types.
impl Square {
    // we dont need to do square: &Square cause
    // rust already knows it. 
    // we use & cause we wanna preserve immutability but we could avoid it
    // and give the method ownership over the instance, in that case we would
    // do &mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Square) -> bool {
        self.width > other.width && self.height > other.height
    }

    // we could define functions that do not take the self parameter and this
    // would be called associated functions instead of methods.
    // Usually used for functions that return a new instance of the struct
    // such as String::from(

    fn from(width: u32, height: u32) -> Square {
        Square{
            width,
            height
        }
    }
}


fn main() {
    let square = Square {
        width: 30,
        height: 50
    };

    let square2 = Square {
        width: 20,
        height: 40
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&square)
    );
    // how we print structs, normal print would fail as the display trait is
    // not implemented on structs due to ambiguity. This is the :? debug print
    // and for it to be used we have to add the #[derive(Debug)] header to the
    // struct, if we use {:#?} the printing is identend --> cool
    println!("The rectangle is the following {:?}", square);

    println!("The area from the method {}", square.area());


    println!("Does square2 fit into square1? {}", square.can_hold(&square2));
    // notice how the associated function is called differently with the ::
    let square3 = Square::from(20, 20);

    println!("A new built square {:#?}", square3);

}

// we borrow instead of take ownership as we may need the square later for 
// other operations, this makes the borrow immutable.
fn area(square: &Square) -> u32 {
    square.width * square.height
}


// structs can have methods and they are defined as functions but withing the
// context of a struct with the first parameter always being &self which
// represents the context! Methods facilitate organization of code.
