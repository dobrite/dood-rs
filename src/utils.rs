use std::cmp::Ordering;

use dist::Dist;
use loc::Loc;

pub fn distance(start: Loc, end: Loc) -> Dist {
    return (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f32).sqrt()
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
    use super::{
        distance,
        get_closest,
    };

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
        assert!(get_closest(Loc { x: 0, y: 0 },
            vec![&Loc { x: 5, y: 5 }, &Loc { x: 1, y: 1 }, &Loc { x: 3, y: -3 }]) == Some(Loc { x: 1, y: 1 }))
    }

    #[test]
    fn it_returns_the_closest_location_in_array_none() {
        assert!(get_closest(Loc { x: 0, y: 0 }, vec![]) == None)
    }
}
