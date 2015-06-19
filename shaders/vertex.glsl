#version 140

uniform mat4 mvp;

in float scale;

in vec2 vertex_position;
in vec2 loc;
in vec2 tex_coords;

in vec3 color;

out vec3 vColor;
out vec2 v_tex_coords;

void main() {
    gl_Position = mvp * vec4(vertex_position * scale + loc, 0.0, 1.0);
    vColor = color;
    v_tex_coords = tex_coords;
}
