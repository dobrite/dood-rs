//16:06 < tomaka> If I understand correctly, you need vertex attributes for both the texture coordinates and the color
//16:06 < tomaka> in your vertex shader you just pass the values through, so that the fragment shader gets them
//16:06 < tomaka> and in the fragment shader, you load the color of the texture and do the blending manually
//16:07 < tomaka> with mix(tex_color.rgb, color.rgb, tex_color.a) I guess
//16:07 < tomaka> or tex_color.rgb * tex_color.a + color.rgb * (1 - tex_color.a) more explicitely
extern crate glium;

use std::sync::{
    Arc,
    Mutex,
};

use std::collections::HashMap;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::TrianglesList;

use pixset::{
    Pix,
    Pixset,
};

use renderable::{
    Renderable,
    Vertex,
};

pub type TexCoords = [[f32; 2]; 4];

pub fn vertices(display: &Display, tiles: &Pixset, renderables: &Vec<Arc<Mutex<Box<Renderable+Send+Sync>>>>)
    -> (VertexBufferAny, IndexBuffer) {
        //let data: Vec<Vertex> = renderables.iter().flat_map(|r| r.lock().unwrap().render(tiles)).collect();
        //let len = data.len();

        let v: Vec<Vertex> = vec![];
        return (
            VertexBuffer::new(display, v).into_vertex_buffer_any(),
            IndexBuffer::new(display, TrianglesList(indices(0))),
        )
}

fn indices(length: usize) -> Vec<u16> {
    return (0..(length / 4)).into_iter().flat_map(|i|
        vec![0, 1, 2, 0, 2, 3].into_iter().map(|j| (j + i * 4) as u16).collect::<Vec<u16>>()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::indices;

    #[test]
    fn it_returns_indices_for_len_four() {
        assert!(indices(4) == [0u16, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn it_returns_indices_for_len_eight() {
        assert!(indices(8) == [0u16, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }
}
