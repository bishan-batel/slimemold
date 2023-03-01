#version 430
layout(local_size_x = 1024, local_size_y = 1, local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D trailMap;

struct CellData {
    vec2 position;
    float angle;
};

layout(std430, binding=1) buffer Cell {
    CellData cells[];
};

uniform float deltaTime;

const int WIDTH = 1920;
const int HEIGHT = 1080;

const float TAU = 6.283185307179586;
const float PI = TAU / 2;

const float CELL_SPEED = 40.;
const float SENSOR_ANGLE_SPACING = PI / 6.;
const float TURN_SPEED = 20.;
const int SENSOR_SIZE = 2;
const float SENSOR_OFFSET_DIST = 4;

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

float rand01(uint rand) {
    return hash(rand) / 4294697295.;
}

float lerp(float a, float b, float t) {
    return a + (b - a) * t;
}

float sense(CellData cell, float offset) {
    float angle = cell.angle + offset;
    vec2 dir = vec2(cos(angle), sin(angle));
    ivec2 center = ivec2(cell.position + dir * SENSOR_OFFSET_DIST);

    float sum = 0;

    for (int x  = -SENSOR_SIZE; x <= SENSOR_SIZE; x++) {
        for (int y  = -SENSOR_SIZE; y <= SENSOR_SIZE; y++) {
            ivec2 pos = center + ivec2(x, y);

            if (pos.x  >= 0 && pos.x < WIDTH && pos.y >= 0 && pos.y < HEIGHT) {
                sum += imageLoad(trailMap, pos).x;
            }
        }
    }
    return sum;
    //    return imageLoad(trailMap, ivec2(cell.position) +  ivec2(vec2(cos(cell.angle + offset), sin(cell.angle + offset)))).r;
}

void main() {
    uint id = gl_GlobalInvocationID.x;// get value stored in the image

    //    vec2 pos = position[id];
    //    ivec2 pos = ivec2(int(pos.x), int(pos.y));
    CellData cell = cells[id];

    ivec2 ipos = ivec2(cell.position);

    uint randIndex = int((cell.angle + deltaTime) * 1000. + ipos.x + ipos.y + id.x);

    vec2 direction = vec2(cos(cell.angle), sin(cell.angle));
    cell.position += deltaTime * CELL_SPEED * direction;

    if (cell.position.x >= WIDTH || cell.position.y >= HEIGHT || cell.position.x < 0 || cell.position.y < 0) {
        cell.position = cells[id].position;
        cell.angle = TAU * rand01(randIndex++);
    }

    // steering
    float weightForward = sense(cell, 0);
    float weightLeft = sense(cell, SENSOR_ANGLE_SPACING);
    float weightRight = sense(cell, -SENSOR_ANGLE_SPACING);

    float steerRandom = rand01(randIndex);

    if (weightForward > weightLeft && weightForward > weightRight) {
        cell.angle += 0.;
    } else if (weightForward < weightLeft && weightForward < weightRight) {
        cell.angle += (steerRandom - .5) * 2 * TURN_SPEED * deltaTime;
    } else if (weightRight > weightLeft) {
        cell.angle -= steerRandom * TURN_SPEED * deltaTime;
    } else if (weightLeft > weightRight) {
        cell.angle += steerRandom * TURN_SPEED * deltaTime;
    }

    cells[id] = cell;

    //        imageStore(trailMap, ipos, vec4(1., imageLoad(trailMap, ipos).yzw));

    vec4 trail = imageLoad(trailMap, ipos);
    trail.x = 1.;
    imageStore(trailMap, ipos, trail);
}
