use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop();
    println!("{}", v[1]);
    println!("{}", x.unwrap());

    let mut dic: HashMap<u8, bool> = HashMap::new();
    dic.insert(5, true);
    dic.insert(6, false);
    let have_five = dic.remove(&5).unwrap();

    println!("have_five: {}", have_five);

}