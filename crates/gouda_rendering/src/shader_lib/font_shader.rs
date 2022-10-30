use crate::buffers::{BufferElement, BufferLayout, ShaderDataType};
use crate::shaders::{Shader, ShaderUniformSpec};
use crate::Renderer;

pub fn font_shader_layout() -> BufferLayout {
    return BufferLayout::new(vec![
        BufferElement::new("POSITION", ShaderDataType::Float4),
        BufferElement::new("TEXCOORD", ShaderDataType::Float2),
    ]);
}

pub fn font_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer,
        font_shader_layout(),
        FONT_VERTEX_SHADER,
        FONT_FRAGMENT_SHADER,
        ShaderUniformSpec { uniforms: vec![] },
        ShaderUniformSpec { uniforms: vec![] },
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const FONT_VERTEX_SHADER: &str = "
using namespace metal;

struct VertexIn {
    float4 position   [[attribute(0)]];
    float2 texCoord   [[attribute(1)]];
};

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]])
{
    VertexOut VertexOut;
    VertexOut.position = float4(in.position[0], in.position[1], 0.0, 1.0);
    VertexOut.texCoord = in.texCoord;
    return VertexOut;
}
";

#[cfg(target_os = "windows")]
pub const FONT_VERTEX_SHADER: &str = "
struct VSOut {
    float4 position : SV_POSITION;
    float2 texCoord : TEXCOORD;
};

VSOut VSMain(float4 position : Position, float2 tex : TexCoord)
{
    VSOut vso;
    vso.position = float4(position.x, position.y, 0.0f, 1.0f);
    vso.texCoord = tex;
    return vso;
}
";

#[cfg(target_os = "macos")]
pub const FONT_FRAGMENT_SHADER: &str = "
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

fragment float4 fragment_main(VertexOut interpolated [[stage_in]],
                                constant float4 *color [[buffer(0)]],
                                texture2d<float> tex2D [[ texture(0) ]])
{
    constexpr sampler textureSampler (filter::linear,
                                    mip_filter::linear);
    float alpha = tex2D.sample(textureSampler, interpolated.texCoord)[2];
    return float4(color[0][0], color[0][1], color[0][2], alpha);
}
";

#[cfg(target_os = "windows")]
pub const FONT_FRAGMENT_SHADER: &str = "
cbuffer CBuf
{
    float4 color;
};

struct VSOut {
    float4 position : SV_POSITION;
    float2 texCoord : TEXCOORD;
};

Texture2D tex;

SamplerState splr;

float4 PSMain(VSOut vOut) : SV_Target
{
    float alpha = tex.Sample(splr, vOut.texCoord)[2];
    return float4(color[0], color[1], color[2], alpha);
}
";
