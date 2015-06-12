#version 140

in vec2 v_tex_coords;

in vec3 vColor;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}
