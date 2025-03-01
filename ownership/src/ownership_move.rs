pub fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1); // "hello" 的擁有權由 s1 傳入 takes_and_gives_back 裡再傳出來給 s2
    println!("s2: {}", s2);
    
    let (s3, length) = get_length(s2); // 想同時回多個值（傳擁有權及其他值）可以使用 tuple
    println!("s3: {}, length: {}", s3, length);
}

// 此函式會取得一個 String 然後回傳它
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 進入作用域
    println!("a_string: {}", a_string);

    a_string // 回傳 a_string 並移動給呼叫的函式
}

// 此函式會回傳字串及字串的長度
fn get_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
