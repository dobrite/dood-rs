pub struct HungerComponent {
    value: u16,
}

impl HungerComponent {
    pub fn new(value: u16) -> HungerComponent {
        HungerComponent {
            value: value,
        }
    }
}
