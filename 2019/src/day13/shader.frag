#version 140

out vec4 color;

uniform vec2 window_size;
uniform sampler2D tex;

void main()
{
    vec2 uv = gl_FragCoord.yx / window_size.yx;
    uv.x = 1. - uv.x;
    color = vec4(texture2D(tex, uv).rgb, 1.);
}