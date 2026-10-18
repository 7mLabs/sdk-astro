using System.Reflection.Metadata;using System.Reflection.PortableExecutable;using System.Security.Cryptography;using System.Text.Json;
// Compare the entire PE after excluding build/debug identities only.
foreach(var file in args){using var pe=new PEReader(File.OpenRead(file));var md=pe.GetMetadataReader();var assembly=md.GetAssemblyDefinition();var methods=new List<string>();var api=new List<string>();
foreach(var t in md.TypeDefinitions){var type=md.GetTypeDefinition(t);api.Add(md.GetString(type.Namespace)+"."+md.GetString(type.Name)+":"+type.Attributes);
foreach(var h in type.GetMethods()){var m=md.GetMethodDefinition(h);var id=md.GetString(m.Name)+":"+Convert.ToHexString(md.GetBlobBytes(m.Signature));api.Add(id+":"+m.Attributes+":"+m.ImplAttributes);if(m.RelativeVirtualAddress!=0){var b=pe.GetMethodBody(m.RelativeVirtualAddress);methods.Add(id+":"+Convert.ToHexString(b.GetILBytes()));}}}
if(pe.PEHeaders.CorHeader is null || !pe.PEHeaders.CorHeader.Flags.HasFlag(CorFlags.ILOnly) || pe.PEHeaders.CorHeader.Flags.HasFlag(CorFlags.Requires32Bit)) throw new Exception("Expected portable IL-only assembly");
var normalized=File.ReadAllBytes(file);var guidBytes=md.GetGuid(md.GetModuleDefinition().Mvid).ToByteArray();int matches=0;
for(int i=0;i<=normalized.Length-16;i++){if(normalized.AsSpan(i,16).SequenceEqual(guidBytes)){normalized.AsSpan(i,16).Clear();matches++;}}
if(matches!=1)throw new Exception("Unexpected MVID occurrences");
normalized.AsSpan(pe.PEHeaders.CoffHeaderStartOffset+4,4).Clear();
normalized.AsSpan(pe.PEHeaders.PEHeaderStartOffset+64,4).Clear();
var debugRva=pe.PEHeaders.PEHeader!.DebugTableDirectory.RelativeVirtualAddress;
var debugSection=pe.PEHeaders.SectionHeaders.Single(x=>debugRva>=x.VirtualAddress&&debugRva<x.VirtualAddress+Math.Max(x.VirtualSize,x.SizeOfRawData));
int debugOffset=debugSection.PointerToRawData+debugRva-debugSection.VirtualAddress;int index=0;
foreach(var entry in pe.ReadDebugDirectory()){if(entry.Type!=DebugDirectoryEntryType.CodeView && entry.Type!=DebugDirectoryEntryType.PdbChecksum && entry.Type!=DebugDirectoryEntryType.Reproducible) throw new Exception("Unrecognized debug directory entry");normalized.AsSpan(debugOffset+index*28+4,4).Clear();if(entry.Type==DebugDirectoryEntryType.CodeView||entry.Type==DebugDirectoryEntryType.PdbChecksum) normalized.AsSpan(entry.DataPointer,entry.DataSize).Clear();index++;}
var normalizedSha=Convert.ToHexString(SHA256.HashData(normalized));
var debug=pe.ReadDebugDirectory().Where(x=>x.Type==DebugDirectoryEntryType.CodeView).Select(x=>pe.ReadCodeViewDebugDirectoryData(x)).Select(x=>new{x.Path,x.Guid,x.Age}).ToArray();
Console.WriteLine(JsonSerializer.Serialize(new{file,identity=md.GetString(assembly.Name),version=assembly.Version.ToString(),normalizedSha,mvid=md.GetGuid(md.GetModuleDefinition().Mvid),debug,il=Convert.ToHexString(SHA256.HashData(System.Text.Encoding.UTF8.GetBytes(string.Join("\n",methods)))),api=Convert.ToHexString(SHA256.HashData(System.Text.Encoding.UTF8.GetBytes(string.Join("\n",api))))}));}
