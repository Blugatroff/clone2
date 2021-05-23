#version 450

layout(location=0) in int v;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 view_proj;
    vec4 camera_pos;
    ivec4 num_lights;
};

layout(set=1, binding=0)
uniform ChunkUniforms {
    ivec3 position;
    float chunk_size;
};

void main() {
    int z = (v & 0x000FC000) >> 14;
    int y = (v & 0x03F00000) >> 20;
    int x = (v & 0xFC000000) >> 26;

    gl_Position = view_proj * (vec4(ivec3(x, y, z) + position * chunk_size, 1.0));
}
