use loc::Loc;

pub struct PositionComponent {
    loc: Loc,
}

impl PositionComponent{
    pub fn new(loc: Loc) -> PositionComponent {
        PositionComponent {
            loc: loc,
        }
    }
}
