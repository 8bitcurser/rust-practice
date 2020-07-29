pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // trait objects stored in a vector
    // aka any type that implments the draw trait
    pub components: Vec<Box<dyn Draw>>,
}
// a struct with a generic type parameter would have only allowed one 
// generic type at a time while trait objects allow many different ones as
// long as they implement the trait.
// with the struct we could have only had the type Button stored in the vector
// although many its just one type while trait objects allow to have buttons,
// text fields, whatever if the collection is homogenous use struct if not
// trait objects
// if the type does not implement the trait we dont need to check in runtime
// as it will explode in compiletime


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self){
        // do stuff
    }
}


pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    pub fn draw(&self) {
        //do stuff
    }
}

///Run example...
///fn main() {
/// let screen = Screen {
///     components: vec![
///         Box::new(SelectBox{..}),
///         Box::new(Button{..})
///     ]
/// };
///
/// screen.run()
///}
