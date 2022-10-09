use crate::buffers2::{BufferElement, BufferLayout, ShaderDataType};
use crate::platform::d3d11::shader::Shader;
use crate::platform::d3d11::Renderer;

pub fn obj_model_shader_layout() -> BufferLayout {
    return BufferLayout::new(vec![
        BufferElement::new("POSITION", ShaderDataType::Float4),
        BufferElement::new("TEXCOORD", ShaderDataType::Float2),
        BufferElement::new("NORMAL", ShaderDataType::Float3),
    ]);
}

pub fn obj_model_shader(renderer: &Renderer) -> Shader {
    let shader = Shader::new(
        renderer,
        obj_model_shader_layout(),
        OBJ_MODEL_VERTEX_SHADER,
        OBJ_MODEL_FRAGMENT_SHADER,
    );
    return shader;
}

#[cfg(target_os = "macos")]
pub const OBJ_MODEL_VERTEX_SHADER: &str = "
using namespace metal;

struct VertexUniforms {
    float4x4 mat;
};

struct VertexIn {
    float2 xy   [[attribute(0)]];
};

vertex float4 vertex_main(constant VertexIn *in [[buffer(0)]],
                                 constant ViewProjection &viewProjection [[buffer(1)]],
                                 constant ModelTransform &modelTransform [[buffer(2)]],
                                 uint vid [[vertex_id]])
{
    return float4(in[vid].xy) * transformation.mat * projection.mat;
}
";

#[cfg(target_os = "windows")]
pub const OBJ_MODEL_VERTEX_SHADER: &str = "
struct VSOut {
    float4 position : SV_POSITION;
    float3 fragPos  : POSITION;
    float2 texcoord : TEXCOORD0;
    float3 normal : NORMAL;
};

struct VertexIn {
    float4 position : POSITION;
    float2 texcoord : TEXCOORD0;
    float3 normal : NORMAL;
};

cbuffer CBuf1
{
    matrix projection;
};

cbuffer CBuf2
{
    matrix transformation;
};

VSOut VSMain(VertexIn vertexIn)
{
    VSOut vso;
    float4x4 worldViewProj = mul(projection, transformation);
    vso.position = mul(worldViewProj, vertexIn.position);
    vso.fragPos = mul(transformation, vertexIn.position);
    vso.texcoord = vertexIn.texcoord;
    vso.normal = vertexIn.normal;
    return vso;
}
";

#[cfg(target_os = "macos")]
pub const OBJ_MODEL_FRAGMENT_SHADER: &str = "
using namespace metal;

fragment float4 fragment_main( constant float4 &color [[buffer(0)]])
{
    return float4(color[0], color[1], color[2], color[3]);
}
";

#[cfg(target_os = "windows")]
pub const OBJ_MODEL_FRAGMENT_SHADER: &str = "
struct VSOut {
    float4 position : SV_POSITION;
    float3 fragPos  : POSITION;
    float2 texcoord : TEXCOORD0;
    float3 normal : NORMAL;
};

cbuffer B1 : register(b0)
{
    float3 ambient;
};

cbuffer B2 : register(b1)
{
    float3 diffuse;
};

cbuffer B3 : register(b2)
{
    float3 lightpos;
};

float4 PSMain(VSOut vsout) : SV_Target
{
    float ambientStrength = 1.2;
    float3 lightColor = float3(1.0, 1.0, 1.0);

    float3 ambientLevel = ambientStrength * lightColor;
    float3 ambientColor = mul(ambient, mul(ambientStrength, lightColor));

    float3 norm = normalize(vsout.normal);
    float3 lightdir = normalize(lightpos - vsout.fragPos);
    float diff = max(dot(norm, lightdir), 0.0);
    float3 diffuseLevel = diff * lightColor;

    float3 result = (ambientLevel + diffuseLevel) * diffuse;

    return float4(result[0], result[1], result[2], 1.0);
}
";
