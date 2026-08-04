using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Threading;
using Pets.Enumerations;
using SavingAndLoading.Structs;
using Twitch.Commands;
using Twitch.Utils;
using TwitchLib.Client.Enums;
using Utils;

namespace SavingAndLoading
{
	/// <summary>
	/// Canonical field-by-field codec for raw save DTOs. It deliberately avoids
	/// reflection, Unity serialization and runtime objects so large numeric world
	/// arrays can be written directly with no intermediate text representation.
	/// </summary>
	internal static class BinarySaveCodec
	{
		private const int PayloadTrailer = 0x5354454E; // "STEN"
		private const int MaxStringBytes = 1024 * 1024;
		private const int MaxMeshVertices = 10_000_000;
		private const int MaxTriangleIndices = 60_000_000;
		private const int MaxWorldInstances = 5_000_000;
		private const int MaxEntities = 1_000_000;
		private const int MaxSmallCollection = 100_000;
		private const int CancellationCheckMask = 8191;

		private static readonly UTF8Encoding Utf8 = new UTF8Encoding(false, true);

		public static void Write(BinaryWriter writer, SaveFileData data, CancellationToken cancellationToken)
		{
			writer.Write(data.SchemaVersion);
			WriteString(writer, data.SavedAtUtc);
			WriteGame(writer, data.Game, data.SchemaVersion, cancellationToken);
			WritePlayers(writer, data.Players, data.SchemaVersion, cancellationToken);
			writer.Write(PayloadTrailer);
		}

		public static SaveFileData Read(BinaryReader reader, CancellationToken cancellationToken)
		{
			int schemaVersion = reader.ReadInt32();
			SaveFileData data = new SaveFileData
			{
				SchemaVersion = schemaVersion,
				SavedAtUtc = ReadString(reader),
				Game = ReadGame(reader, schemaVersion, cancellationToken),
				Players = ReadPlayers(reader, schemaVersion, cancellationToken)
			};

			if (reader.ReadInt32() != PayloadTrailer)
				throw new InvalidDataException("The save payload is incomplete or corrupt.");

			return data;
		}

		private static void WriteGame(BinaryWriter writer, SaveGameData data, int schemaVersion, CancellationToken cancellationToken)
		{
			WriteWorldGeneration(writer, data.WorldGenData, schemaVersion, cancellationToken);
			WriteList(writer, data.BuildingSaveData, WriteBuilding, cancellationToken);
			WriteList(writer, data.EnemySaveData, WriteEnemy, cancellationToken);
			WriteWorld(writer, data.WorldSaveData, schemaVersion, cancellationToken);
		}

		private static SaveGameData ReadGame(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			return new SaveGameData
			{
				WorldGenData = ReadWorldGeneration(reader, schemaVersion, cancellationToken),
				BuildingSaveData = ReadList(reader, ReadBuilding, MaxEntities, cancellationToken),
				EnemySaveData = ReadList(reader, ReadEnemy, MaxEntities, cancellationToken),
				WorldSaveData = ReadWorld(reader, schemaVersion, cancellationToken)
			};
		}

		private static void WritePlayers(BinaryWriter writer, SavePlayersData data, int schemaVersion, CancellationToken cancellationToken)
		{
			WriteCount(writer, data.PlayerSaveDatas?.Count ?? -1);
			if (data.PlayerSaveDatas == null)
				return;

			for (int i = 0; i < data.PlayerSaveDatas.Count; i++)
			{
				CheckCancellation(cancellationToken, i);
				WritePlayer(writer, data.PlayerSaveDatas[i], schemaVersion);
			}
		}

		private static SavePlayersData ReadPlayers(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			int count = ReadCount(reader, MaxEntities, true);
			List<PlayerSaveData> players = count < 0 ? null : new List<PlayerSaveData>(count);
			for (int i = 0; i < count; i++)
			{
				CheckCancellation(cancellationToken, i);
				players.Add(ReadPlayer(reader, schemaVersion));
			}

			return new SavePlayersData(players);
		}

