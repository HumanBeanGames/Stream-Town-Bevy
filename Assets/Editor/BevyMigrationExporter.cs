#if UNITY_EDITOR
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
using UnityEngine.SceneManagement;
using Object = UnityEngine.Object;

namespace StreamTown.Migration
{
    /// <summary>
    /// Produces engine-neutral, versioned JSON while Unity can still resolve its
    /// GUIDs, prefab variants, object references, and serialized field layouts.
    /// Runtime behaviour is never invoked by this exporter.
    /// </summary>
    public static class BevyMigrationExporter
    {
        private const int SchemaVersion = 1;
        private const int MaximumSerializedFields = 20000;
        private const int MaximumExpandedArrayElements = 1024;
        private const string OutputArgument = "-streamTownExport";
        private static readonly string[] IncludedRoots = { "Assets" };
        private static readonly HashSet<string> ShippingScenes = new HashSet<string>(StringComparer.OrdinalIgnoreCase)
        {
            "Assets/Scenes/LOADER_INITIAL.unity",
            "Assets/Scenes/Menu/Main_Menu_02.unity",
            "Assets/Scenes/Worlds/World_Town.unity",
            "Assets/Scenes/Menu/Credits.unity"
        };

        [MenuItem("Tools/Stream Town/Export Bevy Migration JSON")]
        public static void ExportFromMenu()
        {
            string destination = EditorUtility.SaveFilePanel(
                "Export Bevy migration data",
                Path.GetFullPath(Path.Combine(Application.dataPath, "../bevy-port/generated")),
                "unity-export",
                "json");
            if (string.IsNullOrWhiteSpace(destination))
                return;
            Export(destination);
            EditorUtility.RevealInFinder(destination);
        }

        public static void ExportForBatch()
        {
            try
            {
                string destination = ReadOutputArgument();
                Export(destination);
                Debug.Log($"STREAM_TOWN_EXPORT_OK={destination}");
                EditorApplication.Exit(0);
            }
            catch (Exception exception)
            {
                Debug.LogException(exception);
                EditorApplication.Exit(1);
            }
        }

        private static void Export(string destination)
        {
            if (string.IsNullOrWhiteSpace(destination))
                throw new ArgumentException("A migration export destination is required.", nameof(destination));

            string[] guids = AssetDatabase.FindAssets(string.Empty, IncludedRoots);
            Array.Sort(guids, StringComparer.Ordinal);
            var assets = new List<NeutralAsset>(guids.Length);
            var warnings = new List<string>();
            foreach (string guid in guids)
            {
                string path = AssetDatabase.GUIDToAssetPath(guid);
                if (!ShouldExport(path))
                    continue;
                try
                {
                    assets.Add(ExportAsset(guid, path, warnings));
                }
                catch (Exception exception)
                {
                    warnings.Add($"{path}: {exception.GetType().Name}: {exception.Message}");
                    assets.Add(new NeutralAsset
                    {
                        Guid = guid,
                        Path = NormalizePath(path),
                        Kind = AssetKind(path),
                        Status = "error"
                    });
                }
            }

            assets.Sort((left, right) => string.CompareOrdinal(left.Path, right.Path));
            warnings.Sort(StringComparer.Ordinal);
            var export = new NeutralExport
            {
                SchemaVersion = SchemaVersion,
                UnityVersion = Application.unityVersion,
                ProjectName = Application.productName,
                ExportedAtUtc = DateTime.UtcNow.ToString("O", CultureInfo.InvariantCulture),
                Assets = assets,
                Warnings = warnings
            };
            string absolute = Path.GetFullPath(destination);
            Directory.CreateDirectory(Path.GetDirectoryName(absolute) ?? ".");
            string temporary = absolute + ".tmp";
            File.WriteAllText(
                temporary,
                JsonConvert.SerializeObject(export, Formatting.None, new JsonSerializerSettings
                {
                    NullValueHandling = NullValueHandling.Include,
                    TypeNameHandling = TypeNameHandling.None
                }));
            if (File.Exists(absolute))
                File.Delete(absolute);
            File.Move(temporary, absolute);
        }

