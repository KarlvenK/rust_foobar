use std::any::type_name;

fn main() {
    let x = 1_i32;
    let add_x = |a| x + a;
    let result = add_x(5);
    println!("x now is {}", result);

    let v: Vec<i32> = vec![];
    let c = ||drop(v);
    c();
    // c();

    let v = vec![1, 2, 3];
    let c = move||for i in &v {
        println!("{}", i);
    };
    c();
    // println!("{:?}", v);
    c();
    // println!("{:?}", v);

    let t = (String::from("1"), String::from("2"));
    let (s1, s2) = t.clone();
    println!("({}, {}) -->-->--> {:?}", s1, s2, t);
}

#[allow(dead_code)]
fn print_type_of<T> (_: &T) {
    println!("{}", type_name::<T>());
}