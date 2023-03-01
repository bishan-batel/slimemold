#version 430 core

uniform sampler2D tex;

in VS_OUTPUT {
    vec2 uv;
} vertex;

out vec4 color;


//const vec3 MID_COLOR = vec3(0.46, 0.87, 1);
const vec3 MID_COLOR = vec3(1, 0.61, 0.46);

const vec3 EMPTY_COLOR = MID_COLOR * .04;

//const vec3 EMPTY_COLOR = .2 * vec3(0.062, 0.019, 0.243);
const vec3 HIGH_COLOR = vec3(1, 0.97, 0.46);

//const vec3 HIGH_COLOR = EMPTY_COLOR;

const float MID_HIGH_MULTIPLIER = 0.5;


vec4 lerp(vec4 a, vec4 b, float t) {
    return a + (b - a) * t;
}

void main() {
    vec4 tex = texture(tex, vertex.uv);
    float w = tex.x;
    //    w = w * exp((w - 1) * 5.);
    w = w * exp((w - 1) * -.9);
    //    w = w * w;


    color = vec4(mix(mix(EMPTY_COLOR, MID_COLOR, w), HIGH_COLOR, (1 / MID_HIGH_MULTIPLIER) * max(w - MID_HIGH_MULTIPLIER, 0)), 1.);
    //    color = vec4(0.2, tex.g, 0.5, 1.);
    color.rgb = mix(color.rgb, MID_COLOR, clamp(1. - exp(-tex.y) - .5, 0., 1.));

    //    color = vec4(tex.x, 0., 0., 1.);
    //        color.yz = tex.yz;

    //    color = mix(color, vec4(1.), max(sin(100. * vertex.uv.x * vertex.uv.y), 0.));
}