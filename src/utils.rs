use dist::Dist;
use loc::Loc;

pub fn distance(start: Loc, end: Loc) -> f32 {
    return (((end.0 - start.0).pow(2) + (end.1 - start.1).pow(2)) as f32).sqrt()
}

#[cfg(test)]
mod tests {
    use super::distance;

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
}