        private static NeutralAsset ExportAsset(string guid, string path, List<string> warnings)
        {
            Object main = AssetDatabase.LoadMainAssetAtPath(path);
            var record = new NeutralAsset
            {
                Guid = guid,
                Path = NormalizePath(path),
                Kind = AssetKind(path),
                Name = main != null ? main.name : Path.GetFileNameWithoutExtension(path),
                UnityType = main != null ? main.GetType().AssemblyQualifiedName : null,
                Status = main != null ? "exported" : "missing_main_object",
                ImporterFields = SerializeImporter(path, warnings),
                Dependencies = AssetDatabase.GetDependencies(path, false)
                    .Where(dependency => !string.Equals(dependency, path, StringComparison.Ordinal))
                    .Select(DependencyReference)
                    .Where(reference => reference != null)
                    .OrderBy(reference => reference.Path, StringComparer.Ordinal)
                    .ToList()
            };

            if (path.EndsWith(".unity", StringComparison.OrdinalIgnoreCase))
            {
                if (ShippingScenes.Contains(NormalizePath(path)))
                    record.Scene = ExportScene(path, warnings);
                else
                    record.Status = "reference_only";
            }
            else if (main is GameObject gameObject)
                record.GameObject = ExportGameObject(gameObject, path, warnings);
            else if (main is ScriptableObject
                || main is Material
                || main is RuntimeAnimatorController
                || main is AnimationClip)
                record.SerializedFields = SerializeObject(main, warnings, path);
            return record;
        }

        private static List<NeutralField> SerializeImporter(string path, List<string> warnings)
        {
            AssetImporter importer = AssetImporter.GetAtPath(path);
            return importer == null
                ? new List<NeutralField>()
                : SerializeObject(importer, warnings, path + ":importer");
        }

        private static NeutralScene ExportScene(string path, List<string> warnings)
        {
            Scene scene = default;
            try
            {
                scene = EditorSceneManager.OpenScene(path, OpenSceneMode.Additive);
                var roots = scene.GetRootGameObjects()
                    .OrderBy(root => root.name, StringComparer.Ordinal)
                    .Select(root => ExportGameObject(root, path, warnings))
                    .ToList();
                return new NeutralScene { Roots = roots };
            }
            finally
            {
                if (scene.IsValid() && scene.isLoaded)
                    EditorSceneManager.CloseScene(scene, true);
            }
        }

        private static NeutralGameObject ExportGameObject(
            GameObject root,
            string assetPath,
            List<string> warnings)
        {
            var components = new List<NeutralComponent>();
            IEnumerable<Transform> transforms = PrefabUtility.IsPartOfPrefabInstance(root)
                ? new[] { root.transform }
                : root.GetComponentsInChildren<Transform>(true);
            foreach (Transform transform in transforms)
            {
                string hierarchyPath = AnimationUtility.CalculateTransformPath(transform, root.transform);
                Component[] attached = transform.GetComponents<Component>();
                for (int index = 0; index < attached.Length; index++)
                {
                    Component component = attached[index];
                    if (component == null)
                    {
                        warnings.Add($"{assetPath}:{hierarchyPath}: missing script at component {index}");
                        components.Add(new NeutralComponent
                        {
                            HierarchyPath = hierarchyPath,
                            ComponentIndex = index,
                            MissingScript = true
                        });
                        continue;
                    }
                    components.Add(new NeutralComponent
                    {
                        HierarchyPath = hierarchyPath,
                        ComponentIndex = index,
                        Type = component.GetType().AssemblyQualifiedName,
                        Fields = SerializeComponent(component, warnings, assetPath)
                    });
                }
            }

            GameObject source = PrefabUtility.GetCorrespondingObjectFromSource(root);
            string sourcePath = source != null ? AssetDatabase.GetAssetPath(source) : null;
            PropertyModification[] modifications = PrefabUtility.GetPropertyModifications(root);
            return new NeutralGameObject
            {
                PrefabAssetType = PrefabUtility.GetPrefabAssetType(root).ToString(),
                PrefabInstanceStatus = PrefabUtility.GetPrefabInstanceStatus(root).ToString(),
                VariantSource = AssetReference(source),
                VariantSourcePath = string.IsNullOrWhiteSpace(sourcePath) ? null : NormalizePath(sourcePath),
                Components = components,
                Overrides = modifications == null
                    ? new List<NeutralOverride>()
                    : modifications.Select(ExportModification).ToList()
            };
        }

        private static NeutralOverride ExportModification(PropertyModification modification)
        {
            return new NeutralOverride
            {
                Target = AssetReference(modification.target),
                PropertyPath = modification.propertyPath,
                Value = modification.value,
                ObjectReference = AssetReference(modification.objectReference)
            };
        }

