use crate::buffers::{BufferElement, BufferLayout, ShaderDataType};
use crate::shaders::{Shader, ShaderUniformSpec};
use crate::Renderer;

pub fn imgui_shader_layout() -> BufferLayout {
    return BufferLayout::new(vec![
        BufferElement::new("pos", ShaderDataType::Float2),
        BufferElement::new("uv", ShaderDataType::Float2),
        BufferElement::new("col", ShaderDataType::Float4),
    ]);
}

pub fn imgui_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer,
        imgui_shader_layout(),
        IMGUI_VERTEX_SHADER,
        IMGUI_FRAGMENT_SHADER,
        ShaderUniformSpec { uniforms: vec![] },
        ShaderUniformSpec { uniforms: vec![] },
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const IMGUI_VERTEX_SHADER: &str = "
using namespace metal;

struct ModelTransform {
    float4x4 transform;
};

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

struct VertexIn {
    float2 position [[attribute(0)]];
};

vertex VertexOut vertex_main(const VertexIn in [[stage_in]],
                                 constant ModelTransform &transform [[buffer(1)]])
{
    VertexOut VertexOut;
    VertexOut.position = transform.transform * float4(in.position, 0.0, 1.0);
    VertexOut.texCoord = float2((in.position.x) / 2.0, 1.0 - (in.position.y / 2.0));
    return VertexOut;
}
";

#[cfg(target_os = "windows")]
pub const IMGUI_VERTEX_SHADER: &str = "
struct VertexIn {
    float2 position : pos;
    float2 uv : uv;
    float4 color : col;
};

struct VertexOut {
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
    float4 color : COLOR;
};

cbuffer Matrix {
    float4x4 _matrix;
};

VertexOut VSMain(VertexIn vIn)
{
    VertexOut output;
    output.position = mul(_matrix, float4(vIn.position.x, vIn.position.y, 0.0, 1.0));
    output.uv = vIn.uv;
    output.color = vIn.color;
    return output;
}
";

#[cfg(target_os = "macos")]
pub const IMGUI_FRAGMENT_SHADER: &str = "
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

float square(float val)
{
    return val * val;
}

float calc_rounded_corners(float2 coord, float radius, float width, float height)
{
    if (radius <= 0.0) {
        return 1.0;
    }
    float cornerSmooth = 0.002;

    float2 pixelPos = coord * float2(width, height);
    float2 minCorner = float2(radius, radius);
    float2 maxCorner = float2(width - radius, height - radius);

    float2 cornerPoint = clamp(pixelPos, minCorner, maxCorner);
    float lowerBound = square(radius - cornerSmooth);
    float upperBound = square(radius + cornerSmooth);
    return smoothstep(upperBound, lowerBound, distance_squared(pixelPos, cornerPoint));
}

fragment float4 fragment_main(VertexOut interpolated [[stage_in]], constant float4 &color [[buffer(0)]], constant float2 &dimensions [[buffer(1)]], constant float &radius [[buffer(2)]])
{
    float alpha = calc_rounded_corners(interpolated.texCoord, radius, dimensions.x, dimensions.y) * color[3];
    return float4(color[0], color[1], color[2], alpha);
}
";

#[cfg(target_os = "windows")]
pub const IMGUI_FRAGMENT_SHADER: &str = "
Texture2D tex;
SamplerState splr;

struct VertexOut {
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
    float4 color : COLOR;
};

float4 PSMain(VertexOut vout) : SV_TARGET
{
    return vout.color * tex.Sample(splr, vout.uv);
}
";
