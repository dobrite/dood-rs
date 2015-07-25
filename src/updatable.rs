
use chunks::Chunks;

pub trait Updatable {
    fn update(&mut self, chunks: &Chunks);
}
