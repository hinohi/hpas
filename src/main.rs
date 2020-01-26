use std::env::args;

use rand::{
    distributions::{Bernoulli, Distribution, Uniform},
    Rng, SeedableRng,
};
use rand_pcg::Mcg128Xsl64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Agent {
    hp: u32,
    attack: u32,
    speed: u32,
}

impl Agent {
    pub fn new(attack: u32, speed: u32) -> Agent {
        assert!(attack > 0);
        assert!(speed > 0);
        assert!(attack + speed < 100);
        Agent {
            hp: (100 - attack - speed) * 10,
            attack,
            speed,
        }
    }
}

const ATTACK_FACTOR: u32 = 1024;
const ATTACK_RANGE: u32 = 128;

pub struct Arena {
    p1: Bernoulli,
    p2: Uniform<u32>,
    p3: Uniform<u32>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WhichAgent {
    A,
    B,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            p1: Bernoulli::new(0.5).unwrap(),
            p2: Uniform::new_inclusive(ATTACK_FACTOR - ATTACK_RANGE, ATTACK_FACTOR + ATTACK_RANGE),
            p3: Uniform::new_inclusive(0, 99),
        }
    }

    fn damage<R: Rng>(&self, random: &mut R, attack: u32) -> u32 {
        attack * self.p2.sample(random) / ATTACK_FACTOR
    }

    pub fn battle<R: Rng>(&self, random: &mut R, a: Agent, b: Agent) -> WhichAgent {
        let mut a_hp = a.hp;
        let mut b_hp = b.hp;
        let a_speed_gain = a.speed.saturating_sub(b.speed);
        let b_speed_gain = b.speed.saturating_sub(a.speed);
        let mut a_count = a.speed;
        let mut b_count = b.speed;
        loop {
            let which = if a_count == b_count {
                if self.p1.sample(random) {
                    WhichAgent::A
                } else {
                    WhichAgent::B
                }
            } else if a_count > b_count {
                WhichAgent::A
            } else {
                WhichAgent::B
            };
            match which {
                WhichAgent::A => {
                    if b_speed_gain == 0 || a_speed_gain < self.p3.sample(random) {
                        b_hp = b_hp.saturating_sub(self.damage(random, a.attack));
                    }
                    b_count += b.speed;
                }
                WhichAgent::B => {
                    if a_speed_gain == 0 || a_speed_gain < self.p3.sample(random) {
                        a_hp = a_hp.saturating_sub(self.damage(random, b.attack));
                    }
                    a_count += a.speed;
                }
            }
            if a_hp == 0 {
                return WhichAgent::B;
            }
            if b_hp == 0 {
                return WhichAgent::A;
            }
        }
    }
}

fn main() {
    let mut random = Mcg128Xsl64::from_entropy();
    let arena = Arena::new();

    let mut args = args().skip(1);
    let a = Agent::new(
        args.next().unwrap().parse().unwrap(),
        args.next().unwrap().parse().unwrap(),
    );
    let b = Agent::new(
        args.next().unwrap().parse().unwrap(),
        args.next().unwrap().parse().unwrap(),
    );
    let n = args
        .next()
        .unwrap_or("1000000".to_string())
        .parse()
        .unwrap();
    let mut win = [0, 0];
    for _ in 0..n {
        let result = arena.battle(&mut random, a, b);
        win[result as usize] += 1;
    }
    println!("{} {}", win[0], win[1]);
}
