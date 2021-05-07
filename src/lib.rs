// //ch7 was about modules - so lib.rs relates to ch7
//
// //mod ch7 tells rust to use contents from another file with the same name
// //pub before mod tells rust to ALSO export it further out
// //taken together this is like export (import) from js
// pub mod ch7;
//
// // -----------------------------------------------------------------------------
// // stuff from main
// use rustbook::ch7;
// use rustbook::ch7::testmod;
//
// fn main() {
//     println!("Hello, world!");
//
//     ch7::test();
//     ch7::testmod::deeptest();
//     testmod::deeptest();
// }
//

// // -----------------------------------------------------------------------------
// // documentation
// // fns need to come from this lib file (so defined here or imported into here)
// // fns also need to be PUB to be visible in the docs - private aren't
// // anything in main is ignored
//
// //! comment on rustbook itself
//
// /// Adds one to the number given.
// ///
// /// # Examples
// ///
// /// ```
// /// let arg = 5;
// /// let answer = rustbook::add_one(arg);
// ///
// /// assert_eq!(6, answer);
// /// ```
// pub fn add_one(x: i32) -> i32 {
//     x + 1
// }
//
// /// This wont generate docs even though I wrote them:(
// fn less_one(x: i32) -> i32 {
//     x + 1
// }