using System;
using System.Collections.Generic;
using Character;
using GameResources;
using Processors;
using SavingAndLoading.Structs;
using UnityEngine;
using UnityEngine.Rendering;
using Utils;
using World.Generation;

namespace SavingAndLoading
{
	/// <summary>
	/// Stateless mapping between raw save DTOs and runtime value types. World
	/// creation and processor calls remain in SaveProcessor.
	/// </summary>
	public sealed class SaveDataMapper
	{
		public TransformSaveData CaptureTransform(Transform transform)
		{
			return new TransformSaveData
			{
				Position = CaptureVector3(transform.position),
				Rotation = CaptureVector3(transform.eulerAngles),
				LossyScale = CaptureVector3(transform.lossyScale)
			};
		}

		public void ApplyTransform(Transform transform, TransformSaveData data)
		{
			transform.position = RestoreVector3(data.Position);
			transform.eulerAngles = RestoreVector3(data.Rotation);
			transform.localScale = RestoreVector3(data.LossyScale);
		}

		public Vector3SaveData CaptureVector3(Vector3 value)
		{
			return new Vector3SaveData(value.x, value.y, value.z);
		}

		public Vector3 RestoreVector3(Vector3SaveData value)
		{
			return new Vector3(value.X, value.Y, value.Z);
		}

		public MeshSaveData CaptureMesh(Mesh mesh)
		{
			if (mesh == null)
				throw new InvalidOperationException("The generated terrain mesh is not available to save.");

			Vector3[] vertices = mesh.vertices;
			Vector2[] uvs = mesh.uv;
			Vector3SaveData[] savedVertices = new Vector3SaveData[vertices.Length];
			Vector2SaveData[] savedUvs = new Vector2SaveData[uvs.Length];
			for (int i = 0; i < vertices.Length; i++)
				savedVertices[i] = CaptureVector3(vertices[i]);

			for (int i = 0; i < uvs.Length; i++)
				savedUvs[i] = new Vector2SaveData(uvs[i].x, uvs[i].y);

			return new MeshSaveData(savedVertices, mesh.triangles, savedUvs);
		}

		public Mesh RestoreMesh(MeshSaveData data)
		{
			Vector3SaveData[] savedVertices = data.Verticies ?? Array.Empty<Vector3SaveData>();
			Vector2SaveData[] savedUvs = data.UVs ?? Array.Empty<Vector2SaveData>();
			Vector3[] vertices = new Vector3[savedVertices.Length];
			Vector2[] uvs = new Vector2[savedUvs.Length];

			for (int i = 0; i < savedVertices.Length; i++)
				vertices[i] = RestoreVector3(savedVertices[i]);

			for (int i = 0; i < savedUvs.Length; i++)
				uvs[i] = new Vector2(savedUvs[i].X, savedUvs[i].Y);

			Mesh mesh = new Mesh
			{
				name = "Loaded Mesh",
				indexFormat = data.Uses32BitIndices || vertices.Length > ushort.MaxValue
					? IndexFormat.UInt32
					: IndexFormat.UInt16,
				vertices = vertices,
				uv = uvs,
				triangles = data.Triangles ?? Array.Empty<int>()
			};
			mesh.RecalculateNormals();
			mesh.RecalculateBounds();
			return mesh;
		}

		public InventorySaveData CaptureInventory(Dictionary<Resource, ResourceInventory> inventory)
		{
			List<InventoryEntrySaveData> entries = new List<InventoryEntrySaveData>();
			if (inventory != null)
			{
				foreach (KeyValuePair<Resource, ResourceInventory> pair in inventory)
				{
					if (pair.Value == null)
						continue;

					entries.Add(new InventoryEntrySaveData
					{
						ResourceType = pair.Key.ToString(),
						Amount = pair.Value.Amount,
						MaxAmount = pair.Value.MaxAmount,
						IsUnlimited = pair.Value.IsUnlimited
					});
				}
			}

			return new InventorySaveData { Entries = entries };
		}

		public Dictionary<Resource, ResourceInventory> RestoreInventory(
			InventorySaveData data,
			Dictionary<Resource, ResourceInventory> defaults = null)
		{
			Dictionary<Resource, ResourceInventory> inventory = new Dictionary<Resource, ResourceInventory>();
			if (defaults != null)
			{
				foreach (KeyValuePair<Resource, ResourceInventory> pair in defaults)
				{
					if (pair.Value == null)
						continue;

					inventory[pair.Key] = new ResourceInventory(
						pair.Value.Amount,
						pair.Value.MaxAmount,
						pair.Value.IsUnlimited);
				}
			}

			if (data.Entries == null)
				return inventory;

			foreach (InventoryEntrySaveData entry in data.Entries)
			{
				if (!Enum.TryParse(entry.ResourceType, true, out Resource resource) || resource == Resource.Count)
					continue;

				inventory[resource] = new ResourceInventory(entry.Amount, entry.MaxAmount, entry.IsUnlimited);
			}

			return inventory;
		}

