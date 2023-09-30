#version 150

in vec3 position;
in vec3 normal;
out vec3 v_normal;
uniform mat4 m;

void main() {
    v_normal = transpose(inverse(mat3(m))) * normal;
    gl_Position = m * vec4(position, 1.0);
}