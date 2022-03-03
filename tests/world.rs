use async_trait::async_trait;
use std::convert::Infallible;

/// A World is your shared, likely mutable state
pub struct MyWorld {}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl cucumber::World for MyWorld {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {})
    }
}