		private static void WriteWorldGeneration(
			BinaryWriter writer,
			WorldGenSaveData data,
			int schemaVersion,
			CancellationToken cancellationToken)
		{
			if (schemaVersion >= 2)
			{
				writer.Write(data.HasTerrainSeed);
				writer.Write(data.TerrainSeed);
				writer.Write(data.TerrainGeneratorVersion);
				if (!data.HasTerrainSeed)
					WriteMesh(writer, data.MapMesh, cancellationToken);
			}
			else
			{
				WriteMesh(writer, data.MapMesh, cancellationToken);
			}

			WriteResources(writer, data.Resources, schemaVersion, cancellationToken);
			WriteFoliage(writer, data.Foliage, schemaVersion, cancellationToken);
			WriteList(writer, data.EnemyCamps, WriteEnemyCamp, cancellationToken);
		}

		private static WorldGenSaveData ReadWorldGeneration(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			bool hasTerrainSeed = false;
			int terrainSeed = 0;
			int terrainGeneratorVersion = 0;
			MeshSaveData mapMesh = default;
			if (schemaVersion >= 2)
			{
				hasTerrainSeed = reader.ReadBoolean();
				terrainSeed = reader.ReadInt32();
				terrainGeneratorVersion = reader.ReadInt32();
				if (!hasTerrainSeed)
					mapMesh = ReadMesh(reader, cancellationToken);
			}
			else
			{
				mapMesh = ReadMesh(reader, cancellationToken);
			}

			return new WorldGenSaveData
			{
				HasTerrainSeed = hasTerrainSeed,
				TerrainSeed = terrainSeed,
				TerrainGeneratorVersion = terrainGeneratorVersion,
				MapMesh = mapMesh,
				Resources = ReadResources(reader, schemaVersion, cancellationToken),
				Foliage = ReadFoliage(reader, schemaVersion, cancellationToken),
				EnemyCamps = ReadList(reader, ReadEnemyCamp, MaxEntities, cancellationToken)
			};
		}

		private static void WriteMesh(BinaryWriter writer, MeshSaveData data, CancellationToken cancellationToken)
		{
			WriteCount(writer, data.Verticies?.Length ?? -1);
			if (data.Verticies != null)
			{
				for (int i = 0; i < data.Verticies.Length; i++)
				{
					CheckCancellation(cancellationToken, i);
					WriteVector3(writer, data.Verticies[i]);
				}
			}

			WriteCount(writer, data.Triangles?.Length ?? -1);
			if (data.Triangles != null)
			{
				for (int i = 0; i < data.Triangles.Length; i++)
				{
					CheckCancellation(cancellationToken, i);
					writer.Write(data.Triangles[i]);
				}
			}

			WriteCount(writer, data.UVs?.Length ?? -1);
			if (data.UVs != null)
			{
				for (int i = 0; i < data.UVs.Length; i++)
				{
					CheckCancellation(cancellationToken, i);
					WriteVector2(writer, data.UVs[i]);
				}
			}

			writer.Write(data.Uses32BitIndices);
		}

		private static MeshSaveData ReadMesh(BinaryReader reader, CancellationToken cancellationToken)
		{
			int vertexCount = ReadCount(reader, MaxMeshVertices, true);
			Vector3SaveData[] vertices = vertexCount < 0 ? null : new Vector3SaveData[vertexCount];
			for (int i = 0; i < vertexCount; i++)
			{
				CheckCancellation(cancellationToken, i);
				vertices[i] = ReadVector3(reader);
			}

			int triangleCount = ReadCount(reader, MaxTriangleIndices, true);
			int[] triangles = triangleCount < 0 ? null : new int[triangleCount];
			for (int i = 0; i < triangleCount; i++)
			{
				CheckCancellation(cancellationToken, i);
				triangles[i] = reader.ReadInt32();
			}

			int uvCount = ReadCount(reader, MaxMeshVertices, true);
			Vector2SaveData[] uvs = uvCount < 0 ? null : new Vector2SaveData[uvCount];
			for (int i = 0; i < uvCount; i++)
			{
				CheckCancellation(cancellationToken, i);
				uvs[i] = ReadVector2(reader);
			}

			return new MeshSaveData
			{
				Verticies = vertices,
				Triangles = triangles,
				UVs = uvs,
				Uses32BitIndices = reader.ReadBoolean()
			};
		}

