#version 430
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D trailMap;
uniform float deltaTime;

const float diffuseSpeed = 10.0;
const float evaporationSpeed = 0.2;

const int WIDTH = 1920;
const int HEIGHT = 1080;

uint hash(uint state)
{
    state ^= 2747636419u;
    state *= 2654435769u;
    state ^= state >> 16;
    state *= 2654435769u;
    state ^= state >> 16;
    state *= 2654435769u;
    return state;
}

float lerp(float a, float b, float t) {
    return a + (b - a) * t;
}

void main() {
    // get position to read/write data from
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);// get value stored in the image


    vec4 val = imageLoad(trailMap, pixel);// store new value in image

    // diffusion
    float sum = 0.;

    for (int offsetX = -1; offsetX <= 1; offsetX++) {
        for (int offsetY = -1; offsetY <= 1; offsetY++) {
            int sampleX = pixel.x + offsetX;
            int sampleY = pixel.y + offsetY;

            if (sampleX >= 0 && sampleX < WIDTH && sampleY >= 0 && sampleY < HEIGHT) {
                sum += imageLoad(trailMap, ivec2(sampleX, sampleY)).x;
            }
        }
    }

    float avg = sum / 9.;

    val.r = mix(val.r, avg, diffuseSpeed * deltaTime);
    //    val.g = lerp(val.g, avg.g, diffuseSpeed * deltaTime);
    //    val.b = lerp(val.b, avg.b, diffuseSpeed * deltaTime);


    // evaporation
    val.r = max(0., val.r - evaporationSpeed * deltaTime);

    imageStore(trailMap, pixel, val);
}