#version 450
layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;
layout(set = 0, binding = 0) uniform sampler2D tex;

void main() {
    f_color = texture(tex, tex_coords);
    f_color = vec4(
        f_color[0] * 0.299 + f_color[1] * 0.587 + f_color[2] *0.114,
        f_color[0] * 0.299 + f_color[1] * 0.587 + f_color[2] *0.114,
        f_color[0] * 0.299 + f_color[1] * 0.587 + f_color[2] *0.114,
        1.0);

}