		public List<PlayerRoleSaveData> CaptureRoles(PlayerRoleData[] roles)
		{
			List<PlayerRoleSaveData> data = new List<PlayerRoleSaveData>();
			if (roles == null)
				return data;

			for (int i = 0; i < roles.Length; i++)
			{
				if (roles[i] == null)
					continue;

				data.Add(new PlayerRoleSaveData
				{
					Role = roles[i].Role,
					Level = roles[i].CurrentLevel,
					Experience = roles[i].CurrentExp
				});
			}

			return data;
		}

		public void RestoreRoles(List<PlayerRoleSaveData> savedRoles, PlayerRoleData[] runtimeRoles)
		{
			if (savedRoles == null || runtimeRoles == null)
				return;

			Dictionary<PlayerRole, PlayerRoleSaveData> byRole = new Dictionary<PlayerRole, PlayerRoleSaveData>();
			for (int i = 0; i < savedRoles.Count; i++)
				byRole[savedRoles[i].Role] = savedRoles[i];

			for (int i = 0; i < runtimeRoles.Length; i++)
			{
				PlayerRoleData runtimeRole = runtimeRoles[i];
				if (runtimeRole == null || !byRole.TryGetValue(runtimeRole.Role, out PlayerRoleSaveData savedRole))
					continue;

				runtimeRole.SetLevel(Mathf.Clamp(savedRole.Level, 1, RoleProcessor.MAX_ROLE_LEVEL));
				runtimeRole.SetExperience(Mathf.Max(0, Mathf.RoundToInt(savedRole.Experience)));
			}
		}

		public PlayerCustomizationSaveData CaptureCustomization(CharacterModelHandler model)
		{
			if (model == null)
				return default;

			return new PlayerCustomizationSaveData(
				model.ChosenEyeIndex,
				model.ChosenHairIndex,
				model.ChosenFacialHairIndex,
				model.ChosenSkinColorIndex,
				model.chosenHairColorIndex,
				model.ChosenEyeColorIndex,
				model.ChosenBodyTypeIndex);
		}

		public void RestoreCustomization(CharacterModelHandler model, PlayerCustomizationSaveData data)
		{
			if (model == null)
				return;

			model.SetBodyTypeByIndex(data.ChosenBodyTypeIndex + 1);
			model.SetHairByIndex(data.ChosenHairIndex + 1);
			model.SetHairColorByIndex(data.ChosenHairColourIndex + 1);
			model.SetEyeColorByIndex(data.ChosenEyeColourIndex + 1);
			model.SetEyesByIndex(data.ChosenEyeIndex + 1);
			model.SetFacialHairByIndex(data.ChosenFacialHairIndex + 1);
			model.ChosenSkinColorIndex = data.ChosenSkinIndex;
		}

		public ResourceDataSaveData CaptureResource(ResourceData resource)
		{
			return new ResourceDataSaveData
			{
				PositionX = resource.Position.x,
				PositionY = resource.Position.y,
				PositionZ = resource.Position.z,
				CurrentAmount = resource.CurrentAmount,
				IsUnlimited = resource.IsUnlimited,
				GUID = resource.GUID
			};
		}

		public ResourceData RestoreResource(
			ResourceDataSaveData data,
			Resource fallbackType,
			int meshCount,
			int materialCount,
			bool useLegacyVisualData)
		{
			Resource type = useLegacyVisualData && Enum.TryParse(data.ResourceType, true, out Resource parsedType)
				? parsedType
				: fallbackType;
			Vector3 position = new Vector3(data.PositionX, data.PositionY, data.PositionZ);
			Matrix4x4 matrix;
			int meshIndex;
			int materialIndex;

			if (useLegacyVisualData)
			{
				matrix = new Matrix4x4
				{
					m00 = data.Matrix00, m01 = data.Matrix01, m02 = data.Matrix02, m03 = data.Matrix03,
					m10 = data.Matrix10, m11 = data.Matrix11, m12 = data.Matrix12, m13 = data.Matrix13,
					m20 = data.Matrix20, m21 = data.Matrix21, m22 = data.Matrix22, m23 = data.Matrix23,
					m30 = data.Matrix30, m31 = data.Matrix31, m32 = data.Matrix32, m33 = data.Matrix33
				};
				meshIndex = data.MeshIndex;
				materialIndex = data.MaterialIndex;
			}
			else
			{
				meshIndex = WorldInstanceDeterminism.SelectResourceMesh(position, type, meshCount);
				materialIndex = WorldInstanceDeterminism.SelectResourceMaterial(position, type, materialCount);
				matrix = Matrix4x4.TRS(
					position,
					WorldInstanceDeterminism.SelectResourceRotation(position, type),
					Vector3.one);
			}

			return new ResourceData(
				position,
				type,
				data.CurrentAmount,
				data.IsUnlimited,
				matrix,
				data.GUID,
				meshIndex,
				materialIndex);
		}
	}
}
