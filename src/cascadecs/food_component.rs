
use food::Food;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FoodComponent {
    pub kind: Food,
    pub noms: f32,
}

impl FoodComponent {
    pub fn new(kind: Food, noms: f32) -> Self {
        FoodComponent { kind: kind, noms: noms }
    }
}
