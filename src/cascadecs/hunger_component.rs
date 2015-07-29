
pub struct HungerComponent {
    pub value: u16,
    pub rate: u8,
}

impl HungerComponent {
    // TODO use Self
    pub fn new(value: u16, rate: u8) -> HungerComponent {
        HungerComponent { value: value, rate: rate }
    }
}
