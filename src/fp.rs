#[allow(dead_code)]
mod closure {
    // use std::thread;
    // use std::time::Duration;
    fn do_sports() {
        fn workout(intensity: u32, random_number: u32) {
            let action = || {
                println!("muu......");
                // thread::sleep(Duration::from_secs(1));
                intensity
            };

            if intensity < 25 {
                println!("do {} pull", action());
                println!("a girl is watching at me. do another {} pull", action());
            } else if random_number == 3 {
                println!("do too much. take a break.");
            } else {
                println!("do too much. run {} min", action());
            }
        }
        let intensity = 10;
        let random_number = 7;
        workout(intensity, random_number);
    }
    fn simple_cache() {
        struct Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
        {
            query: T,
            value: Option<E>,
        }

        impl<T, E> Cacher<T, E>
        where
            T: Fn(E) -> E,
            E: Copy,
        {
            fn new(query: T) -> Self {
                Cacher { query, value: None }
            }

            fn value(&mut self, arg: E) -> E {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.query)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
        let mut c = Cacher::new(|a| a);
        let _v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 1);
    }

    fn try_three_fn() {
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool + Copy,
        {
            println!("{}", func(3));
            println!("{}", func(10000));
        }
        let x = vec![1, 2, 3];
        fn_once(|z| z == x.len());

        let mut s = String::new();
        let mut update_string = |str| s.push_str(str);
        update_string("hello~");
        println!("{}", s);

        fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
            f("hello~")
        }

        let mut s = String::new();
        let update_string = |str| s.push_str(str);
        exec(update_string);
        println!("{}", s);

        fn exe<F: FnOnce()>(f: F) {
            // Fn() also works. cause closure also impl Fn()
            f()
        }
        let s = String::new();
        let update_string = move || println!("{}", s);
        exe(update_string);
    }
    fn return_closure() {
        fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 7;

            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x - num)
            }
        }
        let f = factory(1);
        println!("{}", (*f)(10));
    }
    pub fn test() {
        do_sports();
        simple_cache();
        try_three_fn();
        return_closure();
    }
}

#[allow(dead_code)]
mod iterator {
    use std::collections::HashMap;

    fn consumer() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        let v1 = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);

        let names = ["a", "b", "c"];
        let ages = [1, 2, 3];
        let map: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
        println!("{:?}", map);

        struct Shoe {
            size: u32,
            style: String,
        }
        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }
    }
    fn impl_iter() {
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }

        let mut cnt = Counter::new();
        for i in 1..=5 {
            assert_eq!(Some(i), cnt.next());
        }
        assert_eq!(None, cnt.next());

        let cnt = Counter::new();
        for i in cnt.into_iter() {
            println!("{} ", i);
        }

        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);

        let v = vec![1, 2, 3, 4, 5, 6, 7];
        for (i, v) in v.iter().enumerate() {
            println!("the {}th num is {}", i, v);
        }
        let val = v
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx % 2 == 0)
            .map(|(_idx, val)| val)
            .fold(0, |sum, acm| sum + acm);
        println!("sum after fold func equals to {}", val);

        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let zero = "0".to_string();
        let ret = v.iter().fold(zero, |acc, x| format!("({} + {})", acc, x));
        println!("{}", ret);
    }
    pub fn test() {
        consumer();
        impl_iter();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        closure::test();
        println!("------------------------");
        iterator::test();
    }
}
