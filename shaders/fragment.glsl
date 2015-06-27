#version 140

in vec2 v_tex_coords;

in vec3 vColor;

out vec4 color;

uniform sampler2D tex;

void main() {
    vec4 tex_color = texture(tex, v_tex_coords);
    color = mix(vec4(0.0, 0.0, 0.0, 255.0), vec4(vColor, 255.0) * tex_color, tex_color.a);
}