		private static void WriteResources(
			BinaryWriter writer,
			ResourceProcessorSaveData data,
			int schemaVersion,
			CancellationToken cancellationToken)
		{
			WriteCount(writer, data.Groups?.Count ?? -1);
			if (data.Groups == null)
				return;

			for (int i = 0; i < data.Groups.Count; i++)
			{
				CheckCancellation(cancellationToken, i);
				WriteResourceGroup(writer, data.Groups[i], schemaVersion, cancellationToken);
			}
		}

		private static ResourceProcessorSaveData ReadResources(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			int count = ReadCount(reader, MaxSmallCollection, true);
			List<ResourceGroupSaveData> groups = count < 0 ? null : new List<ResourceGroupSaveData>(count);
			for (int i = 0; i < count; i++)
			{
				CheckCancellation(cancellationToken, i);
				groups.Add(ReadResourceGroup(reader, schemaVersion, cancellationToken));
			}

			return new ResourceProcessorSaveData { Groups = groups };
		}

		private static void WriteResourceGroup(
			BinaryWriter writer,
			ResourceGroupSaveData data,
			int schemaVersion,
			CancellationToken cancellationToken)
		{
			WriteString(writer, data.ResourceType);
			WriteCount(writer, data.Instances?.Count ?? -1);
			if (data.Instances == null)
				return;

			for (int i = 0; i < data.Instances.Count; i++)
			{
				CheckCancellation(cancellationToken, i);
				WriteResourceInstance(writer, data.Instances[i], schemaVersion);
			}
		}

		private static ResourceGroupSaveData ReadResourceGroup(
			BinaryReader reader,
			int schemaVersion,
			CancellationToken cancellationToken)
		{
			string resourceType = ReadString(reader);
			int count = ReadCount(reader, MaxWorldInstances, true);
			List<ResourceDataSaveData> instances = count < 0 ? null : new List<ResourceDataSaveData>(count);
			for (int i = 0; i < count; i++)
			{
				CheckCancellation(cancellationToken, i);
				instances.Add(ReadResourceInstance(reader, schemaVersion));
			}

			return new ResourceGroupSaveData
			{
				ResourceType = resourceType,
				Instances = instances
			};
		}

		private static void WriteResourceInstance(BinaryWriter writer, ResourceDataSaveData data, int schemaVersion)
		{
			writer.Write(data.PositionX);
			writer.Write(data.PositionY);
			writer.Write(data.PositionZ);
			if (schemaVersion >= 2)
			{
				writer.Write(data.CurrentAmount);
				writer.Write(data.IsUnlimited);
				writer.Write(data.GUID);
				return;
			}

			writer.Write(data.Matrix00);
			writer.Write(data.Matrix01);
			writer.Write(data.Matrix02);
			writer.Write(data.Matrix03);
			writer.Write(data.Matrix10);
			writer.Write(data.Matrix11);
			writer.Write(data.Matrix12);
			writer.Write(data.Matrix13);
			writer.Write(data.Matrix20);
			writer.Write(data.Matrix21);
			writer.Write(data.Matrix22);
			writer.Write(data.Matrix23);
			writer.Write(data.Matrix30);
			writer.Write(data.Matrix31);
			writer.Write(data.Matrix32);
			writer.Write(data.Matrix33);
			WriteString(writer, data.ResourceType);
			writer.Write(data.CurrentAmount);
			writer.Write(data.IsUnlimited);
			writer.Write(data.GUID);
			writer.Write(data.MeshIndex);
			writer.Write(data.MaterialIndex);
		}

		private static ResourceDataSaveData ReadResourceInstance(BinaryReader reader, int schemaVersion)
		{
			ResourceDataSaveData data = new ResourceDataSaveData
			{
				PositionX = reader.ReadSingle(),
				PositionY = reader.ReadSingle(),
				PositionZ = reader.ReadSingle()
			};

			if (schemaVersion >= 2)
			{
				data.CurrentAmount = reader.ReadInt32();
				data.IsUnlimited = reader.ReadBoolean();
				data.GUID = reader.ReadUInt32();
				return data;
			}

			data.Matrix00 = reader.ReadSingle();
			data.Matrix01 = reader.ReadSingle();
			data.Matrix02 = reader.ReadSingle();
			data.Matrix03 = reader.ReadSingle();
			data.Matrix10 = reader.ReadSingle();
			data.Matrix11 = reader.ReadSingle();
			data.Matrix12 = reader.ReadSingle();
			data.Matrix13 = reader.ReadSingle();
			data.Matrix20 = reader.ReadSingle();
			data.Matrix21 = reader.ReadSingle();
			data.Matrix22 = reader.ReadSingle();
			data.Matrix23 = reader.ReadSingle();
			data.Matrix30 = reader.ReadSingle();
			data.Matrix31 = reader.ReadSingle();
			data.Matrix32 = reader.ReadSingle();
			data.Matrix33 = reader.ReadSingle();
			data.ResourceType = ReadString(reader);
			data.CurrentAmount = reader.ReadInt32();
			data.IsUnlimited = reader.ReadBoolean();
			data.GUID = reader.ReadUInt32();
			data.MeshIndex = reader.ReadInt32();
			data.MaterialIndex = reader.ReadInt32();
			return data;
		}

