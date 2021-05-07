struct SillyCount {
    count: u32,
}

impl SillyCount {
    fn new() -> Self { //no, does not take a mutable ref to self
        Self {
            count: 0,
        }
    }
}

impl Iterator for SillyCount {
    type Item = u32; //yes, the weird = syntax

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 2 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut sc = SillyCount::new();
    println!("{}", sc.count);

    let x = sc.next(); //counts to 1
    println!("{}", x.unwrap()); //can print the return value
    println!("{}", sc.count); //or call count separately

    sc.next(); //counts to 2
    println!("{}", sc.count);

    sc.next(); //does not count any futher
    println!("{}", sc.count);

    //because we implemented the Iterator trait for our struct, we can now call any methods associated with Iterator in the docs
    let last_val = sc.last();
    println!("last val is {}", last_val.unwrap());
}