use crate::buffers::{BufferElement, BufferLayout, ShaderDataType};
use crate::shaders::Shader;
use crate::Renderer;

pub fn basic_shader_layout() -> BufferLayout {
    return BufferLayout::new(vec![
        BufferElement::new("POSITION", ShaderDataType::Float3),
        BufferElement::new("COLOR", ShaderDataType::Float4),
    ]);
}

pub fn basic_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer,
        basic_shader_layout(),
        BASIC_VERTEX_SHADER,
        BASIC_FRAGMENT_SHADER,
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const BASIC_VERTEX_SHADER: &str = "
using namespace metal;

struct VertexIn {
    float3 position [[ attribute(0) ]];
    float4 color [[ attribute(1) ]];
};

struct RasterizerData {
    float4 position [[ position ]];
    float4 color;
};

vertex RasterizerData vertex_main(const VertexIn vIn [[ stage_in ]]) {
    RasterizerData rd;

    rd.position = float4(vIn.position, 1);
    rd.color = vIn.color;

    return rd;
}
";

#[cfg(target_os = "windows")]
pub const BASIC_VERTEX_SHADER: &str = "
struct VSIn {
    float3 position : POSITION;
    float4 color : COLOR;
};

struct VSOut {
    float4 position : SV_POSITION;
    float4 color : COLOR;
};

VSOut VSMain(VSIn input)
{
    VSOut vso;
    vso.position = float4(input.position.x, input.position.y, 0.0f, 1.0f);
    vso.color = input.color;
    return vso;
}";

#[cfg(target_os = "macos")]
pub const BASIC_FRAGMENT_SHADER: &str = "
using namespace metal;

struct RasterizerData {
    float4 position [[ position ]];
    float4 color;
};

fragment half4 fragment_main(RasterizerData rd [[ stage_in ]]) {
    float4 color = rd.color;
    return half4(color.r, color.g, color.b, color.a);
}
";

#[cfg(target_os = "windows")]
pub const BASIC_FRAGMENT_SHADER: &str = "
struct PixelInput {
    float4 position : SV_POSITION;
    float4 color : COLOR;
};

float4 PSMain(PixelInput pixelInput) : SV_Target
{
    float4 color = pixelInput.color;
    return float4(color[0], color[1], color[2], color[3]);
}
";
