#version 430 core

uniform sampler2D tex;

in VS_OUTPUT {
    vec2 uv;
} vertex;

out vec4 color;


const vec3 MID_COLOR = vec3(0.67, 0.98, 0.85);
const vec3 EMPTY_COLOR = MID_COLOR * .04;

//const vec3 EMPTY_COLOR = .2 * vec3(0.062, 0.019, 0.243);
//const vec3 MID_COLOR = vec3(0.91, 0.18, 0.34);

const vec3 HIGH_COLOR = EMPTY_COLOR;

const float MID_HIGH_MULTIPLIER = 0.5;


vec4 lerp(vec4 a, vec4 b, float t) {
    return a + (b - a) * t;
}

void main() {
    vec4 tex = texture(tex, vertex.uv);
    float w = tex.r;
    //    w = w * exp((w - 1) * 5.);
    //    w = w * exp((w - 1) * 1.);
    //    w = w * w;


    color = vec4(mix(mix(EMPTY_COLOR, MID_COLOR, w), HIGH_COLOR, (1 / MID_HIGH_MULTIPLIER) * max(w - MID_HIGH_MULTIPLIER, 0)), 1.);
    //    color = vec4(tex.x, 0., 0., 1.);
    //    color.yz = tex.yz;

    //    color = mix(color, vec4(1.), max(sin(100. * vertex.uv.x * vertex.uv.y), 0.));
}