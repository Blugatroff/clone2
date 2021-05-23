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
    uint lighting_enabled;
    float time;
};

struct RealLight {
    mat4 proj;
    vec4 pos;
    vec4 color;
    float def;
    float constant;
    float linear;
    float quadratic;
    uint enabled;
};

layout(set=0, binding=1) buffer real_lights_buffer {
    RealLight real_lights[];
};

layout(set=0, binding=3) uniform texture2DArray t_shadow;
layout(set=0, binding=4) uniform samplerShadow s_shadow;

float fetch_shadow(int light_id, vec4 homogeneous_coords) {
    if (homogeneous_coords.w <= 0.0) {
        return 1.0;
    }
    // compensate for the Y-flip difference between the NDC and texture coordinates
    vec2 flip_correction = vec2(0.5, -0.5);
    // compute texture coordinates for shadow lookup
    vec4 light_local = vec4(
        homogeneous_coords.xy * flip_correction + vec2(0.5, 0.5),
        light_id,
        homogeneous_coords.z / homogeneous_coords.w
    );
    // do the lookup, using HW PCF and comparison
    return texture(sampler2DArrayShadow(t_shadow, s_shadow), light_local);
}

layout(set=1, binding=0) uniform texture2D t_diffuse;
layout(set=1, binding=1) uniform sampler s_diffuse;

void main() {
    vec4 object_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    if (lighting_enabled != 0) {
        vec4 color = ambient_color;
        vec3 view_dir = normalize(camera_pos.xyz - in_position.xyz);
        for (int i = 0; i < num_lights.x; ++i) {
            RealLight light = real_lights[i];
            if (light.enabled == 0) {
                continue;
            }
            // project into the light space
            vec4 light_view_space_pos = light.proj * vec4(in_position, 1.0);
            float correction = 1.0 / light_view_space_pos.w;
            light_view_space_pos = light_view_space_pos * correction;
            float shadow = light.def;
            if (
            !(
                light_view_space_pos.x > 1.0 ||
                light_view_space_pos.y > 1.0 ||
                light_view_space_pos.x < -1.0 ||
                light_view_space_pos.y < -1.0
            )
            ) {
                shadow = fetch_shadow(i, light_view_space_pos);
            }

            // diffuse lighting
            vec3 light_dir = normalize(light.pos.xyz - in_position.xyz);

            float diffuse = max(0.0, dot(in_normal, light_dir));

            // specular lighting
            vec3 reflect_dir = normalize(reflect(-light_dir, in_normal));

            float specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32);

            float d = distance(light.pos.xyz, in_position.xyz);
            float attenuation = 1.0 / (light.constant + light.linear * d + light.quadratic * (d * d));

            color += shadow * (diffuse + specular) * vec4(light.color.xyz, 1.0) * attenuation * light.color.w;
            //color = shadow * vec4(1.0, 1.0, 1.0, 1.0);
            //color += vec4(view_dir, 1.0);
        }
        f_color = color * object_color;
        //f_color = color;
    } else {
        f_color = object_color * ambient_color;
    }
}
