pub fn main() {
    let s = String::from("hello");
    let len = s.len();

    // 要是你想用 Rust 指定範圍的語法 .. 從索引 0 開始的話，你可以省略兩個句點之前的值。換句話說，以下兩個是相等的
    let slice = &s[0..2];
    let slice = &s[..2];

    // 同樣地，如果你的切片包含 String 的最後一個位元組的話，你同樣能省略最後一個數值。這代表以下都是相等的
    let slice = &s[3..len];
    let slice = &s[3..];

    // 如果你要獲取整個字串的切片，你甚至能省略兩者的數值，以下都是相等的
    let slice = &s[0..len];
    let slice = &s[..];
}