struct A {
    text: String,
}

struct B {
    text: String,
}

trait Hello {
    fn say_nothing(&self); //not implemented, would have to be overwritten at parent level

    fn say_hello(&self) {
        println!("hello!");
    }
    fn print_text(&self) {
        println!("another hello"); //default behavior
    }
}

impl Hello for A {
    // fn say_hello(); //no need to state if only using default
    fn say_nothing(&self) {} //can't NOT implement. will give an error
    fn print_text(&self) {
        println!("{}", self.text); //overwrite default behavior. can't use self.text from trait coz trait doesn't have it
    }
}

impl Hello for B {
    // fn say_hello(); //no need to state if only using default
    fn say_nothing(&self) {} //can't NOT implement. will give an error
    fn print_text(&self) {
        println!("{}", self.text); //overwrite default behavior. can't use self.text from trait coz trait doesn't have it
    }
}

//3 diff ways to require a param to have a certain trait
fn says_hello(some_struct: &impl Hello) {
    some_struct.say_hello();
}

fn says_hello2<T: Hello>(some_struct: &T) {
    some_struct.say_hello();
}

fn says_hello3<T>(some_struct: &T)
    where T: Hello {
    some_struct.say_hello();
}


fn main() {
    let a = A {text: "am A".into()};
    let b = B {text: "am B".into()};
    a.say_hello();
    a.print_text();
    b.say_hello();
    b.print_text();

    says_hello(&a);
    says_hello2(&a);
    says_hello3(&a);

}


// -----------------------------------------------------------------------------

//normal
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


//using clone
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone(); //we clone the value instead of copying it

    for item in list { //for each reference
        if *item > largest { //deref the value and compare
            largest = item.clone(); //clone the value if bigger
        }
    }

    largest //return actual value, not a reference
}


//using references
fn largest<T: PartialOrd + std::fmt::Display>(list: &[T]) -> &T {
    let mut largest = &list[0]; //assign initial reference

    for item in list { //for each reference in list
        if item > largest { //if reference is bigger than our current reference
            largest = &item; //then replace the reference
        }
    }

    largest //this time we're returning a reference

    //note that list itself goes completely unchanged
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
