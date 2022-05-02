#[allow(dead_code)]
mod use_box {
    fn alloc_data_on_heap() {
        let mut a = Box::new(3);
        println!("a = {}", a); // it prints "a = 3"
        *a = *a + 1;
        println!("a = {}", a); //it prints "a = 4"

        let arr = [0; 1000];
        let arr1 = arr;
        println!("{:?} {:?}", arr.len(), arr1.len());

        let arr = Box::new([0; 1000]);
        let arr1 = arr;
        println!("{:?}", arr1.len());
        // println!("{:?}", arr.len()); //error. ownership of arr is moved to arr1
    }

    pub fn test() {
        alloc_data_on_heap();
    }
}

#[allow(dead_code)]
mod use_deref {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    impl Person {
        fn new(name: String, age: u8) -> Self {
            Person { name, age }
        }
        fn display(self: &mut Person) {
            let Person { name: _, age: _ } = &self;
        }
    }
    fn my_box() {
        struct MyBox<T>(T);

        impl<T> MyBox<T> {
            fn new(x: T) -> Self {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for MyBox<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        let y = MyBox::new(1);
        assert_eq!(1, *y); //when we deref a box, for example *y, rust do *(y.deref())

        fn display(s: &str) {
            println!("{}", s);
        }
        let s = String::from("hello world");
        display(&s); //&String deref ==> &str

        let s = MyBox::new(String::from("hello world."));
        display(&s); //deref continuously: MyBox => String => &str
        let s = MyBox::new(String::from("hello, world"));
        let _s1: &str = &s; //deref continuously
        let _s: String = s.to_string();
    }
    pub fn test() {
        my_box();
    }
}

#[allow(dead_code)]
mod use_drop {
    fn example() {
        struct HasDrop1;
        struct HasDrop2;
        impl Drop for HasDrop1 {
            fn drop(&mut self) {
                println!("Dropping HasDrop1");
            }
        }
        impl Drop for HasDrop2 {
            fn drop(&mut self) {
                println!("Dropping HasDrop2");
            }
        }
        struct HasTwoDrops {
            one: HasDrop1,
            two: HasDrop2,
        }
        impl Drop for HasTwoDrops {
            fn drop(&mut self) {
                println!("Dropping HasTwoDrops");
            }
        }

        struct Foo;
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("Dropping Foo");
            }
        }

        let _x = HasTwoDrops {
            two: HasDrop2,
            one: HasDrop1,
        };
        let _foo = Foo;
        println!("Running");
    }
    pub fn test() {
        example();
    }
}
#[allow(dead_code)]
mod use_rc {
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;

    fn try_rc() {
        let s = String::from("hello world");
        let a = Rc::new(s);
        let b = Rc::clone(&a);
        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

        let a = Rc::new(String::from("hello rust"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&b));
        {
            let c = Rc::clone(&b);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        #[derive(Debug)]
        struct Foo {
            v: i32,
        }
        impl Foo {
            fn new() -> Self {
                Foo { v: 1 }
            }
            fn do_something(&self) {
                println!("v of Foo is {}", self.v);
            }
        }
        let a = Rc::new(Foo::new());
        a.do_something();
    }
    fn try_arc() {
        let s1 = Arc::new(String::from("multiple threads tourist"));
        for _ in 0..10 {
            let s = Arc::clone(&s1);
            let _handle = thread::spawn(move || {
                println!("{}", s);
            });
        }
    }
    pub fn test() {
        try_rc();
        try_arc();
    }
}
#[allow(dead_code)]
mod use_cell {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn try_cell() {
        let c = Cell::new("a");
        let one = c.get();
        c.set("b");
        let two = c.get();
        println!("{}, {}", one, two);
    }

    fn try_refcell() {
        let s = RefCell::new(String::from("hello rust"));
        let s1 = s.borrow();
        let s2 = s.borrow();
        println!("{} || {}", s1, s2);
        // let s3 = s.borrow_mut();//error. borrow_mut and borrow
        drop(s1);
        drop(s2);
        let mut s3 = s.borrow_mut();
        s3.push_str("!");
        println!("{}", s3);
        *s3 = "no letters".to_string();
        println!("{}", s3);
    }
    fn interior_mut() {
        pub trait Messenger {
            fn send(&self, msg: String);
        }
        struct MsgQueue {
            msg_cache: RefCell<Vec<String>>,
        }
        impl Messenger for MsgQueue {
            fn send(&self, msg: String) {
                println!("push \"{}\" to messege queue", msg);
                self.msg_cache.borrow_mut().push(msg);
            }
        }

        let mq = MsgQueue {
            msg_cache: RefCell::new(Vec::new()),
        };
        mq.send("hello rust".to_string());
    }
    fn combination_rc_ref_cell() {
        let s = Rc::new(RefCell::new("i have multiple owners".to_string()));
        let s1 = s.clone();
        let s2 = s.clone();
        println!("before change : {}", s1.borrow());
        s2.borrow_mut().push_str(", =======");
        println!("after change : {}", s1.borrow());
    }
    pub fn test() {
        try_cell();
        try_refcell();
        interior_mut();
        combination_rc_ref_cell();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use_box::test();
        use_deref::test();
        println!("-----------------------------");
        use_drop::test();
        println!("-----------------------------");
        use_rc::test();
        println!("-----------------------------");
        use_cell::test();
    }
}
