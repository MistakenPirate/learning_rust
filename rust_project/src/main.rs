// use std::collections::HashMap;

fn main() {
    // let x: i32 = -5;
    // let y: u32 = 1000;
    // let z: f32 = 1000.001;
    // print!("x = {}, y = {}, z = {}\n", x, y, z);
    //if we dont give a type rust by default infers any integer as i32 and float as f64

    // let mut x: i8 = 10000;
    // for i in 0..1000 {
    //     x = x + 100;
    // }
    // print!("x = {}\n", x);
    //by default all variables in rust are mutable, unless mut is defined explicitly

    // bools
    // let is_male : bool = true;
    // let is_above_18 : bool = true;
    // if is_male{
    //     println!("You are male")
    // } else {
    //     println!("You are not a male")
    // }
    // if is_male && is_above_18{
    //     println!("You are an adult")
    // }

    // let greeting : String = String::from("Hello World");
    // println!("{}", greeting);
    // let char1: Option<char> = greeting.chars().nth(1);
    // match char1{
    //     Some(c) => println!("{}", c),
    //     None => println!("No character at index 1000"),
    // }

    // let sentence : String = String::from("sameer is learning rust");
    // let first_word : String = get_first_word(sentence);
    // println!("First word is : {}",first_word);

    // let n = 1000;
    // let mut sum = 0;
    // for i in 0..n{
    //     if i % 250 == 0{
    //         sum += i;
    //     }
    // }
    // println!("Sum: {}", sum);

    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // even_values(&mut vec);
    // println!("{:?}",vec);

    // let numbers = vec![1,2,3];
    // for number in numbers{
    //     println!("{}", number);
    // }

    // let mut users: HashMap<String, i32> = HashMap::new();
    // users.insert(String::from("sameer"),21);
    // users.insert(String::from("zoro"),22);

    // let user1 = users.get("sameer");

    // println!("{}", user1.unwrap())

    // let pairs: Vec<(String, i32)> = vec![
    //     (String::from("sameer"),21),
    //     (String::from("zoro"),22)
    // ];

    // let grouped_pairs = group_values_by_key(pairs);
    // for (key, value) in grouped_pairs{
    //     println!("{}: {:?}", key, value);
    // }

    // the iter method in rust provides a way to iterate over the elements
    // of a collection by borrowing them.
    // we cant mutate the variable since we have an immutable reference to
    // the iternal elements

    // let nums = vec![1,2,3];
    // let iter = nums.iter();

    // for value in iter{
    //     println!("{}", value);
    // }

    // we cant mutate the data below either
    // the iterator is mutable but the inner elements (val)
    // still is an immutable reference

    // let nums = vec![1,2,3];
    // let mut iter = nums.iter();

    // while let Some(val) = iter.next(){
    //     print!("{}", val)
    // }

    // iterMut

    // let mut nums = vec![1,2,3];
    // let iter = nums.iter_mut();

    // for value in iter{
    //     *value = *value + 1;
    // }

    // println!("{:?}", nums)


    // IntoIter
    // the intoiterator trait is used to convert 
    // a collection into an iterator that takes ownership
    // of the collection
    // useful when:
    // 1. you no longer need the original collection
    // when you need to squeeze performance benefits by transferring
    // ownership (avoiding references)

    // let nums = vec![1,2,3];
    // let iter = nums.into_iter();

    // for value in iter{
    //     println!("{}", value);
    // }

    // the above code is same as the original iterator i.e
    // for value in nums{
    //     println!("{}", value);
    // }

    // 1. Iter: if you want immutable references to the inner variable but dont want to transfer ownership
    // 2. IterMut: if you want mutable references to the inner variable dont want to transfer ownership
    // 3. IterInto: If you want to move the variable into the iterator and don't want to use it afterwards

}

// fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32>{
//     let mut map = HashMap::new();
//     for (key, value) in pairs{
//         map.insert(key, value);
//     }
//     return map;
// }


// fn even_values(v: &mut Vec<i32>){
//     let mut i = 0;
//     while i < v.len(){
//         if v[i] % 2 != 0{
//             v.remove(i);
//         } else {
//             i += 1;
//         }
//     }
// }

// fn get_first_word(sentence: String) -> String{
//     let mut ans = String::from("");
//     for char in sentence.chars(){
//         ans.push_str(char.to_string().as_str());
//         if char == ' '{
//             break;
//         }
//     }
//     return ans;
// }