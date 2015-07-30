
use brain::Brain;

pub struct BrainComponent {
    pub brain: Brain,
}

impl BrainComponent {
    pub fn new(brain: Brain) -> Self {
        BrainComponent { brain: brain }
    }
}
