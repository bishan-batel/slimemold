#version 430
layout(local_size_x = 1024, local_size_y = 1, local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D trailMap;

struct CellData {
    vec2 position;
    float angle;
    vec3 mask;
};

layout(std430, binding=1) buffer Cell {
    CellData cells[];
};

uniform float deltaTime;

uniform vec2 windowSize;
#define WIDTH windowSize.x
#define HEIGHT windowSize.x

const float TAU = 6.283185307179586;
const float PI = TAU / 2;

const float CELL_SPEED = 100.;
const float TURN_SPEED = 100.;
const float SENSOR_ANGLE_SPACING = PI / 6.;
const int SENSOR_SIZE = 2;
const float SENSOR_OFFSET_DIST = 40;

const float POSITION_EPSILON = 1.;

uint rand32Bits(uint seed)
{
    seed ^= 2747636419u;
    seed *= 2654435769u;
    seed ^= seed >> 16;
    seed *= 2654435769u;
    seed ^= seed >> 16;
    seed *= 2654435769u;
    return seed;
}

float randNormalized(uint rand) {
    return rand32Bits(rand) / 4294697295.;
}

float sense(CellData cell, float offset) {
    float angle = cell.angle + offset;
    vec2 dir = vec2(cos(angle), sin(angle));
    ivec2 center = ivec2(cell.position + dir * SENSOR_OFFSET_DIST);

    float sum = 0.;

    for (int x  = -SENSOR_SIZE; x <= SENSOR_SIZE; x++) {
        for (int y  = -SENSOR_SIZE; y <= SENSOR_SIZE; y++) {
            ivec2 pos = center + ivec2(x, y);

            if (pos.x  >= 0 && pos.x < WIDTH && pos.y >= 0 && pos.y < HEIGHT) {
                sum += dot(imageLoad(trailMap, pos).xyz, cell.mask * 2. - 1.);
                //                float dist = length(imageLoad(trailMap, pos).xyz - cell.mask) - 0.5;
            }
        }
    }
    return sum;
    //    return imageLoad(trailMap, ivec2(cell.position) +  ivec2(vec2(cos(cell.angle + offset), sin(cell.angle + offset)))).r;
}

void main() {
    uint id = gl_GlobalInvocationID.x;// get value stored in the image

    CellData cell = cells[id];

    ivec2 ipos = ivec2(cell.position);

    uint randSeed = int((cell.angle + deltaTime) * 1000000. + ipos.x + ipos.y + id.x);

    vec2 direction = vec2(cos(cell.angle), sin(cell.angle));
    cell.position += deltaTime * CELL_SPEED * direction;


    if (cell.position.x >= WIDTH || cell.position.y >= HEIGHT || cell.position.x < 0 || cell.position.y < 0) {
        cell.position = clamp(cells[id].position, vec2(POSITION_EPSILON), windowSize - vec2(POSITION_EPSILON));
        cell.position = windowSize / 2.;
        cell.angle = TAU * randNormalized(randSeed++);
    }

    // steering
    float forward = sense(cell, 0);
    float left = sense(cell, SENSOR_ANGLE_SPACING);
    float right = sense(cell, -SENSOR_ANGLE_SPACING);
    float randomSteer = randNormalized(randSeed++);

    if (forward < left && forward < right) {
        // randomly steer of right & left are around equal
        cell.angle += (randomSteer - .5) * 2 * TURN_SPEED * deltaTime;
    } else if (right > left) {
        // ster right if right sensor is larger
        cell.angle -= randomSteer * TURN_SPEED * deltaTime;
    } else if (left > right) {
        // steer left if left sensor is larger
        cell.angle += randomSteer * TURN_SPEED * deltaTime;
    }


    vec4 trail = imageLoad(trailMap, ipos);

    //    if (trail.a > 1000. && randNormalized(++randSeed) < .5) {
    if (trail.a > 10000.) {
        cell.mask= vec3(0.);

        int i = int(round(randNormalized(randSeed++) * 2));
        cell.mask[i] = 1.;
    }

    cells[id] = cell;

    trail.xyz = max(trail.xyz, cell.mask);
    trail.w += 1.;
    //    trail.xyz = cell.mask;

    imageStore(trailMap, ipos, trail);
}
