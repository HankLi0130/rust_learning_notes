fn main() {
    // function pointer
    // 給一個已經定義好的函數當成參數傳入，類似 Kotlin 中的 Function Reference (::my_api)
    request_1(my_api);

    // closure
    // 實作參數中 closure 的 body，類似 Kotlin 中的 lambda (call_api = {})
    request_2(|domain| {
        println!("Send a Request to {} by closure!", domain);
        String::from("OK")
    });
}

// function pointer
// 注意，function pointer 宣告為 fn，這裡的 f 為小寫
fn request_1(call_api: fn(domain: &str) -> String) {
   // do something ...

    let result = call_api("www.domain.com");
    println!("Result: {}", result);
}

fn my_api(domain: &str) -> String {
    println!("Send a Request to {} by function pointer!", domain);
    String::from("OK")
}

// closure
// 注意，使用 closure 要實作 Fn trait，這裡的 F 為大寫
fn request_2<CallApiType>(call_api: CallApiType)
where
    CallApiType: Fn(&str) -> String,
{
    // do something ...

    let result = call_api("www.domain.com");
    println!("Result: {}", result);
}