		private static void WriteFoliage(
			BinaryWriter writer,
			FoliageProcessorSaveData data,
			int schemaVersion,
			CancellationToken cancellationToken)
		{
			if (schemaVersion >= 2)
			{
				WriteList(writer, data.OnLandGroups, WriteFoliageGroup, cancellationToken);
				WriteList(writer, data.UnderWaterGroups, WriteFoliageGroup, cancellationToken);
				return;
			}

			WriteList(writer, data.OnLand, WriteFoliageInstance, cancellationToken);
			WriteList(writer, data.UnderWater, WriteFoliageInstance, cancellationToken);
		}

		private static FoliageProcessorSaveData ReadFoliage(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			if (schemaVersion >= 2)
			{
				return new FoliageProcessorSaveData
				{
					OnLandGroups = ReadList(reader, ReadFoliageGroup, MaxSmallCollection, cancellationToken),
					UnderWaterGroups = ReadList(reader, ReadFoliageGroup, MaxSmallCollection, cancellationToken)
				};
			}

			return new FoliageProcessorSaveData
			{
				OnLand = ReadList(reader, ReadFoliageInstance, MaxWorldInstances, cancellationToken),
				UnderWater = ReadList(reader, ReadFoliageInstance, MaxWorldInstances, cancellationToken)
			};
		}

		private static void WriteFoliageGroup(BinaryWriter writer, FoliageGroupSaveData data)
		{
			WriteString(writer, data.SettingsId);
			WriteList(writer, data.Positions, WriteVector3, CancellationToken.None);
		}

		private static FoliageGroupSaveData ReadFoliageGroup(BinaryReader reader)
		{
			return new FoliageGroupSaveData
			{
				SettingsId = ReadString(reader),
				Positions = ReadList(reader, ReadVector3, MaxWorldInstances, CancellationToken.None)
			};
		}

		private static void WriteFoliageInstance(BinaryWriter writer, FoliageInstanceSaveData data)
		{
			WriteTransform(writer, data.Transform);
			WriteString(writer, data.SettingsId);
			writer.Write(data.MeshIndex);
		}

		private static FoliageInstanceSaveData ReadFoliageInstance(BinaryReader reader)
		{
			return new FoliageInstanceSaveData
			{
				Transform = ReadTransform(reader),
				SettingsId = ReadString(reader),
				MeshIndex = reader.ReadInt32()
			};
		}

		private static void WriteEnemyCamp(BinaryWriter writer, EnemyCampSaveData data)
		{
			WriteTransform(writer, data.Transform);
			writer.Write(data.Health);
			writer.Write(data.GUID);
		}

		private static EnemyCampSaveData ReadEnemyCamp(BinaryReader reader)
		{
			return new EnemyCampSaveData
			{
				Transform = ReadTransform(reader),
				Health = reader.ReadInt32(),
				GUID = reader.ReadUInt32()
			};
		}

		private static void WriteBuilding(BinaryWriter writer, BuildingSaveData data)
		{
			WriteTransform(writer, data.BuildingTranform);
			WriteString(writer, data.BuildingType);
			writer.Write(data.BuildingHealth);
			writer.Write(data.GUID);
			writer.Write((int)data.BuildingState);
			writer.Write(data.Level);
			WriteList(writer, data.DestroyedFoliage, WriteDestroyedFoliage, CancellationToken.None);
		}

