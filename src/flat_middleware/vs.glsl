#version 450

layout(location=0) in vec2 in_position;
layout(location=1) in vec2 in_tex_coords;

layout(location=2) in vec2 position;
layout(location=3) in vec2 scale;

layout(location=0) out vec2 out_tex_coords;

void main() {
    out_tex_coords = in_position;

    gl_Position = vec4((in_tex_coords * scale + position) * 2.0 - 1.0, 0.0, 1.0);
}