        private static List<NeutralField> SerializeObject(
            Object target,
            List<string> warnings,
            string assetPath)
        {
            var fields = new List<NeutralField>();
            try
            {
                var serialized = new SerializedObject(target);
                SerializedProperty iterator = serialized.GetIterator();
                bool enterChildren = true;
                while (iterator.NextVisible(enterChildren))
                {
                    enterChildren = true;
                    if (fields.Count >= MaximumSerializedFields)
                    {
                        warnings.Add($"{assetPath}:{target.name}: serialized field limit reached");
                        break;
                    }
                    if (iterator.isArray && iterator.arraySize > MaximumExpandedArrayElements)
                    {
                        fields.Add(new NeutralField
                        {
                            Path = iterator.propertyPath,
                            PropertyType = iterator.propertyType.ToString(),
                            Value = new { Count = iterator.arraySize, Omitted = true }
                        });
                        enterChildren = false;
                        continue;
                    }
                    if (iterator.propertyPath == "m_Script")
                    {
                        fields.Add(new NeutralField
                        {
                            Path = iterator.propertyPath,
                            PropertyType = iterator.propertyType.ToString(),
                            Value = AssetReference(iterator.objectReferenceValue)
                        });
                        continue;
                    }
                    if (iterator.propertyType == SerializedPropertyType.Generic)
                        continue;
                    fields.Add(new NeutralField
                    {
                        Path = iterator.propertyPath,
                        PropertyType = iterator.propertyType.ToString(),
                        Value = PropertyValue(iterator)
                    });
                }
            }
            catch (Exception exception)
            {
                warnings.Add($"{assetPath}:{target.name}: serialization failed: {exception.Message}");
            }
            return fields;
        }

        private static List<NeutralField> SerializeComponent(
            Component component,
            List<string> warnings,
            string assetPath)
        {
            if (component is Transform transform)
            {
                return new List<NeutralField>
                {
                    Field("localPosition", "Vector3", Vector3Value(transform.localPosition)),
                    Field("localRotation", "Quaternion", Vector4Value(transform.localRotation)),
                    Field("localScale", "Vector3", Vector3Value(transform.localScale))
                };
            }
            if (component is SkinnedMeshRenderer skinned)
            {
                return new List<NeutralField>
                {
                    Field("sharedMesh", "ObjectReference", AssetReference(skinned.sharedMesh)),
                    Field("sharedMaterials", "ObjectReferenceArray", skinned.sharedMaterials.Select(AssetReference).ToArray()),
                    Field("enabled", "Boolean", skinned.enabled)
                };
            }
            if (component is MeshRenderer renderer)
            {
                return new List<NeutralField>
                {
                    Field("sharedMaterials", "ObjectReferenceArray", renderer.sharedMaterials.Select(AssetReference).ToArray()),
                    Field("enabled", "Boolean", renderer.enabled)
                };
            }
            if (component is MeshFilter meshFilter)
            {
                return new List<NeutralField>
                {
                    Field("sharedMesh", "ObjectReference", AssetReference(meshFilter.sharedMesh))
                };
            }
            if (component is Animator animator)
            {
                return new List<NeutralField>
                {
                    Field("runtimeAnimatorController", "ObjectReference", AssetReference(animator.runtimeAnimatorController)),
                    Field("avatar", "ObjectReference", AssetReference(animator.avatar)),
                    Field("applyRootMotion", "Boolean", animator.applyRootMotion)
                };
            }
            if (component is AudioSource audio)
            {
                return new List<NeutralField>
                {
                    Field("clip", "ObjectReference", AssetReference(audio.clip)),
                    Field("volume", "Float", audio.volume),
                    Field("loop", "Boolean", audio.loop),
                    Field("spatialBlend", "Float", audio.spatialBlend)
                };
            }
            if (component is Light light)
            {
                return new List<NeutralField>
                {
                    Field("type", "Enum", light.type.ToString()),
                    Field("color", "Color", ColorValue(light.color)),
                    Field("intensity", "Float", light.intensity),
                    Field("range", "Float", light.range)
                };
            }
            if (component is Camera camera)
            {
                return new List<NeutralField>
                {
                    Field("orthographic", "Boolean", camera.orthographic),
                    Field("orthographicSize", "Float", camera.orthographicSize),
                    Field("fieldOfView", "Float", camera.fieldOfView),
                    Field("nearClipPlane", "Float", camera.nearClipPlane),
                    Field("farClipPlane", "Float", camera.farClipPlane)
                };
            }
            return component is MonoBehaviour
                ? SerializeObject(component, warnings, assetPath)
                : new List<NeutralField>();
        }

