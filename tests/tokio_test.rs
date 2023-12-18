use tokio;

async fn say_world() {
    println!("开始执行 say_world");
    println!("YES.")
}

// 
// #[test]
#[tokio::test]
async fn test_tokio() {
    let op = say_world();
    println!("Hello world.");
    // 调用 await  开始执行 say_world
    let res = op.await;
    println!("{:?}", res)
}
