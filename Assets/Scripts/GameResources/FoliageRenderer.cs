using System.Collections.Generic;
using UnityEngine;
using Reflex.Attributes;
using Processors;

namespace GameResources
{
	/// <summary>
	/// Renders foliage using GPU instancing for batched draw calls.
	/// Similar to ResourceRenderer but for non-resource foliage.
	/// </summary>
	public class FoliageRenderer : MonoBehaviour
	{
        /// <summary>
        /// Foliage processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private FoliageProcessor _foliageProcessor;

        /// <summary>
        /// Maximum instances per draw call.
        /// </summary>
		private const int MAX_INSTANCES_PER_DRAW = 1023;
		private readonly Matrix4x4[] _batchMatrices = new Matrix4x4[MAX_INSTANCES_PER_DRAW];

        /// <summary>
        /// Updates the foliage rendering each frame.
        /// </summary>
		private void Update()
		{
			if (_foliageProcessor == null)
				return;

			RenderFoliageType(_foliageProcessor.GetOnLandMatrices());
			RenderFoliageType(_foliageProcessor.GetUnderWaterMatrices());
		}

        /// <summary>
        /// Renders foliage of a specific type using GPU instancing.
        /// </summary>
        /// <param name="matricesDict">Dictionary of mesh-material combinations to transformation matrices.</param>
		private void RenderFoliageType(Dictionary<(Mesh mesh, Material material), Matrix4x4[]> matricesDict)
		{
			if (matricesDict == null || matricesDict.Count == 0)
				return;

			// Render each mesh+material combination.
			foreach (var kvp in matricesDict)
			{
				var (mesh, material) = kvp.Key;
				Matrix4x4[] matrices = kvp.Value;

				if (matrices == null || matrices.Length == 0 || mesh == null || material == null)
					continue;

				// Enable GPU instancing on the material if not already enabled.
				if (!material.enableInstancing)
				{
					material.enableInstancing = true;
				}

				// Render in batches of MAX_INSTANCES_PER_DRAW.
				for (int i = 0; i < matrices.Length; i += MAX_INSTANCES_PER_DRAW)
				{
					int batchCount = Mathf.Min(MAX_INSTANCES_PER_DRAW, matrices.Length - i);
					System.Array.Copy(matrices, i, _batchMatrices, 0, batchCount);

					Graphics.DrawMeshInstanced(mesh, 0, material, _batchMatrices, batchCount, null, UnityEngine.Rendering.ShadowCastingMode.On, true);
				}
			}
		}
	}
}
