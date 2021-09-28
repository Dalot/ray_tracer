use async_trait::async_trait;
use ray_tracer::tuples::{Point, Vector};
use std::convert::Infallible;

/// A World is your shared, likely mutable state
pub struct TuplesWorld {
    point: Point,
    vector: Vector,
    norm: Vector,
}

/// `cucumber::World` needs to be implemented so this World is accepted in `Steps`
#[async_trait(?Send)]
impl cucumber::World for TuplesWorld {
    // We require some error type
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let v = Vector::new(4.3, -4.2, 3.1);
        Ok(Self {
            point: Point::new(4.3, -4.2, 3.1),
            vector: v,
            norm: v.norm(),
        })
    }
}

// These are the actual test steps that will be matched against all your features
mod addition_steps {
    use super::{Vector, Point};
    use cucumber::Steps;
    use ray_tracer::helpers::f64_equal;

    pub fn steps() -> Steps<crate::TuplesWorld> {
        let mut builder: Steps<crate::TuplesWorld> = Steps::new();

        builder
            .given("a ← point(4.3, -4.2, 3.1)", |w, _| w)
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
            });

        builder
            .given("a ← vector(4.3, -4.2, 3.1)", |w, _| w)
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
            });

        builder.given("p ← point(4.3, -4.2, 3.1)", |w, _| w).then(
            "p = point(4.3, -4.2, 3.1)",
            |w, _| {
                assert_eq!(w.point, Point::new(4.3, -4.2, 3.1));
                w
            },
        );

        builder.given("v ← vector(4.3, -4.2, 3.1)", |w, _| w).then(
            "v = vector(4.3, -4.2, 3.1)",
            |w, _| {
                assert_eq!(w.vector, Vector::new(4.3, -4.2, 3.1));
                w
            },
        );

        builder
            .given("p ← point(4.3, -4.2, 3.1)", |w, _| w)
            .then("p == point(4.3, -4.2, 3.1)", |w, _| {
                assert_eq!(w.point, Point::new(4.3, -4.2, 3.1));
                w
            });

        builder
            .given(
                "a1 ← point(3, -2, 5) AND a2 ← vector(-2, 3, 1)",
                |mut w, _| {
                    w.point = Point::new(3.0, -2.0, 5.0);
                    w.vector = Vector::new(-2.0, 3.0, 1.0);
                    w
                },
            )
            .then("a1 + a2 = tuple(1, 1, 6, 1)", |w, _| {
                let res = w.point + w.vector;
                assert_eq!(res, Point::new(1.0, 1.0, 6.0));
                w
            });

        builder
            .given(
                "a1 ← vector(3, -2, 5) AND a2 ← point(-2, 3, 1)",
                |mut w, _| {
                    w.vector = Vector::new(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 + a2 = tuple(1, 1, 6, 0)", |w, _| {
                let other = Vector::new(-2.0, 3.0, 1.0);
                let res = w.vector + other;
                assert_eq!(res, Vector::new(1.0, 1.0, 6.0));
                w
            });

        builder
            .given(
                "a1 ← point(3, -2, 5) AND a2 ← vector(-2, 3, 1)",
                |mut w, _| {
                    w.point = Point::new(3.0, -2.0, 5.0);
                    w.vector = Vector::new(-2.0, 3.0, 1.0);
                    w
                },
            )
            .then("a1 - a2 = point(5, -5, 4)", |w, _| {
                let res = w.point - w.vector;
                assert_eq!(res, Point::new(5.0, -5.0, 4.0));
                w
            });

        builder
            .given(
                "a1 ← point(3, -2, 5) AND a2 ← point(-2, 3, 1)",
                |mut w, _| {
                    w.point = Point::new(3.0, -2.0, 5.0);
                    w
                },
            )
            .then("a1 - a2 = vector(5, -5, 4)", |w, _| {
                let other = Point::new(-2.0, 3.0, 1.0);
                let res = w.point - other;
                assert_eq!(res, Vector::new(5.0, -5.0, 4.0));
                w
            });

            // TODO: CHECK THIS ONE
        builder
            .given("v ← vector(1, -2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, -2.0, 3.0);
                w
            })
            .then("-v = vector(-1, 2, -3)", |w, _| {
                let res = -w.vector;
                assert_eq!(res, Vector::new(-1.0, 2.0, -3.0));

                w
            });

        builder
            .given("a ← vector(1, -2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, -2.0, 3.0);
                w
            })
            .then("a * 3.5 = vector(3.5, -7, 10.5)", |w, _| {
                let res = w.vector * 3.5;
                assert_eq!(res, Vector::new(3.5, -7.0, 10.5));

                w
            });

        builder
            .given("a ← vector(1, -2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, -2.0, 3.0);
                w
            })
            .then("a * 0.5 = vector(0.5, -1, 1.5)", |w, _| {
                let res = w.vector * 0.5;
                assert_eq!(res, Vector::new(0.5, -1.0, 1.5));

                w
            });

        builder
            .given("a ← vector(1, -2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, -2.0, 3.0);
                w
            })
            .then("a / 2 = vector(0.5, -1, 1.5)", |w, _| {
                let res = w.vector / 2.0;
                assert_eq!(res, Vector::new(0.5, -1.0, 1.5));

                w
            });

        builder
            .given("v ← vector(1, 0, 0)", |mut w, _| {
                w.vector = Vector::new(1.0, 0.0, 0.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(0, 1, 0)", |mut w, _| {
                w.vector = Vector::new(0.0, 1.0, 0.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(0, 0, 1)", |mut w, _| {
                w.vector = Vector::new(0.0, 0.0, 1.0);
                w
            })
            .then("magnitude(v) = 1", |w, _| {
                assert!(f64_equal(1.0, w.vector.mag()));
                w
            });

        builder
            .given("[1] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, 2.0, 3.0);
                w
            })
            .then("magnitude(v) = √14", |w, _| {
                assert!(f64_equal(14_f64.sqrt(), w.vector.mag()));
                w
            });

        builder
            .given("[2] v ← vector(-1, -2, -3)", |mut w, _| {
                w.vector = Vector::new(-1.0, -2.0, -3.0);
                w
            })
            .then("magnitude(v) = √14", |w, _| {
                assert!(f64_equal(14_f64.sqrt(), w.vector.mag()));
                w
            });

        builder
            .given("v ← vector(4, 0, 0)", |mut w, _| {
                w.vector = Vector::new(4.0, 0.0, 0.0);
                w
            })
            .then("normalize(v) = vector(1, 0, 0)", |w, _| {
                assert_eq!(Vector::new(1.0, 0.0, 0.0), w.vector.norm());
                w
            });

        builder
            .given("[normalization] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, 2.0, 3.0);
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

        builder
            .given("[3] v ← vector(1, 2, 3)", |mut w, _| {
                w.vector = Vector::new(1.0, 2.0, 3.0);
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
            .given(
                "a ← vector(1, 2, 3) AND b ← vector(2, 3, 4)",
                |mut w, _| {
                    w.vector = Vector::new(1.0, 2.0, 3.0);
                    w
                },
            )
            .then("dot(a, b) = 20", |w, _| {
                let other = Vector::new(2.0, 3.0, 4.0);
                assert!(f64_equal(20.0, w.vector.dot(&other)));
                w
            });

        builder
            .given(
                "[2] a ← vector(1, 2, 3) And b ← vector(2, 3, 4)",
                |mut w, _| {
                    w.vector = Vector::new(1.0, 2.0, 3.0);
                    w
                },
            )
            .then("cross(a, b) = vector(-1, 2, -1)", |w, _| {
                let other = Vector::new(2.0, 3.0, 4.0);
                assert_eq!(Vector::new(-1.0, 2.0, -1.0), w.vector.cross(&other));
                w
            })
            .then("cross(b, a) = vector(1, -2, 1)", |w, _| {
                let other = Vector::new(2.0, 3.0, 4.0);
                assert_eq!(Vector::new(1.0, -2.0, 1.0), other.cross(&w.vector));
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
