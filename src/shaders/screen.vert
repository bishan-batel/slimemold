#version 430 core

layout(location=0) in vec2 position;
layout(location=1) in vec2 texCoord;

uniform vec2 camPos;
uniform float zoom;

out VS_OUTPUT {
    vec2 uv;
} vertex;

void main() {
    gl_Position = vec4((position + camPos) * zoom, 1., 1.);
    vertex.uv = texCoord;
}