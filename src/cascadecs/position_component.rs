use loc::Loc;

pub struct PositionComponent {
    pub loc: Loc,
}

impl PositionComponent {
    pub fn new(loc: Loc) -> PositionComponent {
        PositionComponent {
            loc: loc,
        }
    }
}
