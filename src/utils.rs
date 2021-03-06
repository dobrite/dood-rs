
use std::cmp::Ordering;

use dist::Dist;
use loc::Loc;

pub fn distance(start: Loc, end: Loc) -> Dist {
    (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f32).sqrt()
}

pub fn get_closest(start: Loc, collection: Vec<&Loc>) -> Option<Loc> {
    let mut closest = collection.iter()
        .map(|&to| (distance(start, *to), *to))
        .collect::<Vec<_>>();

    closest.sort_by(|&(d1, _), &(d2, _)| {
        d1.partial_cmp(&d2).unwrap_or(Ordering::Equal)
    });

    match closest.first() {
        Some(&(_, loc)) => Some(loc),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use loc::Loc;
    use super::{distance, get_closest};

    #[test]
    fn it_returns_distance_for_zero_zero() {
        assert!(distance(Loc { x: 0, y: 0 }, Loc { x: 0, y: 0 }) == 0.0);
    }

    #[test]
    fn it_returns_distance_for_one_one() {
        assert!(distance(Loc { x: 0, y: 0 }, Loc { x: 1, y: 1 }) == 2.0f32.sqrt());
    }

    #[test]
    fn it_returns_distance_for_minus_one_minus_one() {
        assert!(distance(Loc { x: 0, y: 0 }, Loc { x: -1, y: -1 }) == 2.0f32.sqrt());
    }

    #[test]
    fn it_returns_distance_for_four_two() {
        assert!(distance(Loc { x: 1, y: 2 }, Loc { x: 3, y: 4 }) == 8.0f32.sqrt());
    }

    #[test]
    fn it_returns_the_closest_location_in_array_some() {
        let l1 = Loc { x: 5, y: 5 };
        let l2 = Loc { x: 1, y: 1 };
        let l3 = Loc { x: 3, y: -3 };
        let vec = vec![&l1, &l2, &l3];
        assert!(get_closest(Loc { x: 0, y: 0 }, vec) == Some(Loc { x: 1, y: 1 }))
    }

    #[test]
    fn it_returns_the_closest_location_in_array_none() {
        assert!(get_closest(Loc { x: 0, y: 0 }, vec![]) == None)
    }
}
