#version 450

layout(location=0) in int v;

layout(location=0) out vec2 out_tex_coords;
layout(location=1) out vec3 out_position;
layout(location=2) out vec3 out_normal;

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
    int tex_index =  v & 0x000007FF       ;
    int normal    = (v & 0x00003800) >> 11;
    int z         = (v & 0x000FC000) >> 14;
    int y         = (v & 0x03F00000) >> 20;
    int x         = (v & 0xFC000000) >> 26;

    gl_Position = view_proj * (vec4(ivec3(x, y, z) + position * chunk_size, 1.0));
    out_position = gl_Position.xyz;
    out_tex_coords = tex_coords[tex_index];
    if (normal == 0) {
        out_normal = vec3(0.0, 0.0, 1.0);
    } else if (normal == 1) {
        out_normal = vec3(0.0, 0.0, -1.0);
    } if (normal == 2) {
        out_normal = vec3(1.0, 0.0, 0.0);
    } else if (normal == 3) {
        out_normal = vec3(-1.0, 0.0, 0.0);
    } if (normal == 4) {
        out_normal = vec3(0.0, 1.0, 0.0);
    } else if (normal == 5) {
        out_normal = vec3(0.0, -1.0, 0.0);
    }
}
