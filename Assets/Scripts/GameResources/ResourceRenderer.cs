using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;

namespace GameResources
{
	/// <summary>
	/// Renders resources using GPU instancing for batched draw calls.
	/// Replaces individual GameObject renderers with efficient instanced rendering.
	/// </summary>
	public class ResourceRenderer : MonoBehaviour
	{
		[Inject] private ResourceManager _resourceManager;

		private const int MAX_INSTANCES_PER_DRAW = 1023;

		private void Update()
		{
			if (_resourceManager == null)
				return;

			RenderResourceType(_resourceManager.GetWoodMatrices(), _resourceManager.GetWoodMeshMaterials());
			RenderResourceType(_resourceManager.GetOreMatrices(), _resourceManager.GetOreMeshMaterials());
			RenderResourceType(_resourceManager.GetFoodMatrices(), _resourceManager.GetFoodMeshMaterials());
			RenderResourceType(_resourceManager.GetGoldMatrices(), _resourceManager.GetGoldMeshMaterials());
			RenderResourceType(_resourceManager.GetRecruitMatrices(), _resourceManager.GetRecruitMeshMaterials());
		}

		private void RenderResourceType(Dictionary<(int meshIndex, int materialIndex), Matrix4x4[]> matricesDict, (List<Mesh> meshes, List<Material> materials) meshMaterialsData)
		{
			if (matricesDict == null || matricesDict.Count == 0 || meshMaterialsData.meshes == null || meshMaterialsData.materials == null)
				return;

			// Render each mesh+material combination
			foreach (var kvp in matricesDict)
			{
				var (meshIndex, materialIndex) = kvp.Key;
				Matrix4x4[] matrices = kvp.Value;

				if (matrices == null || matrices.Length == 0)
					continue;

				// Look up mesh and material by index
				if (meshIndex < 0 || meshIndex >= meshMaterialsData.meshes.Count || materialIndex < 0 || materialIndex >= meshMaterialsData.materials.Count)
					continue;

				Mesh mesh = meshMaterialsData.meshes[meshIndex];
				Material material = meshMaterialsData.materials[materialIndex];

				if (mesh == null || material == null)
					continue;

				// Enable GPU instancing on the material if not already enabled
				if (!material.enableInstancing)
				{
					material.enableInstancing = true;
				}

				// Render in batches of MAX_INSTANCES_PER_DRAW
				for (int i = 0; i < matrices.Length; i += MAX_INSTANCES_PER_DRAW)
				{
					int batchCount = Mathf.Min(MAX_INSTANCES_PER_DRAW, matrices.Length - i);
					Matrix4x4[] batchMatrices = new Matrix4x4[batchCount];
					System.Array.Copy(matrices, i, batchMatrices, 0, batchCount);

					Graphics.DrawMeshInstanced(mesh, 0, material, batchMatrices, batchCount);
				}
			}
		}
	}
}
