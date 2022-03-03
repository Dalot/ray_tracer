use async_trait::async_trait;
use ray_tracer::color::RGB;
use std::convert::Infallible;

/// A World is your shared, likely mutable state
pub struct World {
    color: RGB<f32>,
}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl cucumber::World for World {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            color: RGB::<f32>::new(0.0,0.0,0.0),
        })
    }
}

// These are the actual test steps that will be matched against all your features
mod addition_steps {
    use cucumber::Steps;
    use super::RGB;
    pub fn steps() -> Steps<crate::World> {
        let mut builder: Steps<crate::World> = Steps::new();
    
        builder.given("c ← color(-0.5, 0.4, 1.7)", |mut w, _| {
            w.color = RGB::<f32>::new(-0.5, 0.4, 1.7);
            w
        }).then("r ← -0.5, g ← 0.4, b ← 1.7", |w, _ | {
            assert_eq!(w.color, RGB::<f32>::new(-0.5, 0.4, 1.7));
            w
        });
        builder
    }
}

// This runs before everything else, so you can setup things here
#[tokio::main]
async fn main() {
    let runner = cucumber::Cucumber::<World>::new()
        .features(&["./features"])
        .steps(addition_steps::steps());

    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    runner.run().await;
}
