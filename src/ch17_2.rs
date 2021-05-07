use std::boxed::Box;

struct Button { //trait obj
    x: u32,
    y: u32,
}

struct Menu { //trait obj
    x: u32,
    y: u32,
    len: u32,
}

struct Screen { //makes use of trait obj
    pub components: Vec<Box<dyn Draw>>, //vector of trait objects w/ trait Draw
}

// -----------------------------------------------------------------------------
// trait

trait Draw { //trait to impl by all trait objects
    fn draw(&self); //no default impl
    //IMPORTANT: must have &self, ie must be a method, not an associated fn
}

// -----------------------------------------------------------------------------
// impl

impl Draw for Button {
    fn draw(&self) {
        println!("drawing a button!")
    }
}

impl Draw for Menu {
    fn draw(&self) {
        println!("drawing a menu!")
    }
}

impl Screen {
    fn time_to_draw(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

// -----------------------------------------------------------------------------

fn main() {
    let b = Box::new(Button{x:1,y:2});
    let m = Box::new(Menu{x:1,y:2,len:3});

    let s = Screen{components: vec![b, m]};

    s.time_to_draw();
}