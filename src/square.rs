//16:06 < tomaka> If I understand correctly, you need vertex attributes for both the texture coordinates and the color
//16:06 < tomaka> in your vertex shader you just pass the values through, so that the fragment shader gets them
//16:06 < tomaka> and in the fragment shader, you load the color of the texture and do the blending manually
//16:07 < tomaka> with mix(tex_color.rgb, color.rgb, tex_color.a) I guess
//16:07 < tomaka> or tex_color.rgb * tex_color.a + color.rgb * (1 - tex_color.a) more explicitely

extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::TrianglesList;

use renderable::{Render, Renderable};

#[derive(Copy, Clone)]
struct Vertex {
    vertex_position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn vertices(display: &Display) -> VertexBufferAny {
    implement_vertex!(Vertex, vertex_position, tex_coords);

    VertexBuffer::new(display,
        vec![
            Vertex { vertex_position: [-0.5,  0.5], tex_coords: [0.0,  1.0] },  // left  top
            Vertex { vertex_position: [ 0.5,  0.5], tex_coords: [0.25, 1.0] },  // right top
            Vertex { vertex_position: [ 0.5, -0.5], tex_coords: [0.25, 0.75] }, // right bottom
            Vertex { vertex_position: [-0.5, -0.5], tex_coords: [0.0,  0.75] }, // left  bottom
        ]
    ).into_vertex_buffer_any()
}

pub fn indices(display: &Display) -> IndexBuffer {
    IndexBuffer::new(display, TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}

pub fn instances(display: &Display, stuffs: &Vec<Box<Renderable+Send>>) -> VertexBufferAny {
    implement_vertex!(Render, loc, scale, color);

    let mut data = Vec::new();
    for stuff in stuffs.iter() {
        data.push(stuff.render())
    }

    VertexBuffer::new(display, data).into_vertex_buffer_any()
}
