use std::cmp::Ordering;

use dist::Dist;
use loc::Loc;

pub fn distance(start: Loc, end: Loc) -> Dist {
    return (((end.0 - start.0).pow(2) + (end.1 - start.1).pow(2)) as f32).sqrt()
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
    use super::{
        distance,
        get_closest,
    };

    #[test]
    fn it_returns_distance_for_zero_zero() {
        assert!(distance((0, 0), (0, 0)) == 0.0);
    }

    #[test]
    fn it_returns_distance_for_one_one() {
        assert!(distance((0, 0), (1, 1)) == 2.0f32.sqrt());
    }

    #[test]
    fn it_returns_distance_for_minus_one_minus_one() {
        assert!(distance((0, 0), (-1, -1)) == 2.0f32.sqrt());
    }

    #[test]
    fn it_returns_distance_for_four_two() {
        assert!(distance((1, 2), (3, 4)) == 8.0f32.sqrt());
    }

    #[test]
    fn it_returns_the_closest_location_in_array_some() {
        assert!(get_closest((0, 0), vec![(5, 5), (1, 1), (3, -3)]) == Some((1, 1)))
    }

    #[test]
    fn it_returns_the_closest_location_in_array_none() {
        assert!(get_closest((0, 0), vec![]) == None)
    }
}
