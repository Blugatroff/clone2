#version 450

layout(location=0) in int v;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 view_proj;
    vec4 camera_pos;
    ivec4 num_lights;
};

layout(set=2, binding=0)
uniform ChunkUniforms {
    ivec3 position;
    float chunk_size;
};

layout(set=2, binding=1) buffer TexCoords {
    vec2 tex_coords[];
};

void main() {
    int z = (v & 0xFC000) >> 14;
    int y = (v & 0x3F00000) >> 20;
    int x = (v & 0xFC000000) >> 26;

    gl_Position = view_proj * (vec4(ivec3(x, y, z) + position * chunk_size, 1.0));
}
