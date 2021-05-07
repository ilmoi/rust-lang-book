use std::collections::HashMap;
use std::hash::Hash;

//below implementation correctly updates for new values (uses hashmap) AND is generic over any type that fits onto the stack

struct Cacher<T, X>
where
    T: Fn(X) -> X,
    X: Eq + Hash + Copy,
{
    method: T,
    values: HashMap<X, X>,
}

impl<T, X> Cacher<T, X>
where
    T: Fn(X) -> X,
    X: Eq + Hash + Copy,
{
    fn new(method: T) -> Cacher<T, X> {
        Cacher {
            method,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, k:X) -> X {
        match self.values.get(&k) {
            Some(&v) => v,
            None => {
                let v = (self.method)(k);
                self.values.insert(k, v);
                v
            }
        }
    }
}


fn main() {

    let some_closure = |x| {
        println!("going through closure");
        x
    };

    let mut some_cacher = Cacher::new(some_closure);

    // let some_value = some_cacher.value(3); //goes through closure
    // println!("{}", some_value);
    // let some_value = some_cacher.value(3); //goes to cache
    // println!("{}", some_value);
    // let some_value = some_cacher.value(3); //goes to cache
    // println!("{}", some_value);
    // let some_value = some_cacher.value(4); //goes through closure again, coz diff val
    // println!("{}", some_value);

    //uncomment the above FIRST (otherwise closure bound to int)
    let some_value = some_cacher.value("yay"); //diff type
    println!("{}", some_value);

}