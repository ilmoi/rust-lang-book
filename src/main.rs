struct A {}

// -----------------------------------------------------------------------------
// >>> define using "associated type"
trait B {
    type T; // we call this an "associated type"

    fn b(&self, x:Self::T) -> Self::T {x}
}

// >>> this is how you actually put it to use:
// impl B for A {
//     type T = i32;
// }

impl B for A {
    type T = f64;
}

// -----------------------------------------------------------------------------
// >>> define using generics
trait C<T> {
    fn c(&self, x:T) -> T {x}
}

impl C<f64> for A {}
impl C<i32> for A {}

// -----------------------------------------------------------------------------

fn main() {
    let a = A{};

    //assoc type - CANT IMPLEMENT BOTH. NEED TO CHOOSE JUST ONE. THUS MORE RESTRICTIVE.
    println!("{}", a.b(55.55));
    // println!("{}", a.b(55));

    //generics - CAN IMPLEMENT BOTH AT THE SAME TIME
    println!("{}", a.c(55.55));
    println!("{}", a.c(55));
}

// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// calling associated functions with same name

trait Animal {
    fn baby_name() -> String; //doesnt take self
    fn self_baby_name(&self) -> String; // does take self
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
    fn self_baby_name(&self) -> String {
        String::from("123123")
    }
}

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::self_baby_name(&Dog));
}