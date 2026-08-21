using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;

namespace StreamTown.Migration
{
    /// <summary>
    /// Exports the authored Main_Menu_02 scene camera and prefab-instance
    /// transforms. This is a migration build artifact, not a runtime Unity
    /// dependency.
    /// </summary>
    public static class MainMenuReferenceExporter
    {
        private const string ScenePath = "Assets/Scenes/Menu/Main_Menu_02.unity";
        private const string OutputArgument = "-streamTownMainMenuReference";

        public static void ExportForBatch()
        {
            try
            {
                string destination = Argument(OutputArgument);
                EditorSceneManager.OpenScene(ScenePath, OpenSceneMode.Single);
                Export(destination);
                Debug.Log($"STREAM_TOWN_MAIN_MENU_REFERENCE_OK={destination}");
            }
            catch (Exception exception)
            {
                Debug.LogException(exception);
                EditorApplication.Exit(1);
            }
        }

        private static void Export(string destination)
        {
            Camera camera = UnityEngine.Object.FindObjectsByType<Camera>(
                    FindObjectsInactive.Include,
                    FindObjectsSortMode.None)
                .FirstOrDefault(candidate => candidate.CompareTag("MainCamera"))
                ?? UnityEngine.Object.FindFirstObjectByType<Camera>(FindObjectsInactive.Include)
                ?? AssetDatabase.LoadAssetAtPath<GameObject>("Assets/ProjectCamera.prefab")
                    ?.GetComponent<Camera>();
            if (camera == null)
                throw new InvalidOperationException("Main menu scene has no camera");

            HashSet<GameObject> prefabRoots = new HashSet<GameObject>();
            foreach (MeshRenderer renderer in UnityEngine.Object.FindObjectsByType<MeshRenderer>(
                         FindObjectsInactive.Exclude,
                         FindObjectsSortMode.None))
            {
                GameObject root = PrefabUtility.GetOutermostPrefabInstanceRoot(renderer.gameObject);
                if (root != null)
                    prefabRoots.Add(root);
            }

            List<PrefabInstanceReference> instances = new List<PrefabInstanceReference>();
            foreach (GameObject root in prefabRoots.OrderBy(HierarchyPath, StringComparer.Ordinal))
            {
                string sourcePath = PrefabUtility.GetPrefabAssetPathOfNearestInstanceRoot(root);
                if (string.IsNullOrWhiteSpace(sourcePath))
                    continue;
                // UI, project settings, and particle systems have dedicated
                // Bevy equivalents. This manifest contains rendered 3D content.
                if (sourcePath.Contains("UserInterface", StringComparison.OrdinalIgnoreCase)
                    || sourcePath.Contains("DefaultSettings", StringComparison.OrdinalIgnoreCase)
                    || sourcePath.Contains("VFX", StringComparison.OrdinalIgnoreCase)
                    || sourcePath.Contains("Lighting", StringComparison.OrdinalIgnoreCase)
                    || sourcePath.EndsWith("Generation_Main_Menu.prefab", StringComparison.OrdinalIgnoreCase))
                    continue;

                Transform transform = root.transform;
                instances.Add(new PrefabInstanceReference
                {
                    HierarchyPath = HierarchyPath(root),
                    SourceGuid = AssetDatabase.AssetPathToGUID(sourcePath),
                    SourcePath = sourcePath.Replace('\\', '/'),
                    Position = transform.position,
                    Rotation = transform.rotation,
                    Scale = transform.lossyScale,
                });
            }

            List<EmbeddedMeshReference> embeddedMeshes = new List<EmbeddedMeshReference>();
            foreach (MeshFilter filter in UnityEngine.Object.FindObjectsByType<MeshFilter>(
                         FindObjectsInactive.Exclude,
                         FindObjectsSortMode.None))
            {
                Mesh mesh = filter.sharedMesh;
                if (mesh == null)
                    continue;
                string meshPath = AssetDatabase.GetAssetPath(mesh);
                if (!string.IsNullOrEmpty(meshPath) && meshPath != ScenePath)
                    continue;
                Vector3[] sourceVertices = mesh.vertices;
                Vector3[] sourceNormals = mesh.normals;
                Vector2[] sourceUv = mesh.uv;
                Vector3[] vertices = new Vector3[sourceVertices.Length];
                Vector3[] normals = new Vector3[sourceVertices.Length];
                Vector2[] uv = new Vector2[sourceVertices.Length];
                for (int index = 0; index < sourceVertices.Length; index++)
                {
                    vertices[index] = filter.transform.TransformPoint(sourceVertices[index]);
                    normals[index] = sourceNormals.Length == sourceVertices.Length
                        ? filter.transform.TransformDirection(sourceNormals[index]).normalized
                        : Vector3.up;
                    uv[index] = sourceUv.Length == sourceVertices.Length ? sourceUv[index] : Vector2.zero;
                }
                embeddedMeshes.Add(new EmbeddedMeshReference
                {
                    HierarchyPath = HierarchyPath(filter.gameObject),
                    Vertices = vertices,
                    Normals = normals,
                    Uv = uv,
                    Triangles = mesh.triangles,
                });
            }

            MenuReference reference = new MenuReference
            {
                SchemaVersion = 1,
                Purpose = "Converted authored Main_Menu_02 scene; no Unity runtime dependency",
                SourceScene = ScenePath,
                Camera = new CameraReference
                {
                    Position = camera.transform.position,
                    Rotation = camera.transform.rotation,
                    Orthographic = camera.orthographic,
                    OrthographicSize = camera.orthographicSize,
                    FieldOfView = camera.fieldOfView,
                    Near = camera.nearClipPlane,
                    Far = camera.farClipPlane,
                    Background = camera.backgroundColor,
                },
                Instances = instances.ToArray(),
                EmbeddedMeshes = embeddedMeshes.ToArray(),
            };
            string directory = Path.GetDirectoryName(destination);
            if (!string.IsNullOrEmpty(directory))
                Directory.CreateDirectory(directory);
            File.WriteAllText(destination, JsonUtility.ToJson(reference, true));
        }

