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
                println!("push \"{}\" to message queue", msg);
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

#[allow(dead_code)]
mod ptr_err {
    use std::cell::RefCell;
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::ptr::NonNull;
    use std::rc::{Rc, Weak};

    fn circle_ref() {
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }
        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    List::Cons(_num, item) => Some(item),
                    List::Nil => None,
                }
            }
        }

        let a = Rc::new(List::Cons(1, RefCell::new(Rc::new(List::Nil))));
        println!("a's initial rc count = {}", Rc::strong_count(&a));
        println!("a points to {:?}", a.tail());

        //b ---> a
        let b = Rc::new(List::Cons(2, RefCell::new(Rc::clone(&a))));

        println!("after creating b, rc count of a = {}", Rc::strong_count(&a));
        println!("b's initial rc count = {}", Rc::strong_count(&b));
        println!("b points to {:?}", b.tail());

        //       |---->
        //      a      b
        //       <----/
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("after changing a, b's rc count = {}", Rc::strong_count(&b));
        println!("after changing a, a's rc count = {}", Rc::strong_count(&a));

        // println!("{:?}", a.tail()); //error
    }
    fn try_weak() {
        let nine = Rc::new(9);
        let weak_nine = Rc::downgrade(&nine);
        let string_nine = weak_nine.upgrade();
        assert_eq!(*string_nine.unwrap(), 9);
        drop(nine);
        let strong_nine = weak_nine.upgrade();
        assert_eq!(strong_nine, None);
    }
    fn weak_solve_circle_ref() {
        //owner <==> gadgets
        //owner has multiple gadgets, every single gadget has its owner
        #[derive(Debug)]
        struct Owner {
            name: String,
            gadgets: RefCell<Vec<Weak<Gadget>>>,
        }
        #[derive(Debug)]
        struct Gadget {
            id: i32,
            owner: Rc<Owner>,
        }

        let gadget_owner = Rc::new(Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        });

        let gadget1 = Rc::new(Gadget {
            id: 1,
            owner: gadget_owner.clone(),
        });
        let gadget2 = Rc::new(Gadget {
            id: 2,
            owner: gadget_owner.clone(),
        });
        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget1));
        gadget_owner
            .gadgets
            .borrow_mut()
            .push(Rc::downgrade(&gadget2));
        for gadget_opt in gadget_owner.gadgets.borrow().iter() {
            let gadget = gadget_opt.upgrade().unwrap();
            println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
        }
    }

    fn try_tree() {
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 9,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    fn self_ref() {
        // struct SelfRef<'a> {
        //     value: String,
        //     ptr_to_value: &'a str,
        // }
        // let s = "a".to_string();
        // let v = SelfRef {
        //     value: s,
        //     ptr_to_value: &s,
        // };
        #[derive(Debug)]
        struct WhatAboutThis<'a> {
            name: String,
            nickname: Option<&'a str>,
        }
        impl<'a> WhatAboutThis<'a> {
            fn tie_the_knot(&'a mut self) {
                self.nickname = Some(&self.name[..3]);
            }
        }
        let mut tricky = WhatAboutThis {
            name: "Tom Riddle".to_string(),
            nickname: None,
        };
        println!("{:?}", tricky);
        tricky.tie_the_knot();
        // println!("{:?}", tricky);//error, borrow as immutable while it is also borrowed as mutable
        println!("try selfRef with unsafe:\n");
        {
            #[derive(Debug)]
            struct SelfRef {
                value: String,
                ptr_to_value: *const String, //using "*mut String", you can change value
            }
            impl SelfRef {
                fn new(txt: &str) -> Self {
                    SelfRef {
                        value: txt.to_string(),
                        ptr_to_value: std::ptr::null(),
                    }
                }

                fn init(&mut self) {
                    let self_ref: *const String = &self.value;
                    self.ptr_to_value = self_ref;
                }

                fn value(&self) -> &str {
                    &self.value
                }

                fn ptr_to_value(&self) -> &String {
                    assert!(
                        !self.ptr_to_value.is_null(),
                        "Test::b called without Test::init being called first"
                    );
                    unsafe { &*(self.ptr_to_value) }
                }
            }

            let mut t = SelfRef::new("rust");
            t.init();
            println!("{}, {:p}", t.value(), t.ptr_to_value);
        }

        {
            struct Unmovable {
                data: String,
                slice: NonNull<String>,
                _pin: PhantomPinned,
            }
            impl Unmovable {
                fn new(data: String) -> Pin<Box<Self>> {
                    let res = Unmovable {
                        data,
                        slice: NonNull::dangling(),
                        _pin: PhantomPinned,
                    };
                    let mut boxed = Box::pin(res);
                    let slice = NonNull::from(&boxed.data);
                    unsafe {
                        let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
                        Pin::get_unchecked_mut(mut_ref).slice = slice;
                    }
                    boxed
                }
            }
        }
    }

    pub fn test() {
        circle_ref();
        try_weak();
        println!("-------------------------");
        weak_solve_circle_ref();
        try_tree();
        println!("-------------------------");
        self_ref(); //not common
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
        println!("-----------------------------");
        ptr_err::test();
    }
}
