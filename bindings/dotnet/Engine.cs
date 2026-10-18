using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

namespace SevenMLabs.Astrology;

public sealed class EngineException : ArgumentException
{
    public string Code { get; }
    public string ResultJson { get; }
    internal EngineException(string code, string message, string result) : base(message)
    { Code = code; ResultJson = result; }
}

/// <summary>The original calculated payload and its separately prepared context.</summary>
public sealed record CalculationWithContext(JsonElement Result, JsonElement Context);

public static class Engine
{
    private static readonly JsonSerializerOptions ContextSerializerOptions = new() { MaxDepth = 128 };
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern uint astro_abi_version();
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_calculate_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_compress_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_expand_context_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_resolve_birth_time_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_calculate_from_local_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_render_chart_svg_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr astro_capabilities_json(IntPtr input);
    [DllImport("astro_engine", CallingConvention = CallingConvention.Cdecl)]
    private static extern void astro_free_string(IntPtr output);

    /// <summary>Return the complete JSON envelope, including validation errors.</summary>
    public static string CalculateJson(string request)
        => NativeJson(request, astro_calculate_json, 4 * 1024 * 1024);

    private static string NativeJson(string request, Func<IntPtr, IntPtr> operation, int maxBytes)
    {
        ArgumentNullException.ThrowIfNull(request);
        if (request.Contains('\0')) throw new ArgumentException("Input cannot contain a literal NUL", nameof(request));
        if (Encoding.UTF8.GetByteCount(request) > maxBytes) throw new ArgumentException($"Input exceeds {maxBytes} bytes", nameof(request));
        if (astro_abi_version() != 1) throw new InvalidOperationException("Unsupported astrology ABI version");
        var input = Marshal.StringToCoTaskMemUTF8(request);
        try
        {
            var output = operation(input);
            if (output == IntPtr.Zero) throw new OutOfMemoryException("Engine returned a null result");
            try { return Marshal.PtrToStringUTF8(output) ?? throw new InvalidOperationException("Empty engine result"); }
            finally { astro_free_string(output); }
        }
        finally { Marshal.FreeCoTaskMem(input); }
    }

    /// <summary>Calculate from a serializable request. Return value owns its JSON data.</summary>
    public static JsonElement Calculate<T>(T request)
        => CheckedResult(CalculateJson(JsonSerializer.Serialize(request)));

    private static JsonElement CheckedResult(string output)
    {
        using var doc = JsonDocument.Parse(output, new JsonDocumentOptions { MaxDepth = 128 });
        var errors = doc.RootElement.GetProperty("errors");
        if (errors.GetArrayLength() > 0)
        {
            var error = errors[0];
            throw new EngineException(error.GetProperty("code").GetString()!, error.GetProperty("message").GetString()!, output);
        }
        return doc.RootElement.Clone();
    }

    /// <summary>Prepare context without reparsing raw number literals; errors remain in the JSON envelope.</summary>
    public static string CompressPayloadJson(string payloadJson, string optionsJson = "{}")
    {
        ArgumentNullException.ThrowIfNull(payloadJson);
        ArgumentNullException.ThrowIfNull(optionsJson);
        return NativeJson("{\"payload\":" + payloadJson + ",\"options\":" + optionsJson + "}",
            astro_compress_json, 256 * 1024 * 1024);
    }

    /// <summary>Create an astro-context/1 envelope in the shared core. Does not mutate the payload.</summary>
    public static JsonElement CompressPayload<T>(T payload, object? options = null)
        => CheckedResult(CompressPayloadJson(JsonSerializer.Serialize(payload, ContextSerializerOptions),
            options is null ? "{}" : JsonSerializer.Serialize(options, ContextSerializerOptions)));

    /// <summary>Decode a context envelope to its retained engine envelope; omitted facts cannot be restored.</summary>
    public static string ExpandContextJson(string contextJson)
        => NativeJson(contextJson, astro_expand_context_json, 256 * 1024 * 1024);

    public static JsonElement ExpandContext<T>(T context)
        => CheckedResult(ExpandContextJson(JsonSerializer.Serialize(context, ContextSerializerOptions)));

    /// <summary>Calculate synchronously and return both the original result and its prepared context.</summary>
    public static CalculationWithContext CalculateWithContext<T>(T request, object? options = null)
    {
        var result = Calculate(request);
        return new CalculationWithContext(result, CompressPayload(result, options));
    }

