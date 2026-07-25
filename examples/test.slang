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
[numthreads(1, 1, 1)]
void computeMain<T : IQuality>(uint3 threadId: SV_DispatchThreadID)
{
    uint index = threadId.x;
    result[index] = buffer0[index] + buffer1[index] + T::getValue();

    var dynamic = createDynamicObject<IQuality>(dynamicSelector, 0);
    result[index] += dynamic.getValue();
}
