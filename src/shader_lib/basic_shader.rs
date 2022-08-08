
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