# Function pointer 跟 Closure 的差別

請參考 [src](src) 底下的程式碼．

## Function Pointer

給一個已經定義好的函數當成參數傳入，類似 Kotlin 中的 Function Reference．

```rust
// Rust example

fn main() {
    request_1(my_api);
}

fn request_1(call_api: fn(domain: &str) -> String) {
    // do something ...

    let result = call_api("www.domain.com");
    println!("Result: {}", result);
}

fn my_api(domain: &str) -> String {
    println!("Send a Request to {} by function pointer!", domain);
    String::from("OK")
}
```

```kotlin
// Kotlin example

fun main() {
    request1(::myApi)
}

fun request1(callApi: (domain: String) -> String) {
    // do something ...

    val result = callApi("www.domain.com");
    println("Result: $result")
}

fun myApi(domain: String): String {
    println("Send a Request to $domain by function pointer!")
    return "OK"
}
```

## Closure 

實作參數中函數的 body，類似 Kotlin 中的 lambda．

```rust
// Rust example

fn main() {
    request_2(|domain| {
        println!("Send a Request to {} by closure!", domain);
        String::from("OK")
    });
}

fn request_2<CallApiType>(call_api: CallApiType)
where
    CallApiType: Fn(&str) -> String,
{
    // do something ...

    let result = call_api("www.domain.com");
    println!("Result: {}", result);
}
```

```kotlin
// Kotlin example

fun main() {
     request2 { domain ->
        println("Send a Request to $domain by closure!")
        return@request2 "OK"
    }
}

fun request2(callApi: (domain: String) -> String) {
    // do something ...

    val result = callApi("www.domain.com");
    println("Result: $result")
}
```