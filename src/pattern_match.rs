#[allow(dead_code)]
mod if_let {
    pub fn basic() {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dir = Direction::South;
        match dir {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            }
            _ => {
                println!("West");
            }
        }

        enum Action {
            Say(String),
            MoveTo(i32, i32),
            ChangeColorRGB(u16, u16, u16),
        }

        let actions = [
            Action::Say("Hello, Rust".to_string()),
            Action::MoveTo(0, 0),
            Action::ChangeColorRGB(0, 0, 0),
        ];
        for action in actions {
            match action {
                Action::Say(s) => {
                    println!("{}", s);
                }
                Action::MoveTo(x, y) => {
                    println!("move to ({}, {})", x, y);
                }
                Action::ChangeColorRGB(x, y, z) => {
                    println!("Change color to ({}, {}, {})", x, y, z);
                }
            }
        }
    }
}

#[allow(dead_code)]
mod match_ {
    pub fn basic() {
        #[derive(Debug)]
        enum MyEnum {
            Foo,
            Bar,
        }

        let v = vec![MyEnum::Foo, MyEnum::Bar];
        // v.iter().filter(|x| x == MyEnum::Foo); //error
        println!("before filter: {:?}", v);
        let v = v
            .into_iter()
            .filter(|x| matches!(x, MyEnum::Foo))
            .collect::<Vec<MyEnum>>();
        println!("after filter: {:?}", v);

        let foo = 'a';
        assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
        let bar = Some(9);
        assert!(matches!(bar, Some(x) if x > 8));

        let age = Some(30);
        println!("before match: age = {:?}", age);
        if let Some(age) = age {
            println!("match: age = {}", age);
        }
        println!("after match: age = {:?}", age);

        let mut stack = vec![1, 2, 3, 4, 5, 6, 7];
        while let Some(x) = stack.pop() {
            print!("{} ", x);
        }
        println!();

        struct Point {
            x: String,
            y: String,
        }
        let p = Point {
            x: "0".to_string(),
            y: "1".to_string(),
        };
        let Point { x: a, y: b } = p;
        println!("{} {}", a, b);

        #[derive(Debug)]
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Rgb(0, 0, 0));
        match msg {
            Message::ChangeColor(Color::Rgb(x, y, z)) => {
                println!("Rgb({}, {}, {})", x, y, z);
            }
            Message::ChangeColor(Color::Hsv(a, b, c)) => {
                println!("Hsv({}, {}, {})", a, b, c);
            }
            _ => (),
        }

        let s = Some(String::from("test"));
        if let Some(_s) = s {
            //move occurs here
            // println!("{}", _s); //it works
            println!("get a value");
        }
        // println!("{}", s.unwrap()); //error

        let s = Some(String::from("test"));
        if let Some(_) = s {
            //s not moved
            ()
        }
        println!("{}", s.unwrap());

        struct T {
            a: i32,
            b: i32,
            c: i32,
        }
        let origin = T { a: 1, b: 2, c: 3 };
        match origin {
            T { c, b, .. } => println!("{} {}", b, c),
        }
    }
    pub fn guard() {
        let num = Some(3);
        match num {
            Some(x) if x < 5 => println!("less than five"), //if is a gurad
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    pub fn binding() {
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 9 };
        match msg {
            Message::Hello {
                id: id_variable @ 3..=10,
            } => {
                println!("Found {} belongs to 3..=10", id_variable);
            }
            Message::Hello { id: 11..=20 } => {
                println!("Found value belongs to 11..=20");
                // println!("{}", id); // error. can not use id.because id is not binded
            }
            Message::Hello { id } => {
                println!("Defualt {}", id);
            }
        }

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p @ Point { x: a, y: b } = Point { x: 1, y: 2 };
        println!("{:?}", p);
        println!("{} {}", a, b);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        if_let::basic();
        println!("-----------------------------------");
        match_::basic();
        println!("-----------------------------------");
        match_::guard();
        match_::binding();
    }
}
