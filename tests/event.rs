use curious_server::Event;

#[tokio::test(flavor = "current_thread")]
pub async fn main() {
    let iterator = vec![0, 1, 2, 3].into_iter();
    let callback = |a: i32| println!("a: {a}");
    let handle = Event::from_iterator(iterator, callback);
    handle.await.unwrap();
}