        private static NeutralField Field(string path, string propertyType, object value)
        {
            return new NeutralField
            {
                Path = path,
                PropertyType = propertyType,
                Value = value
            };
        }

        private static object PropertyValue(SerializedProperty property)
        {
            switch (property.propertyType)
            {
                case SerializedPropertyType.Integer: return property.longValue;
                case SerializedPropertyType.Boolean: return property.boolValue;
                case SerializedPropertyType.Float: return property.doubleValue;
                case SerializedPropertyType.String: return property.stringValue;
                case SerializedPropertyType.Color: return ColorValue(property.colorValue);
                case SerializedPropertyType.ObjectReference: return AssetReference(property.objectReferenceValue);
                case SerializedPropertyType.LayerMask: return property.intValue;
                case SerializedPropertyType.Enum:
                    return new { Index = property.enumValueIndex, Name = property.enumDisplayNames.ElementAtOrDefault(property.enumValueIndex) };
                case SerializedPropertyType.Vector2: return Vector2Value(property.vector2Value);
                case SerializedPropertyType.Vector3: return Vector3Value(property.vector3Value);
                case SerializedPropertyType.Vector4: return Vector4Value(property.vector4Value);
                case SerializedPropertyType.Rect: return RectValue(property.rectValue);
                case SerializedPropertyType.ArraySize: return property.intValue;
                case SerializedPropertyType.Character: return property.intValue;
                case SerializedPropertyType.AnimationCurve:
                    return property.animationCurveValue.keys.Select(key => new
                    {
                        key.time,
                        key.value,
                        key.inTangent,
                        key.outTangent,
                        key.inWeight,
                        key.outWeight,
                        WeightedMode = key.weightedMode.ToString()
                    }).ToArray();
                case SerializedPropertyType.Bounds: return BoundsValue(property.boundsValue);
                case SerializedPropertyType.Quaternion: return Vector4Value(property.quaternionValue);
                case SerializedPropertyType.ExposedReference: return AssetReference(property.exposedReferenceValue);
                case SerializedPropertyType.FixedBufferSize: return property.fixedBufferSize;
                case SerializedPropertyType.Vector2Int: return new { property.vector2IntValue.x, property.vector2IntValue.y };
                case SerializedPropertyType.Vector3Int: return new { property.vector3IntValue.x, property.vector3IntValue.y, property.vector3IntValue.z };
                case SerializedPropertyType.RectInt:
                    return new { property.rectIntValue.x, property.rectIntValue.y, property.rectIntValue.width, property.rectIntValue.height };
                case SerializedPropertyType.BoundsInt:
                    return new { Position = property.boundsIntValue.position, Size = property.boundsIntValue.size };
                case SerializedPropertyType.ManagedReference:
                    return new { Type = property.managedReferenceFullTypename };
                case SerializedPropertyType.Hash128: return property.hash128Value.ToString();
                default: return null;
            }
        }

        private static NeutralReference DependencyReference(string path)
        {
            string guid = AssetDatabase.AssetPathToGUID(path);
            return string.IsNullOrWhiteSpace(guid)
                ? null
                : new NeutralReference { Guid = guid, Path = NormalizePath(path) };
        }

        private static NeutralReference AssetReference(Object target)
        {
            if (target == null)
                return null;
            AssetDatabase.TryGetGUIDAndLocalFileIdentifier(target, out string guid, out long localId);
            string path = AssetDatabase.GetAssetPath(target);
            return new NeutralReference
            {
                Guid = string.IsNullOrWhiteSpace(guid) ? null : guid,
                LocalId = localId,
                Path = string.IsNullOrWhiteSpace(path) ? null : NormalizePath(path),
                Name = target.name,
                Type = target.GetType().AssemblyQualifiedName
            };
        }

        private static object ColorValue(Color value) => new { value.r, value.g, value.b, value.a };
        private static object Vector2Value(Vector2 value) => new { value.x, value.y };
        private static object Vector3Value(Vector3 value) => new { value.x, value.y, value.z };
        private static object Vector4Value(Vector4 value) => new { value.x, value.y, value.z, value.w };
        private static object Vector4Value(Quaternion value) => new { value.x, value.y, value.z, value.w };
        private static object RectValue(Rect value) => new { value.x, value.y, value.width, value.height };
        private static object BoundsValue(Bounds value) => new
        {
            Center = Vector3Value(value.center),
            Size = Vector3Value(value.size)
        };

