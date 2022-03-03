mod vectors_and_points;
mod world;
use crate::world::MyWorld;
// This runs before everything else, so you can setup things here
#[tokio::main]
async fn main() {
    let runner = cucumber::Cucumber::<MyWorld>::new()
        .features(&["./features"])
        .steps(vectors_and_points::steps());

    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    runner.run().await;
}
