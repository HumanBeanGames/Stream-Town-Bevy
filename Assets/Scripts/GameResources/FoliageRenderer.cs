using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;

namespace GameResources
{
	/// <summary>
	/// Renders foliage using GPU instancing for batched draw calls.
	/// Similar to ResourceRenderer but for non-resource foliage.
	/// </summary>
	public class FoliageRenderer : MonoBehaviour
	{
		[Inject] private FoliageManager _foliageManager;

		private const int MAX_INSTANCES_PER_DRAW = 1023;

		private void Update()
		{
			if (_foliageManager == null)
				return;

			RenderFoliageType(_foliageManager.GetOnLandMatrices());
			RenderFoliageType(_foliageManager.GetUnderWaterMatrices());
		}

		private void RenderFoliageType(Dictionary<(Mesh mesh, Material material), Matrix4x4[]> matricesDict)
		{
			if (matricesDict == null || matricesDict.Count == 0)
				return;

			// Render each mesh+material combination
			foreach (var kvp in matricesDict)
			{
				var (mesh, material) = kvp.Key;
				Matrix4x4[] matrices = kvp.Value;

				if (matrices == null || matrices.Length == 0 || mesh == null || material == null)
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

					Graphics.DrawMeshInstanced(mesh, 0, material, batchMatrices, batchCount, null, UnityEngine.Rendering.ShadowCastingMode.On, true);
				}
			}
		}
	}
}
