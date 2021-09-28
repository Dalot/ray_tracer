use async_trait::async_trait;
use ray_tracer::tuples::PV;
use std::convert::Infallible;

/// A World is your shared, likely mutable state
pub struct TuplesWorld {
    point: PV,
    vector: PV,
    norm: PV,
}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl cucumber::World for TuplesWorld {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let v = PV::new(4.3, -4.2, 3.1, 0.0);
        Ok(Self {
            point: PV::new(4.3, -4.2, 3.1, 1.0),
            vector: v,
            norm: v.norm(),
        })
    }
}

// These are the actual test steps that will be matched against all your features
mod addition_steps {
    use super::PV;
    use cucumber::Steps;
    use ray_tracer::{helpers::f64_equal, tuples::InvalidOperation};

    pub fn steps() -> Steps<crate::TuplesWorld> {
        let mut builder: Steps<crate::TuplesWorld> = Steps::new();

        builder
            .given("a ← tuple(4.3, -4.2, 3.1, 1.0)", |w, _| w)
            .then("a.x = 4.3", |w, _| {
                assert!(f64_equal(w.point.x(), 4.3));
                w
            })
            .then("a.y = -4.2", |w, _| {
                assert!(f64_equal(w.point.y(), -4.2));
                w
            })
            .then("a.z = 3.1", |w, _| {
                assert!(f64_equal(w.point.z(), 3.1));
                w
            })
            .then("a is a point", |w, _| {
                assert!(w.point.is_point());
                w
            });

        builder
            .given("a ← tuple(4.3, -4.2, 3.1, 0.0)", |w, _| w)
            .then("a.x = 4.3", |w, _| {
                assert!(f64_equal(w.vector.x(), 4.3));
                w
            })
            .then("a.y = -4.2", |w, _| {
                assert!(f64_equal(w.vector.y(), -4.2));
                w
            })
            .then("a.z = 3.1", |w, _| {
                assert!(f64_equal(w.vector.z(), 3.1));
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
            .given("a ← tuple(1, -2, 3, 0)", |mut w, _| {
                w.vector = PV::vector(1.0, -2.0, 3.0);
                w
            })
            .then("a * 3.5 = tuple(3.5, -7, 10.5, 0)", |w, _| {
                let res = (w.vector * 3.5).unwrap();
                assert_eq!(res, PV::vector(3.5, -7.0, 10.5));

                w
            });

        builder
            .given("a ← tuple(1, -2, 3, -0)", |mut w, _| {
                w.vector = PV::vector(1.0, -2.0, 3.0);
                w
            })
            .then("a * 0.5 = tuple(0.5, -1, 1.5, 0)", |w, _| {
                let res = (w.vector * 0.5).unwrap();
                assert_eq!(res, PV::vector(0.5, -1.0, 1.5));

                w
            });

        builder
            .given("a ← tuple(1, -2, 3, -4)", |mut w, _| {
                w.vector = PV::vector(1.0, -2.0, 3.0);
                w
            })
            .then("a / 2 = tuple(0.5, -1, 1.5, -2)", |w, _| {
                let res = (w.vector / 2.0).expect("Could not divide vector");
                assert_eq!(res, PV::vector(0.5, -1.0, 1.5));

                w
            });

        builder
            .given("v ← vector(1, 0, 0)", |mut w, _| {
                w.vector = PV::vector(1.0, 0.0, 0.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(0, 1, 0)", |mut w, _| {
                w.vector = PV::vector(0.0, 1.0, 0.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(0, 0, 1)", |mut w, _| {
                w.vector = PV::vector(0.0, 0.0, 1.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("[1] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = PV::vector(1.0, 2.0, 3.0);
                w
            })
            .then("magnitude(v) = √14", |w, _| {
                assert!(f64_equal(14_f64.sqrt(), w.vector.mag()));
                w
            });

        builder
            .given("[2] v ← vector(-1, -2, -3)", |mut w, _| {
                w.vector = PV::vector(-1.0, -2.0, -3.0);
                w
            })
            .then("magnitude(v) = √14", |w, _| {
                assert!(f64_equal(14_f64.sqrt(), w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(4, 0, 0)", |mut w, _| {
                w.vector = PV::vector(4.0, 0.0, 0.0);
                w
            })
            .then("normalize(v) = vector(1, 0, 0)", |w, _| {
                assert_eq!(PV::vector(1.0, 0.0, 0.0), w.vector.norm());
                w
            });

        builder
            .given("[normalization] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = PV::vector(1.0, 2.0, 3.0);
                w
            })
            .then(
                "normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)",
                |w, _| {
                    let norm_vector = w.vector.norm();
                    let x = (norm_vector.x() * 100000.0).round() / 100000.0;
                    let y = (norm_vector.y() * 100000.0).round() / 100000.0;
                    let z = (norm_vector.z() * 100000.0).round() / 100000.0;
                    assert!(f64_equal(0.26726, x));
                    assert!(f64_equal(0.53452, y));
                    assert!(f64_equal(0.80178, z));
                    w
                },
            );

        // TODO: CHECKPOINT - DOT PRODUCT PAGE 31/286

        builder
            .given("[3] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = PV::vector(1.0, 2.0, 3.0);
                w
            })
            .when("norm ← normalize(v)", |mut w, _| {
                w.norm = w.vector.norm();
                w
            })
            .then("magnitude(norm) = 1", |w, _| {
                assert!(f64_equal(1.0, w.norm.mag()));
                w
            });

        builder
            .given("[4] a ← vector(1, 2, 3) AND b ← vector(2, 3, 4)", |mut w, _| {
                w.vector = PV::vector(1.0, 2.0, 3.0);
                w
            })
            .then("dot(a, b) = 20", |w, _| {
                let other = PV::vector(2.0, 3.0, 4.0);
                assert!(f64_equal(20.0, w.vector.dot(&other)));
                w
            });
        builder
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
