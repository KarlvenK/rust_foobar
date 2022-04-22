#[allow(dead_code)]
mod str_and_slice {
    pub fn try_slice() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..];
        println!("{} {}", hello, world);

        fn first_word(s: &String) -> &str {
            &s[..1]
        }
        let word = first_word(&s);
        println!("first word of '{}' is {}", s, word);

        let s = "abcdefghijklmnopqrstuvwxyz";
        println!("{}", s.chars().nth(0).unwrap());
        let mut s: String = "test  ".into();

        s.push_str("哈");

        println!("{}", s.chars().nth(6).unwrap());

        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replacen = string_replace.replacen("rust", "RUST", 2);
        println!("{}", new_string_replacen);

        let s1 = String::from("a");
        let s2 = String::from("b");
        let s3 = s1 + &s2;
        println!("{}", s3);
        println!("--------------------------")
    }

    pub fn try_struct() {
        #[derive(Debug)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        let user1 = User {
            email: String::from("xxx@gmail.com"),
            username: String::from("somebody"),
            active: true,
            sign_in_count: 1,
        };

        let user2 = User {
            email: String::from("xxx@outlook.com"),
            ..user1
        };
        println!("{:#?}", user2);
    }

    pub fn try_enum() {
        enum Foo {
            A,
            B,
        }
        let c = Foo::A;
        match c {
            Foo::A => {
                println!();
            }
            _ => {
                println!();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        str_and_slice::try_slice();
        str_and_slice::try_struct();
        str_and_slice::try_enum();
    }

}