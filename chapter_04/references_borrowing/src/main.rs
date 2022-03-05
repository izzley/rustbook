// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// ##############

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what 
// it refers to, nothing happens
// #######################
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
//     println!("Success!")
// }

// ###############
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
    
//     println!("{}, {}", r1, r2);

//     let r3 = &mut s;

//     println!("{}", r3);

// }

// ######################

fn main() {
    let reference_to_nothing = dangle();
    
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
