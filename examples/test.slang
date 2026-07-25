StructuredBuffer<float> buffer0;
StructuredBuffer<float> buffer1;
RWStructuredBuffer<float> result;

interface IQuality
{
    static float getValue();
};
struct HighQuality : IQuality
{
    static float getValue()
    {
        return 100.0;
    }
}
struct LowQuality : IQuality
{
    static float getValue()
    {
        return 1.0;
    }
}

uniform uint dynamicSelector;

[shader("compute")]
[numthreads(4, 2, 1)]
void computeMain<T : IQuality>(uint3 threadId: SV_DispatchThreadID)
{
    uint index = threadId.x;
    result[index] = buffer0[index] + buffer1[index] + T::getValue();

    var dynamic = createDynamicObject<IQuality>(dynamicSelector, 0);
    result[index] += dynamic.getValue();
}

[shader("vertex")]
float4 vertexMain<T : IQuality>(uint vertexID: SV_VertexID) : SV_Position
{
    return float4(vertexID, 0.0, 0.0, 1.0);
}
