#[allow(dead_code)]
mod generic {
    use std::fmt::Debug;
    use std::ops::Add;

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn sum<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a + b
    }

    fn display_array<T: std::fmt::Debug>(arr: &[T]) {
        //if we delete '&', then compiler do not know the size of arr at compile-time
        println!("{:?}", arr);
    }
    fn display_array_b<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        //const generic
        println!("{:?}", arr);
    }

    fn gen_struct() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        let integer = Point { x: 1, y: 2 };
        let float = Point::<f64> { x: 1.1, y: 2.2 };
        println!("{:?} {:?}", integer, float);
    }
    //------------------------------------------------------------------
    pub trait Summary {
        fn summarize(&self) -> String {
            //default impl method
            String::from("(Read more...)")
        }
    }

    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    #[derive(Debug)]
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }

    impl Post {
        fn do_something(&self) {
            println!("shit");
        }
    }

    fn gen_trait() {
        use std::fmt::Display;
        impl Summary for Post {
            fn summarize(&self) -> String {
                format!("article: {}, author: {}", self.title, self.author)
            }
        }

        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{} post a weibo: {}", self.username, self.content)
            }
        }
        let post = Post {
            title: "A".to_string(),
            author: "B".to_string(),
            content: "C".to_string(),
        };
        println!("{}", post.summarize());
        let weibo = Weibo {
            username: "A".to_string(),
            content: "B".to_string(),
        };
        println!("{}", weibo.summarize());

        fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        fn notify_plus(item: &(impl Summary + Debug)) {
            println!("Breaking news! {:?}", item.summarize());
        }
        notify(&weibo);
        notify_plus(&weibo);

        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Pair { x, y }
            }
        }
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("the largest member is x");
                } else {
                    println!("the largest member is y");
                }
            }
        }

        #[derive(Debug)]
        struct Point<T: Add<T, Output = T>> {
            x: T,
            y: T,
        }

        impl<T: Add<T, Output = T>> Add for Point<T> {
            type Output = Point<T>;
            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        println!("{:?}", p1 + p2);
    }

    fn trait_obj() {
        trait Draw {
            fn draw(&self);
        }
        struct Button {
            width: u32,
            height: u32,
            label: String,
        }

        impl Draw for Button {
            fn draw(&self) {
                println!(
                    "this is a w: {}, h: {}, label: {}  button",
                    self.width, self.height, self.label
                );
            }
        }

        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                println!(
                    "this is a w: {}, h: {}, options: {:?}  SelectBox",
                    self.width, self.height, self.options
                );
            }
        }

        struct Screen {
            components: Vec<Box<dyn Draw>>,
        }
        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![String::from("Yes"), String::from("No")],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }

    fn try_type() {
        #[derive(Debug)]
        struct Millimeters(u32);
        #[derive(Debug)]
        struct Meters(u32);
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, rhs: Meters) -> Self::Output {
                Millimeters(self.0 + (rhs.0 * 1000))
            }
        }
        println!("{:?}", Millimeters(2) + Meters(1));
    }

    pub fn try_it() {
        println!("1 + 2 = {}", add(1, 2));
        gen_struct();
        let t = [0; 10];
        display_array(&t);
        display_array_b(t);
        gen_trait();
        trait_obj();
        try_type();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        generic::try_it();
    }
}