        private static string HierarchyPath(GameObject gameObject)
        {
            List<string> parts = new List<string>();
            Transform current = gameObject.transform;
            while (current != null)
            {
                parts.Add(current.name);
                current = current.parent;
            }
            parts.Reverse();
            return string.Join("/", parts);
        }

        private static string Argument(string name)
        {
            string[] arguments = System.Environment.GetCommandLineArgs();
            for (int index = 0; index + 1 < arguments.Length; index++)
                if (arguments[index] == name)
                    return arguments[index + 1];
            throw new ArgumentException($"Missing required command-line argument {name}");
        }

        [Serializable]
        private sealed class MenuReference
        {
            public int SchemaVersion;
            public string Purpose;
            public string SourceScene;
            public CameraReference Camera;
            public PrefabInstanceReference[] Instances;
            public EmbeddedMeshReference[] EmbeddedMeshes;
        }

        [Serializable]
        private sealed class CameraReference
        {
            public Vector3 Position;
            public Quaternion Rotation;
            public bool Orthographic;
            public float OrthographicSize;
            public float FieldOfView;
            public float Near;
            public float Far;
            public Color Background;
        }

        [Serializable]
        private sealed class PrefabInstanceReference
        {
            public string HierarchyPath;
            public string SourceGuid;
            public string SourcePath;
            public Vector3 Position;
            public Quaternion Rotation;
            public Vector3 Scale;
        }

        [Serializable]
        private sealed class EmbeddedMeshReference
        {
            public string HierarchyPath;
            public Vector3[] Vertices;
            public Vector3[] Normals;
            public Vector2[] Uv;
            public int[] Triangles;
        }
    }
}
