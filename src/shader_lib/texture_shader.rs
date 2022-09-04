use crate::rendering::{shader::Shader, buffers2::{BufferLayout, BufferElement, ShaderDataType}, Renderer};

pub fn texture_shader(renderer: &Renderer) -> Shader {
    let buffer_layout = BufferLayout::new(
        vec![
            BufferElement::new("position", ShaderDataType::Float4),
            BufferElement::new("texCoord", ShaderDataType::Float2)
        ]
    );
    let shader = Shader::new(
        renderer, 
        buffer_layout, 
        TEXTURE_VERTEX_SHADER,
        TEXTURE_FRAGMENT_SHADER,
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const TEXTURE_VERTEX_SHADER: &str = "
using namespace metal;

struct ViewProjection {
    float4x4 vp;
};

struct ModelTransform {
    float4x4 transform;
};

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

struct VertexIn {
    float4 position   [[attribute(0)]];
    float2 texCoord   [[attribute(1)]];
};

vertex VertexOut vertex_main(VertexIn vIn [[stage_in]],
                                constant ViewProjection& viewProjection [[buffer(1)]],
                                constant ModelTransform& modelTransform [[buffer(2)]]) {
    VertexOut VertexOut;
    VertexOut.position = viewProjection.vp * modelTransform.transform * float4(vIn.position);
    VertexOut.texCoord = vIn.texCoord;
    return VertexOut;
}
";

#[cfg(target_os = "windows")]
pub const TEXTURE_VERTEX_SHADER: &str = "
struct VSOut {
    float2 texCoord : TEXCOORD;
    float4 position : SV_POSITION;
};

cbuffer CBuf1
{
    matrix transformation;
};

cbuffer CBuf2
{
    matrix projection;
};

VSOut VSMain(float4 pos : Position, float2 tex : TexCoord)
{
    VSOut vso;
    vso.position = mul(mul(float4(pos.x, pos.y, 0.0f, 1.0f), transformation), projection);
    vso.texCoord = tex;
    return vso;
}
";

#[cfg(target_os = "macos")]
pub const TEXTURE_FRAGMENT_SHADER: &str = "
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float2 texCoord;
};

fragment float4 fragment_main(VertexOut interpolated [[stage_in]],
                                texture2d<float> tex2D [[ texture(0) ]])
{
    constexpr sampler textureSampler (mag_filter::linear,
                                      min_filter::linear);
    float4 col = tex2D.sample(textureSampler, interpolated.texCoord);
    return float4(col[0], col[1], col[2], col[3]);
}
";

#[cfg(target_os = "windows")]
pub const TEXTURE_FRAGMENT_SHADER: &str = "
Texture2D tex;

SamplerState splr;

float4 PSMain(float2 tc : TEXCOORD) : SV_Target
{
    return tex.Sample(splr, tc);
}
";