        private static bool ShouldExport(string path)
        {
            if (string.IsNullOrWhiteSpace(path) || path.EndsWith(".cs", StringComparison.OrdinalIgnoreCase))
                return false;
            if (AssetDatabase.IsValidFolder(path))
                return false;
            string normalized = NormalizePath(path).ToLowerInvariant();
            string extension = Path.GetExtension(normalized);
            bool supported = new[]
            {
                ".unity", ".prefab", ".asset", ".anim", ".controller", ".overridecontroller",
                ".fbx", ".obj", ".blend", ".mat", ".shader", ".shadergraph", ".png", ".tga",
                ".jpg", ".jpeg", ".psd", ".vfx", ".wav", ".ogg", ".mp3", ".json", ".txt", ".bytes"
            }.Contains(extension);
            return supported
                && !normalized.StartsWith("assets/plugins/", StringComparison.Ordinal)
                && !normalized.StartsWith("assets/textmesh pro/", StringComparison.Ordinal)
                && !normalized.StartsWith("assets/astarpathfindingproject/", StringComparison.Ordinal)
                && !normalized.StartsWith("assets/reflexoverride/", StringComparison.Ordinal)
                && !normalized.StartsWith("assets/migrationonly/", StringComparison.Ordinal);
        }

        private static string AssetKind(string path)
        {
            string extension = Path.GetExtension(path).ToLowerInvariant();
            switch (extension)
            {
                case ".unity": return "scene";
                case ".prefab": return "prefab";
                case ".asset": return "scriptable_asset";
                case ".controller": return "animator_controller";
                case ".overridecontroller": return "animator_override_controller";
                case ".anim": return "animation_clip";
                case ".fbx": case ".obj": case ".blend": return "model";
                case ".mat": return "material";
                case ".shader": case ".shadergraph": return "shader";
                case ".vfx": return "vfx";
                default: return "asset";
            }
        }

        private static string ReadOutputArgument()
        {
            string[] arguments = System.Environment.GetCommandLineArgs();
            for (int index = 0; index < arguments.Length - 1; index++)
            {
                if (string.Equals(arguments[index], OutputArgument, StringComparison.Ordinal))
                    return Path.GetFullPath(arguments[index + 1]);
            }
            return Path.GetFullPath(Path.Combine(
                Application.dataPath,
                "../bevy-port/generated/unity-export.json"));
        }

        private static string NormalizePath(string path) => path.Replace('\\', '/');

        [Serializable]
        private sealed class NeutralExport
        {
            public int SchemaVersion;
            public string UnityVersion;
            public string ProjectName;
            public string ExportedAtUtc;
            public List<NeutralAsset> Assets;
            public List<string> Warnings;
        }

        [Serializable]
        private sealed class NeutralAsset
        {
            public string Guid;
            public string Path;
            public string Kind;
            public string Name;
            public string UnityType;
            public string Status;
            public List<NeutralReference> Dependencies = new List<NeutralReference>();
            public List<NeutralField> ImporterFields = new List<NeutralField>();
            public List<NeutralField> SerializedFields = new List<NeutralField>();
            public NeutralGameObject GameObject;
            public NeutralScene Scene;
        }

        [Serializable]
        private sealed class NeutralScene
        {
            public List<NeutralGameObject> Roots;
        }

        [Serializable]
        private sealed class NeutralGameObject
        {
            public string PrefabAssetType;
            public string PrefabInstanceStatus;
            public NeutralReference VariantSource;
            public string VariantSourcePath;
            public List<NeutralComponent> Components;
            public List<NeutralOverride> Overrides;
        }

        [Serializable]
        private sealed class NeutralComponent
        {
            public string HierarchyPath;
            public int ComponentIndex;
            public string Type;
            public bool MissingScript;
            public List<NeutralField> Fields = new List<NeutralField>();
        }

        [Serializable]
        private sealed class NeutralField
        {
            public string Path;
            public string PropertyType;
            public object Value;
        }

        [Serializable]
        private sealed class NeutralOverride
        {
            public NeutralReference Target;
            public string PropertyPath;
            public string Value;
            public NeutralReference ObjectReference;
        }

        [Serializable]
        private sealed class NeutralReference
        {
            public string Guid;
            public long LocalId;
            public string Path;
            public string Name;
            public string Type;
        }
    }
}
#endif
