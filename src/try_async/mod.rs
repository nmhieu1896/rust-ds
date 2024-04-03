#[tokio::main]
pub async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);

    let v = vec![1, 2, 3];

    tokio::spawn(async move {
        println!("Here's a vec: {:?}", v);
    })
    .await
    .unwrap();
}
