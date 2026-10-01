//! The coffee run: a Frogger-style street crossing. Row 0 is the office
//! sidewalk, rows 1-4 are traffic lanes, and row 5 is the coffee shop sidewalk.

use macroquad::rand::gen_range;

pub const WIDTH: f32 = 640.0;
pub const LANES: usize = 4;
pub const SHOP_ROW: usize = LANES + 1;
pub const INTERN_WIDTH: f32 = 16.0;
pub const DOOR_WIDTH: f32 = 32.0;
pub const OFFICE_DOOR: f32 = 144.0;
pub const SHOP_DOOR: f32 = 464.0;
const HOP: f32 = 16.0;
const CAR: f32 = 40.0;
const TRUCK: f32 = 72.0;
/// Speed ranges in pixels per second for each lane, from the office side out.
/// The near lanes drive right and the far lanes drive left.
const SPEEDS: [(f32, f32); LANES] = [
    (70.0, 100.0),
    (120.0, 160.0),
    (-140.0, -105.0),
    (-220.0, -170.0),
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Car {
    pub x: f32,
    pub length: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lane {
    /// Pixels per second; positive drives right.
    pub speed: f32,
    pub cars: Vec<Car>,
    /// Distance left to drive before the next car enters.
    gap: f32,
}

impl Lane {
    fn drive(&mut self, dt: f32) {
        for car in &mut self.cars {
            car.x += self.speed * dt;
        }
        self.cars
            .retain(|car| car.x > -TRUCK && car.x < WIDTH + TRUCK);
        self.gap -= self.speed.abs() * dt;
        if self.gap <= 0.0 {
            let length = if gen_range(0, 4) == 0 { TRUCK } else { CAR };
            let x = if self.speed > 0.0 { -length } else { WIDTH };
            self.cars.push(Car { x, length });
            // Seconds until the next car grow with speed, so fast lanes are sparser.
            let seconds = gen_range(1.0, 2.0) * self.speed.abs() / 100.0;
            self.gap = length + self.speed.abs() * seconds;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    ToShop,
    ToOffice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hop {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Street {
    pub lanes: Vec<Lane>,
    pub x: f32,
    pub row: usize,
    pub leg: Leg,
    /// Set once the intern is back at the office (true) or has been hit (false).
    pub survived: Option<bool>,
}

// Positions and speeds are never NaN, so equality is total.
impl Eq for Street {}

impl Street {
    pub fn new() -> Self {
        let lanes = SPEEDS
            .iter()
            .map(|&(low, high)| Lane {
                speed: gen_range(low, high),
                cars: Vec::new(),
                gap: gen_range(0.0, 200.0),
            })
            .collect();
        let mut street = Self {
            lanes,
            x: OFFICE_DOOR + (DOOR_WIDTH - INTERN_WIDTH) / 2.0,
            row: 0,
            leg: Leg::ToShop,
            survived: None,
        };
        // Let traffic fill the street before the intern steps out.
        for _ in 0..200 {
            street.tick(0.05);
        }
        street
    }

    pub fn hop(&mut self, hop: Hop) {
        if self.survived.is_some() {
            return;
        }
        match hop {
            Hop::Up => self.row = (self.row + 1).min(SHOP_ROW),
            Hop::Down => self.row = self.row.saturating_sub(1),
            Hop::Left => self.x = (self.x - HOP).max(0.0),
            Hop::Right => self.x = (self.x + HOP).min(WIDTH - INTERN_WIDTH),
        }
        let at_door = |door: f32| self.x >= door && self.x + INTERN_WIDTH <= door + DOOR_WIDTH;
        match self.leg {
            Leg::ToShop if self.row == SHOP_ROW && at_door(SHOP_DOOR) => self.leg = Leg::ToOffice,
            Leg::ToOffice if self.row == 0 && at_door(OFFICE_DOOR) => self.survived = Some(true),
            _ => self.check_traffic(),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.survived.is_some() {
            return;
        }
        for lane in &mut self.lanes {
            lane.drive(dt);
        }
        self.check_traffic();
    }

    fn check_traffic(&mut self) {
        let Some(lane) = self.row.checked_sub(1).and_then(|i| self.lanes.get(i)) else {
            return;
        };
        // A little forgiveness at the bumpers.
        let (left, right) = (self.x + 2.0, self.x + INTERN_WIDTH - 2.0);
        if lane
            .cars
            .iter()
            .any(|car| car.x < right && car.x + car.length > left)
        {
            self.survived = Some(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A street with no traffic and none coming.
    fn empty_street() -> Street {
        let mut street = Street::new();
        for lane in &mut street.lanes {
            lane.cars.clear();
            lane.gap = f32::INFINITY;
        }
        street
    }

    fn hops(street: &mut Street, hop: Hop, times: usize) {
        for _ in 0..times {
            street.hop(hop);
        }
    }

    #[test]
    fn lanes_run_both_ways_at_different_speeds() {
        let street = Street::new();
        let speeds: Vec<f32> = street.lanes.iter().map(|l| l.speed).collect();

        assert!(speeds[0] > 0.0 && speeds[1] > 0.0);
        assert!(speeds[2] < 0.0 && speeds[3] < 0.0);
        for (i, speed) in speeds.iter().enumerate() {
            assert!(!speeds[i + 1..].contains(speed));
        }
    }

    #[test]
    fn traffic_starts_flowing_with_gaps_between_cars() {
        let street = Street::new();

        for lane in &street.lanes {
            assert!(!lane.cars.is_empty());
            for pair in lane.cars.windows(2) {
                let (left, right) = if lane.speed > 0.0 {
                    (pair[1], pair[0])
                } else {
                    (pair[0], pair[1])
                };
                assert!(left.x + left.length < right.x, "cars overlap");
            }
        }
    }

    #[test]
    fn faster_lanes_send_cars_less_often() {
        let mut street = Street::new();
        let mut spawns = [0; LANES];
        for _ in 0..4000 {
            for (lane, count) in street.lanes.iter_mut().zip(&mut spawns) {
                let gap = lane.gap;
                lane.drive(0.05);
                if lane.gap > gap {
                    *count += 1;
                }
            }
        }

        // The slowest and fastest lanes have speed ranges that never overlap.
        assert!(spawns[0] > spawns[3], "spawns per lane {spawns:?}");
    }

    #[test]
    fn cars_drive_in_their_lane_direction() {
        let mut street = empty_street();
        street.lanes[0].cars.push(Car {
            x: 100.0,
            length: CAR,
        });
        street.lanes[3].cars.push(Car {
            x: 100.0,
            length: CAR,
        });

        street.tick(0.5);

        assert!(street.lanes[0].cars[0].x > 100.0);
        assert!(street.lanes[3].cars[0].x < 100.0);
    }

    #[test]
    fn the_intern_hops_and_stays_on_the_street() {
        let mut street = empty_street();

        hops(&mut street, Hop::Down, 1);
        hops(&mut street, Hop::Left, 40);
        assert_eq!((street.row, street.x), (0, 0.0));

        hops(&mut street, Hop::Up, 10);
        hops(&mut street, Hop::Right, 60);
        assert_eq!((street.row, street.x), (SHOP_ROW, WIDTH - INTERN_WIDTH));
    }

    #[test]
    fn stepping_into_a_car_is_fatal() {
        let mut street = empty_street();
        let x = street.x;
        street.lanes[0].cars.push(Car {
            x: x - 10.0,
            length: CAR,
        });

        street.hop(Hop::Up);

        assert_eq!(street.survived, Some(false));
    }

    #[test]
    fn a_car_driving_into_the_intern_is_fatal() {
        let mut street = empty_street();
        street.hop(Hop::Up);
        let x = street.x;
        street.lanes[0].cars.push(Car {
            x: x - CAR - 10.0,
            length: CAR,
        });

        street.tick(0.1);
        assert_eq!(street.survived, None);
        for _ in 0..10 {
            street.tick(0.1);
        }
        assert_eq!(street.survived, Some(false));
    }

    #[test]
    fn a_round_trip_through_both_doors_survives() {
        let mut street = empty_street();

        hops(&mut street, Hop::Up, SHOP_ROW);
        assert_eq!(street.leg, Leg::ToShop);
        hops(
            &mut street,
            Hop::Right,
            ((SHOP_DOOR - OFFICE_DOOR) / HOP) as usize,
        );
        assert_eq!(street.leg, Leg::ToOffice);

        hops(&mut street, Hop::Down, SHOP_ROW);
        assert_eq!(street.survived, None);
        hops(
            &mut street,
            Hop::Left,
            ((SHOP_DOOR - OFFICE_DOOR) / HOP) as usize,
        );
        assert_eq!(street.survived, Some(true));
    }

    #[test]
    fn the_office_door_does_not_count_before_the_coffee() {
        let mut street = empty_street();

        hops(&mut street, Hop::Right, 1);
        hops(&mut street, Hop::Left, 1);

        assert_eq!(street.survived, None);
        assert_eq!(street.leg, Leg::ToShop);
    }

    #[test]
    fn nothing_moves_once_the_run_is_over() {
        let mut street = Street::new();
        street.survived = Some(false);
        let before = street.clone();

        street.hop(Hop::Up);
        street.tick(1.0);

        assert_eq!(street, before);
    }
}