    public static string ResolveBirthTimeJson(string request)
        => NativeJson(request, astro_resolve_birth_time_json, 16 * 1024);
    public static JsonElement ResolveBirthTime<T>(T request)
        => CheckedResult(ResolveBirthTimeJson(JsonSerializer.Serialize(request))).GetProperty("data").Clone();
    public static string CalculateFromLocalJson(string request)
        => NativeJson(request, astro_calculate_from_local_json, 4 * 1024 * 1024);
    public static JsonElement CalculateFromLocal<T>(T request)
        => CheckedResult(CalculateFromLocalJson(JsonSerializer.Serialize(request)));
    public static string RenderChartSvgJson(string payloadJson, string optionsJson = "{}")
    {
        ArgumentNullException.ThrowIfNull(payloadJson);
        ArgumentNullException.ThrowIfNull(optionsJson);
        return NativeJson("{\"payload\":" + payloadJson + ",\"options\":" + optionsJson + "}",
            astro_render_chart_svg_json, 32 * 1024 * 1024);
    }
    public static JsonElement RenderChartSvgResult<T>(T payload, object? options = null)
        => CheckedResult(RenderChartSvgJson(JsonSerializer.Serialize(payload, ContextSerializerOptions),
            options is null ? "{}" : JsonSerializer.Serialize(options, ContextSerializerOptions))).GetProperty("data").Clone();
    public static string RenderChartSvg<T>(T payload, object? options = null)
        => RenderChartSvgResult(payload, options).GetProperty("svg").GetString()!;
    /// <summary>Render a standalone wheel/positions/houses/aspectMatrix/aspects/houseFocus part.</summary>
    public static string RenderChartPart<T>(T payload, string part, object? options = null)
        => RenderChartSvg(payload, ComposeRenderOptions(options, "sections", new[] { part }));
    /// <summary>Build interaction geometry and source-derived context without generating SVG.</summary>
    public static JsonElement BuildChartModel<T>(T payload, object? options = null)
        => RenderChartSvgResult(payload, ComposeRenderOptions(options, "output", "model")).GetProperty("model").Clone();
    private static Dictionary<string, JsonElement> ComposeRenderOptions(object? options, string key, object value)
    {
        var result = new Dictionary<string, JsonElement>();
        if (options is not null)
        {
            var input = JsonSerializer.SerializeToElement(options, ContextSerializerOptions);
            if (input.ValueKind != JsonValueKind.Object) throw new ArgumentException("Render options must be an object", nameof(options));
            foreach (var property in input.EnumerateObject()) result[property.Name] = property.Value.Clone();
        }
        result[key] = JsonSerializer.SerializeToElement(value, ContextSerializerOptions);
        return result;
    }
    public static JsonElement GetCapabilities()
        => CheckedResult(NativeJson("{}", astro_capabilities_json, 16 * 1024)).GetProperty("data").Clone();

    private static JsonElement Query<T>(string group, string action, T options)
    {
        var input = JsonSerializer.SerializeToElement(options);
        if (input.ValueKind != JsonValueKind.Object)
            throw new ArgumentException($"{group}.{action} expects an options object", nameof(options));
        var request = new Dictionary<string, object?>();
        foreach (var property in input.EnumerateObject())
        {
            if (property.Name is "operation" or "group" or "action")
                throw new ArgumentException($"{property.Name} is set by {group}.{action}", nameof(options));
            request.Add(property.Name, property.Value);
        }
        request.Add("operation", "query");
        request.Add("group", group);
        request.Add("action", action);
        return Calculate(request);
    }

    public static class Geometry
    {
        public static JsonElement Normalize<T>(T options) => Query("geometry", "normalize", options);
        public static JsonElement Separation<T>(T options) => Query("geometry", "separation", options);
        public static JsonElement Midpoint<T>(T options) => Query("geometry", "midpoint", options);
    }

    public static class Aspects
    {
        public static JsonElement Between<T>(T options) => Query("aspects", "between", options);
        public static JsonElement Inspect<T>(T options) => Query("aspects", "inspect", options);
    }

    public static class Houses
    {
        public static JsonElement Locate<T>(T options) => Query("houses", "locate", options);
        public static JsonElement Inspect<T>(T options) => Query("houses", "inspect", options);
    }

    public static class Points
    {
        public static JsonElement Inspect<T>(T options) => Query("points", "inspect", options);
    }
}