		private static BuildingSaveData ReadBuilding(BinaryReader reader)
		{
			return new BuildingSaveData
			{
				BuildingTranform = ReadTransform(reader),
				BuildingType = ReadString(reader),
				BuildingHealth = reader.ReadInt32(),
				GUID = reader.ReadUInt32(),
				BuildingState = (BuildingState)reader.ReadInt32(),
				Level = reader.ReadInt32(),
				DestroyedFoliage = ReadList(reader, ReadDestroyedFoliage, MaxWorldInstances, CancellationToken.None)
			};
		}

		private static void WriteDestroyedFoliage(BinaryWriter writer, FoliageSaveData data)
		{
			WriteTransform(writer, data.FoliageTransform);
			WriteString(writer, data.FoliageType);
		}

		private static FoliageSaveData ReadDestroyedFoliage(BinaryReader reader)
		{
			return new FoliageSaveData
			{
				FoliageTransform = ReadTransform(reader),
				FoliageType = ReadString(reader)
			};
		}

		private static void WriteEnemy(BinaryWriter writer, EnemySaveData data)
		{
			WriteTransform(writer, data.Transform);
			WriteString(writer, data.EnemyType);
			writer.Write(data.Health);
			writer.Write(data.GUID);
			writer.Write(data.TargetGUID);
			WriteString(writer, data.TargetPoolType);
			writer.Write(data.CampGUID);
			WriteString(writer, data.CampPoolType);
		}

		private static EnemySaveData ReadEnemy(BinaryReader reader)
		{
			return new EnemySaveData
			{
				Transform = ReadTransform(reader),
				EnemyType = ReadString(reader),
				Health = reader.ReadInt32(),
				GUID = reader.ReadUInt32(),
				TargetGUID = reader.ReadUInt32(),
				TargetPoolType = ReadString(reader),
				CampGUID = reader.ReadUInt32(),
				CampPoolType = ReadString(reader)
			};
		}

		private static void WriteWorld(BinaryWriter writer, WorldSaveData data, int schemaVersion, CancellationToken cancellationToken)
		{
			writer.Write(data.WorldAgeInSeconds);
			writer.Write((int)data.LastEvent);
			writer.Write(data.TimeSinceLastEvent);
			WriteTechTree(writer, data.TechTree, schemaVersion, cancellationToken);
			WriteList(writer, data.TownResources, WriteResourceAmount, cancellationToken);
			writer.Write(data.WoodResourceAmount);
			writer.Write(data.OreResourceAmount);
			writer.Write(data.FoodResourceAmount);
			writer.Write(data.GoldResourceAmount);
			writer.Write(data.IsCurrentRuler);
			writer.Write(data.TimeUntillNextRulerVote);
			WriteString(writer, data.RulerName);
		}

		private static WorldSaveData ReadWorld(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			return new WorldSaveData
			{
				WorldAgeInSeconds = reader.ReadSingle(),
				LastEvent = (GameEventType)reader.ReadInt32(),
				TimeSinceLastEvent = reader.ReadInt32(),
				TechTree = ReadTechTree(reader, schemaVersion, cancellationToken),
				TownResources = ReadList(reader, ReadResourceAmount, MaxSmallCollection, cancellationToken),
				WoodResourceAmount = reader.ReadInt32(),
				OreResourceAmount = reader.ReadInt32(),
				FoodResourceAmount = reader.ReadInt32(),
				GoldResourceAmount = reader.ReadInt32(),
				IsCurrentRuler = reader.ReadBoolean(),
				TimeUntillNextRulerVote = reader.ReadSingle(),
				RulerName = ReadString(reader)
			};
		}

		private static void WriteTechTree(BinaryWriter writer, TechTreeSaveData data, int schemaVersion, CancellationToken cancellationToken)
		{
			writer.Write(data.TechAvailable);
			WriteList(writer, data.UnlockedTechIds, WriteString, cancellationToken);
			WriteList(writer, data.UnlockedTechs, WriteBoolean, cancellationToken);
			WriteString(writer, data.CurrentTechName);
			WriteList(writer, data.CurrentTechData, WriteObjective, cancellationToken);
			if (schemaVersion >= 3)
				WriteTechVote(writer, data.TechVote, cancellationToken);
		}

