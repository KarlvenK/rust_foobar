#[allow(dead_code)]
mod vector {
    fn use_of_vec() {
        let mut v = Vec::<i32>::new();
        for i in 0..100 {
            v.push(i);
        }
        let third = &v[2];
        println!("the third elem is {}", third);

        match v.get(2) {
            Some(third) => println!("the third elem is {}", third),
            None => println!("fuck you! There is no third elem!"),
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let first = &v[0];
        println!("the first elem: {}", first);
        v.push(7);
        // println!("the first elem: {}", first); //error.
    }

    fn use_iter() {
        let v = vec![1, 2, 3];
        for i in &v {
            print!("{} ", i);
        }
        println!();
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i *= 2;
            print!("{} ", i);
        }
        println!();
    }
    pub fn test() {
        use_of_vec();
        use_iter();
    }
}

#[allow(dead_code)]
mod hashmap {
    use std::collections::HashMap;
    fn use_of_hashmap() {
        let mut my_gems = HashMap::new();
        my_gems.insert("red", 1);
        my_gems.insert("blue", 2);
        my_gems.insert("shit", 3);

        let team_list = vec![
            ("china".to_string(), 1),
            ("japan".to_string(), 2),
            ("usa".to_string(), 3),
        ];
        let teams_map: HashMap<String, i32> = team_list.into_iter().collect();
        println!("{:?}", teams_map);
    }

    pub fn test() {
        use_of_hashmap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        vector::test();
        hashmap::test();
    }
}
