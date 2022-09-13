
#shader vertex
#version 440 core


layout(location = 0) in vec4 position;
layout(location = 1) in vec3 normal;

out vec3 itpl_position;
out vec3 itpl_normal;

uniform mat4 u_mpv_mat;


void main() {
    gl_Position = u_mpv_mat * position;
    itpl_position = vec3(gl_Position);
    itpl_normal = normal;
    // gl_Position.w = -gl_Position.z;
}



#shader fragment
#version 440 core


layout(location = 0) out vec4 color;

in vec3 itpl_position;
in vec3 itpl_normal;

uniform vec4 u_color;
uniform vec3 u_light_pos;
uniform float u_diffuse_coef;
uniform float u_ambient_light;

void main() {
    vec3 light_dir = normalize(u_light_pos - itpl_position);
    float diffuse = abs(dot(itpl_normal, light_dir));
    color = u_color * (diffuse * u_diffuse_coef + u_ambient_light);
    color.w = u_color.w;
}