		private static TechTreeSaveData ReadTechTree(BinaryReader reader, int schemaVersion, CancellationToken cancellationToken)
		{
			TechTreeSaveData data = new TechTreeSaveData
			{
				TechAvailable = reader.ReadBoolean(),
				UnlockedTechIds = ReadList(reader, ReadString, MaxSmallCollection, cancellationToken),
				UnlockedTechs = ReadList(reader, ReadBoolean, MaxSmallCollection, cancellationToken),
				CurrentTechName = ReadString(reader),
				CurrentTechData = ReadList(reader, ReadObjective, MaxSmallCollection, cancellationToken)
			};
			if (schemaVersion >= 3)
				data.TechVote = ReadTechVote(reader, cancellationToken);
			return data;
		}

		private static void WriteTechVote(BinaryWriter writer, TechVoteSaveData data, CancellationToken cancellationToken)
		{
			writer.Write(data.Exists);
			if (!data.Exists)
				return;

			writer.Write(data.SecondsUntilStart);
			writer.Write(data.RemainingDuration);
			WriteList(writer, data.TechNames, WriteString, cancellationToken);
			WriteList(writer, data.PlayerVotes, WriteTechVotePlayer, cancellationToken);
		}

		private static TechVoteSaveData ReadTechVote(BinaryReader reader, CancellationToken cancellationToken)
		{
			bool exists = reader.ReadBoolean();
			if (!exists)
				return default;

			return new TechVoteSaveData
			{
				Exists = true,
				SecondsUntilStart = reader.ReadSingle(),
				RemainingDuration = reader.ReadSingle(),
				TechNames = ReadList(reader, ReadString, MaxSmallCollection, cancellationToken),
				PlayerVotes = ReadList(reader, ReadTechVotePlayer, MaxEntities, cancellationToken)
			};
		}

		private static void WriteTechVotePlayer(BinaryWriter writer, TechVotePlayerSaveData data)
		{
			WriteString(writer, data.TwitchId);
			WriteString(writer, data.OptionName);
		}

		private static TechVotePlayerSaveData ReadTechVotePlayer(BinaryReader reader)
		{
			return new TechVotePlayerSaveData
			{
				TwitchId = ReadString(reader),
				OptionName = ReadString(reader)
			};
		}

		private static void WriteObjective(BinaryWriter writer, ObjectiveSaveData data)
		{
			WriteString(writer, data.ObjectiveType);
			WriteString(writer, data.ResourceType);
			WriteString(writer, data.BuildingType);
			WriteString(writer, data.EnemyType);
			writer.Write(data.RequiredAmount);
			writer.Write(data.Amount);
		}

		private static ObjectiveSaveData ReadObjective(BinaryReader reader)
		{
			return new ObjectiveSaveData
			{
				ObjectiveType = ReadString(reader),
				ResourceType = ReadString(reader),
				BuildingType = ReadString(reader),
				EnemyType = ReadString(reader),
				RequiredAmount = reader.ReadInt32(),
				Amount = reader.ReadInt32()
			};
		}

		private static void WriteResourceAmount(BinaryWriter writer, ResourceAmountSaveData data)
		{
			WriteString(writer, data.ResourceType);
			writer.Write(data.Amount);
		}

		private static ResourceAmountSaveData ReadResourceAmount(BinaryReader reader)
		{
			return new ResourceAmountSaveData
			{
				ResourceType = ReadString(reader),
				Amount = reader.ReadInt32()
			};
		}

		private static void WritePlayer(BinaryWriter writer, PlayerSaveData data, int schemaVersion)
		{
			WriteString(writer, data.TwitchID);
			WriteString(writer, data.TwitchName);
			writer.Write((int)data.TwitchUserType);
			writer.Write((int)data.GameUserType);
			writer.Write(data.IsBroadcaster);
			if (schemaVersion >= 3)
				writer.Write(data.IsUserPlayer);
			writer.Write(data.GUID);
			writer.Write(data.TargetGUID);
			WriteString(writer, data.TargetPoolType);
			writer.Write(data.StationGUID);
			WriteString(writer, data.StationPoolType);
			writer.Write(data.PetActive);
			writer.Write((int)data.CurrentPet);
			WriteList(writer, data.UnlockedPets, WritePetType, CancellationToken.None);
			WriteTransform(writer, data.Transform);
			writer.Write((int)data.CurrentRole);
			writer.Write((int)data.PreviousRole);
			WriteList(writer, data.Roles, WritePlayerRole, CancellationToken.None);
			WriteInventory(writer, data.Inventory);
			WriteCustomization(writer, data.Customization);
			writer.Write(data.Health);
			writer.Write(data.RegenRequiresFood);
		}

