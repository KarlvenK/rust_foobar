#[allow(dead_code)]
mod get_started {
    use std::fmt::Display;

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn lifetime_of_func() {
        let s1 = String::from("abcd");
        let s2 = "xyz";
        let ret = longest(s1.as_str(), s2);
        println!(
            "between \"{}\" and \"{}\", \"{}\" is the longer one",
            s1, s2, ret
        );
    }

    fn struct_lifetime() {
        #[derive(Debug)]
        struct Imp<'a> {
            part: &'a str,
        }
        {
            let novel = String::from("a. aaaaaaaaaa");
            let first_sentence = novel.split('.').next().expect("could not find");
            let i = Imp {
                part: first_sentence,
            };
            println!("{:?}", i);
        }

        {
            struct ImportantExcerpt<'a> {
                part: &'a str,
            }

            impl<'a> ImportantExcerpt<'a> {
                fn level(&self) -> i32 {
                    3
                }
                fn announce_and_return_part(&self, announcement: &str) -> &str {
                    println!("Attention please: {}", announcement);
                    self.part
                }
            }

            impl<'a: 'b, 'b> ImportantExcerpt<'a> {
                fn announce_another(&self, announcement: &'b str) -> &'b str {
                    println!("Attention please: {}", announcement);
                    self.part
                }
            }
        }

        {
            fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where
                T: Display,
            {
                println!("Announcement! {}", ann);
                if x.len() > y.len() {
                    x
                } else {
                    y
                }
            }
        }
    }

    pub fn run() {
        lifetime_of_func();
        struct_lifetime();
    }
}

#[allow(dead_code)]
mod advanced {
    // #[derive(Debug)]
    // struct Foo;
    // impl Foo {
    //     fn mutate_and_share(&mut self) -> &Self {
    //         &*self
    //     }
    //     fn share(&self) {}
    // }
    // fn example1() {
    //     let mut foo = Foo;
    //     let loan = foo.mutate_and_share();
    //     foo.share();
    //     println!("{:?}", loan);
    // }
    fn reborrow() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Point {
            fn move_to(&mut self, x: i32, y: i32) {
                self.x = x;
                self.y = y;
            }
        }

        let mut p = Point { x: 0, y: 0 };
        let r = &mut p;
        let rr = &*r; //reborrow. we can't use r any more before rr ends its lifetime
        println!("{:?}", rr); //reborrow end
        r.move_to(100, 100);
        println!("{:?}", r);
    }
    fn complex() {
        struct Interface<'b, 'a: 'b> {
            manager: &'b mut Manager<'a>,
        }
        impl<'b, 'a: 'b> Interface<'b, 'a> {
            pub fn noop(self) {
                println!("interface consumed");
            }
        }
        struct Manager<'a> {
            text: &'a str,
        }
        struct List<'a> {
            manager: Manager<'a>,
        }
        impl<'a> List<'a> {
            pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
            where
                'a: 'b,
            {
                Interface {
                    manager: &mut self.manager,
                }
            }
        }

        fn use_list(list: &List) {
            println!("{}", list.manager.text);
        }

        let mut list = List {
            manager: Manager { text: "hello" },
        };
        list.get_interface().noop();
        println!("Interface should be dropped here and the borrow released");
        use_list(&list);
    }
    pub fn run() {
        reborrow();
        complex();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        get_started::run();
        advanced::run();
    }
}
