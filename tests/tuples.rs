use async_trait::async_trait;
use ray_tracer::tuples::PV;
use std::convert::Infallible;

/// A World is your shared, likely mutable state
pub struct TuplesWorld {
    point: PV,
    vector: PV,
}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl cucumber::World for TuplesWorld {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            point: PV::new(4.3, -4.2, 3.1, 1.0),
            vector: PV::new(4.3, -4.2, 3.1, 0.0),
        })
    }
}

// These are the actual test steps that will be matched against all your features
mod addition_steps {
    use super::PV;
    use cucumber::Steps;
    use ray_tracer::tuples::InvalidOperation;

    pub fn steps() -> Steps<crate::TuplesWorld> {
        let mut builder: Steps<crate::TuplesWorld> = Steps::new();

        builder
            .given("a ← tuple(4.3, -4.2, 3.1, 1.0)", |w, _| w)
            .then("a.x = 4.3", |w, _| {
                assert!(f32_equal(w.point.x(), 4.3));
                w
            })
            .then("a.y = -4.2", |w, _| {
                assert!(f32_equal(w.point.y(), -4.2));
                w
            })
            .then("a.z = 3.1", |w, _| {
                assert!(f32_equal(w.point.z(), 3.1));
                w
            })
            .then("a is a point", |w, _| {
                assert!(w.point.is_point());
                w
            });

        builder
            .given("a ← tuple(4.3, -4.2, 3.1, 0.0)", |w, _| w)
            .then("a.x = 4.3", |w, _| {
                assert!(f32_equal(w.vector.x(), 4.3));
                w
            })
            .then("a.y = -4.2", |w, _| {
                assert!(f32_equal(w.vector.y(), -4.2));
                w
            })
            .then("a.z = 3.1", |w, _| {
                assert!(f32_equal(w.vector.z(), 3.1));
                w
            })
            .then("a is not a point", |w, _| {
                assert!(!w.vector.is_point());
                w
            });

        builder.given("p ← point(4.3, -4.2, 3.1)", |w, _| w).then(
            "p = tuple(4.3, -4.2, 3.1, 1.0)",
            |w, _| {
                assert_eq!(w.point, PV::point(4.3, -4.2, 3.1));
                w
            },
        );

        builder.given("v ← vector(4.3, -4.2, 3.1)", |w, _| w).then(
            "v = tuple(4.3, -4.2, 3.1, 0.0)",
            |w, _| {
                assert_eq!(w.vector, PV::vector(4.3, -4.2, 3.1));
                w
            },
        );

        builder
            .given("p ← point(4.3, -4.2, 3.1)", |w, _| w)
            .then("p == point(4.3, -4.2, 3.1)", |w, _| {
                assert_eq!(w.point, PV::point(4.3, -4.2, 3.1));
                w
            })
            .then("p != vector(4.3, -4.2, 3.1)", |w, _| {
                assert!(w.point != PV::vector(4.3, -4.2, 3.1));
                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 0)",
                |mut w, _| {
                    w.point = PV::point(3.0, -2.0, 5.0);
                    w.vector = PV::vector(-2.0, 3.0, 1.0);
                    w
                },
            )
            .then("a1 + a2 = tuple(1, 1, 6, 1)", |w, _| {
                let res = (w.point + w.vector).unwrap();
                assert_eq!(res, PV::point(1.0, 1.0, 6.0));
                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 0) AND a2 ← tuple(-2, 3, 1, 1)",
                |mut w, _| {
                    w.vector = PV::vector(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 + a2 = tuple(1, 1, 6, 0)", |w, _| {
                let other = PV::point(-2.0, 3.0, 1.0);
                let res = (w.vector + other).unwrap();
                assert_eq!(res, PV::point(1.0, 1.0, 6.0));
                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 1)",
                |mut w, _| {
                    w.vector = PV::point(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 + a2 = InvalidOperation", |w, _| {
                let other = PV::point(-2.0, 3.0, 1.0);
                match w.vector - other {
                    Err(err) => assert_eq!(err, InvalidOperation::InvalidAddition),
                    Ok(val) => {
                        dbg!("test failed {:?}", val);
                    }
                }

                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 0)",
                |mut w, _| {
                    w.point = PV::point(3.0, -2.0, 5.0);
                    w.vector = PV::vector(-2.0, 3.0, 1.0);
                    w
                },
            )
            .then("a1 - a2 = tuple(5, -5, 4, 1)", |w, _| {
                let res = (w.point - w.vector).unwrap();
                assert_eq!(res, PV::point(5.0, -5.0, 4.0));
                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 1) AND a2 ← tuple(-2, 3, 1, 1)",
                |mut w, _| {
                    w.point = PV::point(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 - a2 = tuple(5, -5, 4, 0)", |w, _| {
                let other = PV::point(-2.0, 3.0, 1.0);
                let res = (w.point - other).unwrap();
                assert_eq!(res, PV::vector(5.0, -5.0, 4.0));
                w
            });

        builder
            .given(
                "a1 ← tuple(3, -2, 5, 0) AND a2 ← tuple(-2, 3, 1, 1)",
                |mut w, _| {
                    w.vector = PV::vector(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 - a2 = InvalidOperation", |w, _| {
                let other = PV::point(-2.0, 3.0, 1.0);
                match w.vector - other {
                    Err(err) => assert_eq!(err, InvalidOperation::InvalidSubtraction),
                    Ok(val) => {
                        dbg!("test failed {:?}", val);
                    }
                }

                w
            });

        builder
            .given("v ← vector(1, -2, 3)", |mut w, _| {
                w.vector = PV::vector(1.0, -2.0, 3.0);
                w
            })
            .then("zero - v = vector(-1, 2, -3)", |w, _| {
                let res = -w.vector;
                assert_eq!(res, PV::vector(-1.0, 2.0, -3.0));

                w
            });

        builder
    }

    fn f32_equal(a: f32, b: f32) -> bool {
        let margin = f32::EPSILON;
        (a - b).abs() < margin
    }
}

// This runs before everything else, so you can setup things here
#[tokio::main]
async fn main() {
    let runner = cucumber::Cucumber::<TuplesWorld>::new()
        .features(&["./features"])
        .steps(addition_steps::steps());

    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    runner.run().await;
}