		private static PlayerSaveData ReadPlayer(BinaryReader reader, int schemaVersion)
		{
			PlayerSaveData data = new PlayerSaveData
			{
				TwitchID = ReadString(reader),
				TwitchName = ReadString(reader),
				TwitchUserType = (UserType)reader.ReadInt32(),
				GameUserType = (GameUserType)reader.ReadInt32(),
				IsBroadcaster = reader.ReadBoolean()
			};
			if (schemaVersion >= 3)
				data.IsUserPlayer = reader.ReadBoolean();

			data.GUID = reader.ReadUInt32();
			data.TargetGUID = reader.ReadUInt32();
			data.TargetPoolType = ReadString(reader);
			data.StationGUID = reader.ReadUInt32();
			data.StationPoolType = ReadString(reader);
			data.PetActive = reader.ReadBoolean();
			data.CurrentPet = (PetType)reader.ReadInt32();
			data.UnlockedPets = ReadList(reader, ReadPetType, MaxSmallCollection, CancellationToken.None);
			data.Transform = ReadTransform(reader);
			data.CurrentRole = (PlayerRole)reader.ReadInt32();
			data.PreviousRole = (PlayerRole)reader.ReadInt32();
			data.Roles = ReadList(reader, ReadPlayerRole, MaxSmallCollection, CancellationToken.None);
			data.Inventory = ReadInventory(reader);
			data.Customization = ReadCustomization(reader);
			data.Health = reader.ReadInt32();
			data.RegenRequiresFood = reader.ReadBoolean();
			return data;
		}

		private static void WritePlayerRole(BinaryWriter writer, PlayerRoleSaveData data)
		{
			writer.Write((int)data.Role);
			writer.Write(data.Level);
			writer.Write(data.Experience);
		}

		private static PlayerRoleSaveData ReadPlayerRole(BinaryReader reader)
		{
			return new PlayerRoleSaveData
			{
				Role = (PlayerRole)reader.ReadInt32(),
				Level = reader.ReadInt32(),
				Experience = reader.ReadInt32()
			};
		}

		private static void WriteInventory(BinaryWriter writer, InventorySaveData data)
		{
			WriteList(writer, data.Entries, WriteInventoryEntry, CancellationToken.None);
		}

		private static InventorySaveData ReadInventory(BinaryReader reader)
		{
			return new InventorySaveData
			{
				Entries = ReadList(reader, ReadInventoryEntry, MaxSmallCollection, CancellationToken.None)
			};
		}

		private static void WriteInventoryEntry(BinaryWriter writer, InventoryEntrySaveData data)
		{
			WriteString(writer, data.ResourceType);
			writer.Write(data.Amount);
			writer.Write(data.MaxAmount);
			writer.Write(data.IsUnlimited);
		}

		private static InventoryEntrySaveData ReadInventoryEntry(BinaryReader reader)
		{
			return new InventoryEntrySaveData
			{
				ResourceType = ReadString(reader),
				Amount = reader.ReadInt32(),
				MaxAmount = reader.ReadInt32(),
				IsUnlimited = reader.ReadBoolean()
			};
		}

		private static void WriteCustomization(BinaryWriter writer, PlayerCustomizationSaveData data)
		{
			writer.Write(data.ChosenEyeIndex);
			writer.Write(data.ChosenHairIndex);
			writer.Write(data.ChosenFacialHairIndex);
			writer.Write(data.ChosenSkinIndex);
			writer.Write(data.ChosenHairColourIndex);
			writer.Write(data.ChosenEyeColourIndex);
			writer.Write(data.ChosenBodyTypeIndex);
		}

		private static PlayerCustomizationSaveData ReadCustomization(BinaryReader reader)
		{
			return new PlayerCustomizationSaveData
			{
				ChosenEyeIndex = reader.ReadInt32(),
				ChosenHairIndex = reader.ReadInt32(),
				ChosenFacialHairIndex = reader.ReadInt32(),
				ChosenSkinIndex = reader.ReadInt32(),
				ChosenHairColourIndex = reader.ReadInt32(),
				ChosenEyeColourIndex = reader.ReadInt32(),
				ChosenBodyTypeIndex = reader.ReadInt32()
			};
		}

