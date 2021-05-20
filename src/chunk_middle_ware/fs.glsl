#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 in_position;
layout(location=2) in vec3 in_normal;

layout(location=0) out vec4 f_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 view_proj;
    vec4 camera_pos;
    ivec4 num_lights;
    vec4 ambient_color;
    int lighting_enabled;
};

layout(set=1, binding=0) uniform texture2D t_diffuse;
layout(set=1, binding=1) uniform sampler s_diffuse;

void main() {
    vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    f_color = object_color * ambient_color;
}
