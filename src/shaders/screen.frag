#version 430 core

uniform sampler2D tex;

in VS_OUTPUT {
    vec2 uv;
} vertex;

out vec4 color;


//const vec3 MID_COLOR = vec3(0.46, 0.87, 1);
const vec3 EMPTY_COLOR = vec3(1, 0.61, 0.46) * .04;

#define S1_MASK vec3(1, 0.47, 0.35)
#define S2_MASK vec3(0.55, 0.41, 1)
#define S3_MASK vec3(0.41, 1, 0.45)

//const vec3 HIGH_COLOR = EMPTY_COLOR;

const float MID_HIGH_MULTIPLIER = 0.5;


vec4 lerp(vec4 a, vec4 b, float t) {
    return a + (b - a) * t;
}


void main() {
    vec4 tex = texture(tex, vertex.uv);
    //        w = w * exp((w - 1) * -1.);
    //        w = w * exp((w - 1) * -.9);
    //    tex.rgb *= exp((tex.rgb - vec3(1.) * -.9));
    //    w = w * w;
    tex.rgb = tex.rgb * tex.rgb;

    //    color = vec4(mix(mix(EMPTY_COLOR, MID_COLOR, w), HIGH_COLOR, (1 / MID_HIGH_MULTIPLIER) * max(w - MID_HIGH_MULTIPLIER, 0)), 1.);

    vec3 s1 = mix(EMPTY_COLOR, S1_MASK, tex.x);
    vec3 s2 = mix(EMPTY_COLOR, S2_MASK, tex.y);
    vec3 s3 = mix(EMPTY_COLOR, S3_MASK, tex.z);

    color.rgb = max(s1, max(s2, s3));
    //    color.rgb = mix(color.rgb, EMPTY_COLOR, clamp(1. - exp(-tex.a) - .5, 0., 1.));

    //    color = vec4(0., 0., 0., 1.);
    //    color.rgb = tex.rgb;
    //    color = tex;

    //    color = mix(color, vec4(1.), max(sin(100. * vertex.uv.x * vertex.uv.y), 0.));
}