		private static void WriteTransform(BinaryWriter writer, TransformSaveData data)
		{
			WriteVector3(writer, data.Position);
			WriteVector3(writer, data.Rotation);
			WriteVector3(writer, data.LossyScale);
		}

		private static TransformSaveData ReadTransform(BinaryReader reader)
		{
			return new TransformSaveData
			{
				Position = ReadVector3(reader),
				Rotation = ReadVector3(reader),
				LossyScale = ReadVector3(reader)
			};
		}

		private static void WriteVector3(BinaryWriter writer, Vector3SaveData data)
		{
			writer.Write(data.X);
			writer.Write(data.Y);
			writer.Write(data.Z);
		}

		private static Vector3SaveData ReadVector3(BinaryReader reader)
		{
			return new Vector3SaveData(reader.ReadSingle(), reader.ReadSingle(), reader.ReadSingle());
		}

		private static void WriteVector2(BinaryWriter writer, Vector2SaveData data)
		{
			writer.Write(data.X);
			writer.Write(data.Y);
		}

		private static Vector2SaveData ReadVector2(BinaryReader reader)
		{
			return new Vector2SaveData(reader.ReadSingle(), reader.ReadSingle());
		}

		private static void WritePetType(BinaryWriter writer, PetType value)
		{
			writer.Write((int)value);
		}

		private static PetType ReadPetType(BinaryReader reader)
		{
			return (PetType)reader.ReadInt32();
		}

		private static void WriteBoolean(BinaryWriter writer, bool value)
		{
			writer.Write(value);
		}

		private static bool ReadBoolean(BinaryReader reader)
		{
			return reader.ReadBoolean();
		}

		private static void WriteString(BinaryWriter writer, string value)
		{
			if (value == null)
			{
				writer.Write(-1);
				return;
			}

			byte[] bytes = Utf8.GetBytes(value);
			if (bytes.Length > MaxStringBytes)
				throw new InvalidDataException($"A save string exceeds {MaxStringBytes} UTF-8 bytes.");

			writer.Write(bytes.Length);
			writer.Write(bytes);
		}

		private static string ReadString(BinaryReader reader)
		{
			int byteCount = ReadCount(reader, MaxStringBytes, true);
			if (byteCount < 0)
				return null;

			byte[] bytes = reader.ReadBytes(byteCount);
			if (bytes.Length != byteCount)
				throw new EndOfStreamException("A save string ended unexpectedly.");

			return Utf8.GetString(bytes);
		}

		private static void WriteCount(BinaryWriter writer, int count)
		{
			writer.Write(count);
		}

		private static int ReadCount(BinaryReader reader, int maximum, bool allowNull)
		{
			int count = reader.ReadInt32();
			if (allowNull && count == -1)
				return count;

			if (count < 0 || count > maximum)
				throw new InvalidDataException($"Invalid collection length {count}; maximum is {maximum}.");

			return count;
		}

		private static void WriteList<T>(
			BinaryWriter writer,
			List<T> values,
			Action<BinaryWriter, T> writeItem,
			CancellationToken cancellationToken)
		{
			WriteCount(writer, values?.Count ?? -1);
			if (values == null)
				return;

			for (int i = 0; i < values.Count; i++)
			{
				CheckCancellation(cancellationToken, i);
				writeItem(writer, values[i]);
			}
		}

		private static List<T> ReadList<T>(
			BinaryReader reader,
			Func<BinaryReader, T> readItem,
			int maximum,
			CancellationToken cancellationToken)
		{
			int count = ReadCount(reader, maximum, true);
			if (count < 0)
				return null;

			List<T> values = new List<T>(count);
			for (int i = 0; i < count; i++)
			{
				CheckCancellation(cancellationToken, i);
				values.Add(readItem(reader));
			}

			return values;
		}

		private static void CheckCancellation(CancellationToken cancellationToken, int index)
		{
			if ((index & CancellationCheckMask) == 0)
				cancellationToken.ThrowIfCancellationRequested();
		}
	}
}
