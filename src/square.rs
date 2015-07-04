//16:06 < tomaka> If I understand correctly, you need vertex attributes for both the texture coordinates and the color
//16:06 < tomaka> in your vertex shader you just pass the values through, so that the fragment shader gets them
//16:06 < tomaka> and in the fragment shader, you load the color of the texture and do the blending manually
//16:07 < tomaka> with mix(tex_color.rgb, color.rgb, tex_color.a) I guess
//16:07 < tomaka> or tex_color.rgb * tex_color.a + color.rgb * (1 - tex_color.a) more explicitely
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use loc::Loc;
use loc_map::LocMap;
use pixset::Pixset;
use renderable::Vertex;
use renderable::Renderable;

pub type TexCoords = [[f32; 2]; 4];
pub type FixMe = HashMap<Loc, Rc<RefCell<Renderable>>>;

pub fn vertices(tiles: &Pixset, renderables: &FixMe) -> (Vec<Vertex>, Vec<u8>) {
    let mut vertex_data: Vec<Vertex> = Vec::new();

    for (_, render) in renderables {
        for vertex in render.borrow().render(&tiles) {
            vertex_data.push(vertex)
        }
    }

    let len = vertex_data.len();
    (vertex_data, indices(len))
}

pub fn indices(length: usize) -> Vec<u8> {
    (0..(length / 4)).into_iter().flat_map(|i|
        vec![0, 1, 2, 0, 2, 3].into_iter().map(|j| (j + i * 4) as u8).collect::<Vec<u8>>()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::indices;

    #[test]
    fn it_returns_indices_for_len_four() {
        assert!(indices(4) == [0u8, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn it_returns_indices_for_len_eight() {
        assert!(indices(8) == [0u8, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }
}
