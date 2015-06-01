#version 140

uniform mat4 view_transform;

in float scale;

in vec2 vertex_position;
in vec2 loc;
in vec3 color;
in vec2 tex_coords;

out vec3 vColor;
out vec2 v_tex_coords;

void main() {
    gl_Position = view_transform * vec4(loc + scale * vertex_position, 0.0, 1.0);
    vColor = color;
    v_tex_coords = tex_coords;
}
