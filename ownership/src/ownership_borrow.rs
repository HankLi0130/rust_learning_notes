pub fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //「&」符號就是參考，它允許你不必獲取所有權來參考它
    println!("The length of '{}' is {}.", s1, len);
    
    let mut s2 = String::from("hello"); // 需加上 mut 表示可修改
    change(&mut s2);
    println!("s2: {}", s2);
}

fn calculate_length(s: &String) -> usize { // s 是個 String 的參考
    s.len()
} // s 在此離開作用域，但因為它沒有它所指向的資料的所有權，所以不會被釋放掉

fn change(s: &mut String) { // 可變參考 &mut s
    s.push_str(", world");
}