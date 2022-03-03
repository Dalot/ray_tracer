use ray_tracer::point::Point;
use ray_tracer::vector::Vector;
#[derive(Default, Debug)]
pub struct Projectile {
    pos: Point,
    vel: Vector,
}

#[derive(Default, Debug)]
pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

pub fn tick(p: &mut Projectile, env: &Environment) {
    p.pos = *p.pos() + *p.vel();
    p.vel = *p.vel() + *env.gravity() + *env.wind();
}

impl Projectile {
    pub fn new(pos: Point, vel: Vector) -> Projectile {
        Projectile { pos, vel }
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn vel(&self) -> &Vector {
        &self.vel
    }
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        Environment { gravity, wind }
    }

    pub fn gravity(&self) -> &Vector {
        &self.gravity
    }

    pub fn wind(&self) -> &Vector {
        &self.wind
    }
}

fn main() {
    let mut p = Projectile::new(Point::new(0.0, 1.0, 0.0), Vector::new(1.0, 1.0, 0.0));
    let e = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    let mut y = p.pos().y();
    while y > 0.0 {
        println!("positions: {:?}", p.pos());
        tick(&mut p, &e);
        y = p.pos().y();
    }
    println!("positions: {:?}", p.pos());
}
