pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> is a trait object. It's a stand-in for any type that impl's the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

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
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button was drawn with width: {}, height: {}, label: {}"
        , self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox was drawn with width: {}, height: {}, options: {:#?}",
        self.width, self.height, self.options);
    }
}