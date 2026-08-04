#if UNITY_EDITOR
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using UnityEditor;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;
using GameEventSystem;
using GameEventSystem.Events.Voting;
using Processors;
using Reflex.Core;
using TechTree.Data;
using UserInterface;
using AstarPathComponent = AstarPath;

namespace StreamTown.EditorTools
{
	/// <summary>
	/// Provides a deliberately small, file-based command surface for local automated diagnostics.
	/// Commands and responses live under Temp so they are never included in builds or source control.
	/// </summary>
	[InitializeOnLoad]
	internal static class StreamTownSessionBridge
	{
		private const string CommandFileName = "StreamTownSession.command";
		private const string ResponseFileName = "StreamTownSession.response.json";
		private const int FramesToCapture = 8;
		private const double PollIntervalSeconds = 0.2d;

		private static readonly string CommandPath = Path.Combine(Directory.GetCurrentDirectory(), "Temp", CommandFileName);
		private static readonly string ResponsePath = Path.Combine(Directory.GetCurrentDirectory(), "Temp", ResponseFileName);

		private static double _nextPollTime;
		private static FrameCapture _frameCapture;
		private static SaveProcessor _observedSaveProcessor;
		private static string _observedSaveCommand;

		static StreamTownSessionBridge()
		{
			EditorApplication.update += Update;
		}

		[MenuItem("Stream Town/Force Save Current Game _F9")]
		private static void ForceSaveCurrentGame()
		{
			SaveProcessor saveProcessor = ResolveSaveProcessor();
			if (!EditorApplication.isPlaying || EditorApplication.isPaused || saveProcessor == null || saveProcessor.IsBusy)
			{
				UnityEngine.Debug.LogWarning("[SessionBridge] A force save requires an unpaused, running world and an idle SaveProcessor.");
				return;
			}

			saveProcessor.SaveGame();
		}

		[MenuItem("Stream Town/Force Save Current Game _F9", true)]
		private static bool CanForceSaveCurrentGame()
		{
			SaveProcessor saveProcessor = ResolveSaveProcessor();
			return EditorApplication.isPlaying && !EditorApplication.isPaused && saveProcessor != null && !saveProcessor.IsBusy;
		}

		private static void Update()
		{
			CaptureFrameTiming();

			if (EditorApplication.timeSinceStartup < _nextPollTime)
				return;

			_nextPollTime = EditorApplication.timeSinceStartup + PollIntervalSeconds;
			if (!File.Exists(CommandPath))
				return;

			string command;
			try
			{
				command = File.ReadAllText(CommandPath).Trim().ToLowerInvariant();
				File.Delete(CommandPath);
			}
			catch (Exception exception)
			{
				WriteResponse("read-command", false, exception.Message);
				return;
			}

			Execute(command);
		}

		private static void Execute(string command)
		{
			switch (command)
			{
				case "status":
					WriteResponse(command, true, GetEditorStatus());
					break;

				case "play":
					if (!EditorApplication.isPlayingOrWillChangePlaymode)
						EditorApplication.EnterPlaymode();
					WriteResponse(command, true, "Play mode requested.");
					break;

				case "stop":
					if (EditorApplication.isPlayingOrWillChangePlaymode)
						EditorApplication.ExitPlaymode();
					WriteResponse(command, true, "Edit mode requested.");
					break;

				case "generate-world":
					GenerateWorld(command);
					break;

				case "capture-frames":
					CaptureFrames(command);
					break;

				case "spawn-debug-character":
					SpawnDebugCharacter(command);
					break;

				case "save-game":
					SaveGame(command);
					break;

				case "press-f9":
					PressF9(command);
					break;

				case "quit-to-main-menu":
					QuitToMainMenu(command);
					break;

				case "click-load-game":
					ClickLoadGame(command);
					break;

				case "load-saved-world":
					LoadSavedWorld(command);
					break;

				case "trigger-ruler-vote":
					TriggerRulerVote(command);
					break;

				case "authorize-twitch":
					AuthorizeTwitch(command);
					break;

				case "cast-vote-3":
					CastVoteThree(command);
					break;

				case "spawn-test-blargul":
					SpawnTestBlargul(command);
					break;

				case "exercise-health-bars":
					ExerciseHealthBars(command);
					break;

				case "validate-chat-commands":
					ValidateChatCommands(command);
					break;

				default:
					WriteResponse(command, false, "Unknown command. Supported commands: status, play, stop, generate-world, capture-frames, spawn-debug-character, save-game, press-f9, quit-to-main-menu, click-load-game, load-saved-world, trigger-ruler-vote, authorize-twitch, cast-vote-3, spawn-test-blargul, exercise-health-bars, validate-chat-commands.");
					break;
			}
		}

		private static void ValidateChatCommands(string command)
		{
			if (!EditorApplication.isPlaying)
			{
				WriteResponse(command, false, "The editor must be running.");
				return;
			}

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer == null || !projectContainer.HasBinding(typeof(TwitchChatProcessor)))
			{
				WriteResponse(command, false, "TwitchChatProcessor was not ready.");
				return;
			}

			TwitchChatProcessor chat = projectContainer.Resolve<TwitchChatProcessor>();
			FieldInfo field = typeof(TwitchChatProcessor).GetField("_commandDictionary", BindingFlags.Instance | BindingFlags.NonPublic);
			Twitch.Commands.CommandDictionary dictionary = field?.GetValue(chat) as Twitch.Commands.CommandDictionary;
			if (dictionary == null)
			{
				WriteResponse(command, false, "The command dictionary was not initialized.");
				return;
			}

			List<string> results = new List<string>();
			bool passed = true;
			CheckCommandShape(dictionary, "roles", Array.Empty<string>(), true, results, ref passed);
			CheckCommandShape(dictionary, "roles", new[] { "unexpected" }, false, results, ref passed);
			CheckCommandShape(dictionary, "role", new[] { "builder" }, true, results, ref passed);
			CheckCommandShape(dictionary, "role", new[] { "nonsense" }, false, results, ref passed);
			CheckCommandShape(dictionary, "move", new[] { "left", "2" }, true, results, ref passed);
			CheckCommandShape(dictionary, "move", new[] { "sideways", "2" }, false, results, ref passed);
			WriteResponse(command, passed, string.Join("; ", results));
		}

		private static void CheckCommandShape(
			Twitch.Commands.CommandDictionary dictionary,
			string command,
			IReadOnlyList<string> args,
			bool expected,
			List<string> results,
			ref bool passed)
		{
			bool actual = dictionary.TryValidateArguments(command, args, out string usage);
			bool checkPassed = actual == expected && !string.IsNullOrWhiteSpace(usage);
			passed &= checkPassed;
			results.Add($"{command} {string.Join(" ", args)}={actual} (expected={expected}, usage={usage})");
		}

		private static void SpawnTestBlargul(string command)
		{
			if (!EditorApplication.isPlaying)
			{
				WriteResponse(command, false, "The editor must be running.");
				return;
			}

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer == null || !projectContainer.HasBinding(typeof(ObjectPoolingProcessor)) ||
				!projectContainer.HasBinding(typeof(PlayerProcessor)))
			{
				WriteResponse(command, false, "Pooling or player data is not ready.");
				return;
			}

			PlayerProcessor players = projectContainer.Resolve<PlayerProcessor>();
			Character.Player player = players.UserPlayer ?? (players.PlayerCount() > 0 ? players.GetPlayer(0) : null);
			Vector3 origin = player?.Character != null ? player.Character.transform.position : Vector3.zero;
			Utils.Pooling.PoolableObject pooled = projectContainer.Resolve<ObjectPoolingProcessor>()
				.GetPooledObject("Blargul", origin + Vector3.forward * 5f, Quaternion.identity);
			if (pooled == null)
			{
				WriteResponse(command, false, "The Blargul pool was unavailable.");
				return;
			}

			WriteResponse(command, true, $"Spawned {pooled.name} at {pooled.transform.position} near {player?.TwitchUser?.Username ?? "world origin"}.");
		}

		private static void ExerciseHealthBars(string command)
		{
			if (!EditorApplication.isPlaying)
			{
				WriteResponse(command, false, "The editor must be running.");
				return;
			}

			Units.HealthHandler[] handlers = UnityEngine.Object.FindObjectsByType<Units.HealthHandler>(
				FindObjectsInactive.Exclude, FindObjectsSortMode.None);
			List<string> results = new List<string>();
			ExerciseHealthBarCategory(handlers, "player", handler => handler.GetComponent<Character.RoleHandler>() != null, results);
			ExerciseHealthBarCategory(handlers, "monster", handler => handler.GetComponent<Enemies.Enemy>() != null, results);
			ExerciseHealthBarCategory(handlers, "building", handler => handler.GetComponent<Buildings.BuildingBase>() != null, results);
			bool success = results.Count == 3;
			WriteResponse(command, success, results.Count > 0 ? string.Join("; ", results) : "No health-bearing test subjects were active.");
		}

		private static void ExerciseHealthBarCategory(
			Units.HealthHandler[] handlers,
			string category,
			Func<Units.HealthHandler, bool> predicate,
			List<string> results)
		{
			for (int i = 0; i < handlers.Length; i++)
			{
				Units.HealthHandler handler = handlers[i];
				if (handler == null || handler.MaxHealth <= 1 || !predicate(handler))
					continue;

				int originalHealth = handler.Health;
				handler.TakeDamage(1, null);
				Units.UnitHealthBar unitBar = handler.GetComponentInChildren<Units.UnitHealthBar>(true);
				UserInterface_BuildingHealthBar buildingBar =
					handler.GetComponentInChildren<UserInterface_BuildingHealthBar>(true);
				buildingBar?.UpdateHealthBar();
				float expected = handler.HealthPercentage;
				float displayed = unitBar != null ? unitBar.DisplayedHealth : buildingBar != null ? buildingBar.DisplayedHealth : -1f;
				bool visibleAfterDamage = unitBar != null ? unitBar.IsVisible : buildingBar != null && buildingBar.IsConfigured;
				bool passed = displayed >= 0f && Mathf.Abs(displayed - expected) < 0.001f && visibleAfterDamage;
				results.Add($"{category}={passed} ({handler.name}, expected={expected:F3}, displayed={displayed:F3})");
				handler.SetHealth(originalHealth);
				return;
			}
		}

		private static void AuthorizeTwitch(string command)
		{
			if (!EditorApplication.isPlaying)
			{
				WriteResponse(command, false, "The editor must be running.");
				return;
			}

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer == null ||
				!projectContainer.HasBinding(typeof(TwitchChatProcessor)) ||
				!projectContainer.HasBinding(typeof(TechTreeProcessor)))
			{
				WriteResponse(command, false, "TwitchChatProcessor or TechTreeProcessor was not ready.");
				return;
			}

			TwitchChatProcessor chat = projectContainer.Resolve<TwitchChatProcessor>();
			if (!chat.TryAuthorizeBroadcasterConnection(chat.GetBroadcasterConnectCode(), true))
			{
				WriteResponse(command, false, "The current Twitch challenge could not be authorized.");
				return;
			}

			chat.CompleteBroadcasterConnection();
			projectContainer.Resolve<TechTreeProcessor>().RequestDelayedSetup();
			WriteResponse(command, true, "Twitch authorization completed through the same runtime authorization API.");
		}

		private static void CastVoteThree(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer == null ||
				!projectContainer.HasBinding(typeof(TwitchChatProcessor)) ||
				!projectContainer.HasBinding(typeof(PlayerProcessor)) ||
				!projectContainer.HasBinding(typeof(GameEventProcessor)))
			{
				WriteResponse(command, false, "The Twitch, player, or event processor was not ready.");
				return;
			}

			GameEvent currentEvent = projectContainer.Resolve<GameEventProcessor>().CurrentEvent;
			if (!(currentEvent is VoteEvent vote))
			{
				WriteResponse(command, false, $"No vote is active. Current event: {currentEvent?.Event.ToString() ?? "none"}.");
				return;
			}

			PlayerProcessor players = projectContainer.Resolve<PlayerProcessor>();
			Character.Player player = players.PlayerCount() > 0 ? players.GetPlayer(0) : null;
			if (player == null)
			{
				WriteResponse(command, false, "No player was available to cast the diagnostic vote.");
				return;
			}

			int votesBefore = vote.PlayerVotes.Count;
			TwitchChatProcessor chat = projectContainer.Resolve<TwitchChatProcessor>();
			chat.ProcessDebugCommand("vote 3", player);
			int votesAfter = vote.PlayerVotes.Count;
			bool accepted = votesAfter == votesBefore + 1;
			WriteResponse(command, accepted, $"{chat.LastCommandResult}; votes={votesBefore}->{votesAfter}; options={string.Join(",", vote.Options.Keys)}.");
		}

		private static void TriggerRulerVote(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer == null || !projectContainer.HasBinding(typeof(GameEventProcessor)))
			{
				WriteResponse(command, false, "GameEventProcessor was not ready.");
				return;
			}

			GameEventProcessor events = projectContainer.Resolve<GameEventProcessor>();
			events.CanStartNewRulerVote = true;
			events.TimeTillRulerVote = 0f;
			WriteResponse(command, true, "Ruler-vote timer forced to expire on the next processor update.");
		}

		private static void GenerateWorld(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			UserInterface.MainMenu.MainMenuManager mainMenuManager = UnityEngine.Object.FindAnyObjectByType<UserInterface.MainMenu.MainMenuManager>();
			if (mainMenuManager == null)
			{
				WriteResponse(command, false, "MainMenuManager was not ready.");
				return;
			}

			const BindingFlags fieldFlags = BindingFlags.Instance | BindingFlags.NonPublic;
			Type managerType = typeof(UserInterface.MainMenu.MainMenuManager);
			var loadingManager = managerType.GetField("_loadingManager", fieldFlags)?.GetValue(mainMenuManager) as UserInterface.MainMenu.LoadingManager;
			var metadata = managerType.GetField("_metaData", fieldFlags)?.GetValue(mainMenuManager) as MetaData.MetaData;
			object sceneIndexValue = managerType.GetField("_sceneIndex", fieldFlags)?.GetValue(mainMenuManager);
			if (loadingManager == null || metadata == null || !(sceneIndexValue is int sceneIndex))
			{
				WriteResponse(command, false, $"Main-menu runtime dependencies were not ready. LoadingManager={loadingManager != null}; Metadata={metadata != null}; SceneIndex={sceneIndexValue ?? "null"}.");
				return;
			}

			metadata.LoadType = MetaData.LoadType.Generate;
			loadingManager.LoadWorldScene(sceneIndex, MetaData.LoadType.Generate);
			WriteResponse(command, true, $"World generation requested through LoadingManager for build index {sceneIndex}.");
		}

		private static void CaptureFrames(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			BeginFrameCapture(command, 0d, 0L);
		}

		private static void SpawnDebugCharacter(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			UserInterface_Debug debugInterface = UnityEngine.Object.FindAnyObjectByType<UserInterface_Debug>();
			if (debugInterface == null || !debugInterface.isActiveAndEnabled)
			{
				WriteResponse(command, false, "No active UserInterface_Debug was found. Wait for the world scene to finish loading.");
				return;
			}

			long memoryBefore = UnityEngine.Profiling.Profiler.GetMonoUsedSizeLong();
			Stopwatch stopwatch = Stopwatch.StartNew();
			try
			{
				debugInterface.SpawnCharacter();
			}
			catch (Exception exception)
			{
				stopwatch.Stop();
				WriteResponse(command, false, exception.ToString(), stopwatch.Elapsed.TotalMilliseconds);
				return;
			}

			stopwatch.Stop();
			long memoryAfter = UnityEngine.Profiling.Profiler.GetMonoUsedSizeLong();
			BeginFrameCapture(command, stopwatch.Elapsed.TotalMilliseconds, memoryAfter - memoryBefore);
		}

		private static void SaveGame(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			SaveProcessor saveProcessor = ResolveSaveProcessor();
			if (saveProcessor == null || saveProcessor.IsBusy)
			{
				WriteResponse(command, false, saveProcessor == null ? "SaveProcessor was not ready." : "SaveProcessor is busy.");
				return;
			}

			ObserveSaveOperation(saveProcessor, command);
			saveProcessor.SaveGame();
		}

		private static void PressF9(string command)
		{
			SaveProcessor saveProcessor = ResolveSaveProcessor();
			if (!EditorApplication.isPlaying || EditorApplication.isPaused || saveProcessor == null || saveProcessor.IsBusy)
			{
				WriteResponse(command, false, "F9 requires an unpaused, running world and an idle SaveProcessor.");
				return;
			}

			ObserveSaveOperation(saveProcessor, command);
			if (EditorApplication.ExecuteMenuItem("Stream Town/Force Save Current Game"))
				return;

			StopObservingSave();
			WriteResponse(command, false, "Unity could not execute the F9 force-save menu command.");
		}

		private static void QuitToMainMenu(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			UserInterface_GameMenu gameMenu = UnityEngine.Object.FindAnyObjectByType<UserInterface_GameMenu>();
			if (gameMenu == null || !gameMenu.isActiveAndEnabled)
			{
				WriteResponse(command, false, "No active game-menu controller was found.");
				return;
			}

			gameMenu.ToggleMainMenuPanel();
			WriteResponse(command, true, "Quit-to-main-menu button action invoked.");
		}

		private static void ClickLoadGame(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			SaveProcessor saveProcessor = ResolveSaveProcessor();
			UserInterface.MainMenu.MainMenuManager mainMenuManager =
				UnityEngine.Object.FindAnyObjectByType<UserInterface.MainMenu.MainMenuManager>();
			if (saveProcessor == null || !saveProcessor.HasSaveGame || mainMenuManager == null)
			{
				WriteResponse(command, false, $"Load Game was not ready. Save={saveProcessor?.HasSaveGame == true}; MainMenu={mainMenuManager != null}.");
				return;
			}

			const BindingFlags fieldFlags = BindingFlags.Instance | BindingFlags.NonPublic;
			Button loadButton = typeof(UserInterface.MainMenu.MainMenuManager)
				.GetField("_loadButton", fieldFlags)?.GetValue(mainMenuManager) as Button;
			if (loadButton == null || !loadButton.IsActive() || !loadButton.interactable)
			{
				WriteResponse(command, false, $"Load Game button was unavailable. Found={loadButton != null}; Active={loadButton?.IsActive() == true}; Interactable={loadButton?.interactable == true}; SaveProcessor={saveProcessor != null}; HasSave={saveProcessor != null && saveProcessor.HasSaveGame}; SavePath={saveProcessor?.SavePath ?? "null"}.");
				return;
			}

			ObserveSaveOperation(saveProcessor, command);
			loadButton.onClick.Invoke();

			var runtimeData = typeof(UserInterface.MainMenu.MainMenuManager)
				.GetField("_mainMenuRuntimeData", fieldFlags)?.GetValue(mainMenuManager) as MainMenuRuntimeData;
			bool buttonStartedLoad = runtimeData?.Loading == true;
			if (!buttonStartedLoad)
			{
				var loadingManager = typeof(UserInterface.MainMenu.MainMenuManager)
					.GetField("_loadingManager", fieldFlags)?.GetValue(mainMenuManager) as UserInterface.MainMenu.LoadingManager;
				object sceneIndexValue = typeof(UserInterface.MainMenu.MainMenuManager)
					.GetField("_sceneIndex", fieldFlags)?.GetValue(mainMenuManager);
				if (loadingManager == null || !(sceneIndexValue is int sceneIndex))
				{
					StopObservingSave();
					WriteResponse(command, false, "Load Game opened its channel-name prompt, but the diagnostic bridge could not continue the saved-world transition.");
					return;
				}

				// A real first-time user must enter a Twitch channel. Diagnostics must not
				// overwrite that user setting, so continue the already-clicked button's
				// load intent directly through the same LoadingManager pipeline.
				loadingManager.LoadWorldScene(sceneIndex, MetaData.LoadType.Load);
				WriteResponse(command, true, "Load Game button invoked; channel setup bypassed for diagnostics; waiting for load completion.");
				return;
			}

			WriteResponse(command, true, "Load Game button invoked; waiting for load completion.");
		}

		private static void OnSaveOperationChanged(SaveOperationState state, float progress, string status)
		{
			if (state != SaveOperationState.Succeeded && state != SaveOperationState.Failed)
				return;

			bool success = state == SaveOperationState.Succeeded;
			string savePath = _observedSaveProcessor?.SavePath;
			string command = _observedSaveCommand ?? "save-operation";
			StopObservingSave();
			string successMessage = command == "click-load-game"
				? $"Load completed from {savePath}."
				: $"Save completed at {savePath}.";
			WriteResponse(command, success, success ? successMessage : status);
		}

		private static void ObserveSaveOperation(SaveProcessor saveProcessor, string command)
		{
			StopObservingSave();
			_observedSaveProcessor = saveProcessor;
			_observedSaveCommand = command;
			_observedSaveProcessor.OperationChanged += OnSaveOperationChanged;
		}

		private static void StopObservingSave()
		{
			if (_observedSaveProcessor != null)
				_observedSaveProcessor.OperationChanged -= OnSaveOperationChanged;
			_observedSaveProcessor = null;
			_observedSaveCommand = null;
		}

		private static void LoadSavedWorld(string command)
		{
			if (!EditorApplication.isPlaying || EditorApplication.isPaused)
			{
				WriteResponse(command, false, "The editor must be running and unpaused.");
				return;
			}

			SaveProcessor saveProcessor = ResolveSaveProcessor();
			MetaData.MetaData metadata = UnityEngine.Object.FindAnyObjectByType<MetaData.MetaData>();
			UserInterface.MainMenu.LoadingManager loadingManager = UnityEngine.Object.FindAnyObjectByType<UserInterface.MainMenu.LoadingManager>();
			if (saveProcessor == null || !saveProcessor.HasSaveGame || metadata == null || loadingManager == null)
			{
				WriteResponse(command, false, $"Load dependencies were not ready. Save={saveProcessor?.HasSaveGame == true}; Metadata={metadata != null}; LoadingManager={loadingManager != null}.");
				return;
			}

			metadata.LoadType = MetaData.LoadType.Load;
			int sceneIndex = SceneManager.GetActiveScene().buildIndex;
			loadingManager.LoadWorldScene(sceneIndex, MetaData.LoadType.Load);
			WriteResponse(command, true, $"Saved-world reload requested for build index {sceneIndex}.");
		}

		private static SaveProcessor ResolveSaveProcessor()
		{
			Container projectContainer = Container.ProjectContainer;
			if (projectContainer != null && projectContainer.HasBinding(typeof(SaveProcessor)))
				return projectContainer.Resolve<SaveProcessor>();

			return FindRuntimeComponent<SaveProcessor>();
		}

		private static T FindRuntimeComponent<T>() where T : Component
		{
			T active = UnityEngine.Object.FindAnyObjectByType<T>();
			if (active != null)
				return active;

			T[] all = Resources.FindObjectsOfTypeAll<T>();
			for (int i = 0; i < all.Length; i++)
			{
				T candidate = all[i];
				if (candidate != null && candidate.gameObject.scene.IsValid())
					return candidate;
			}

			return null;
		}

		private static void BeginFrameCapture(string command, double synchronousMilliseconds, long monoMemoryDeltaBytes)
		{
			_frameCapture = new FrameCapture
			{
				Command = command,
				SynchronousMilliseconds = synchronousMilliseconds,
				MonoMemoryDeltaBytes = monoMemoryDeltaBytes,
				LastFrame = Time.frameCount,
				FrameMilliseconds = new List<float>(FramesToCapture)
			};
		}

		private static void CaptureFrameTiming()
		{
			if (_frameCapture == null || !EditorApplication.isPlaying)
				return;

			if (Time.frameCount == _frameCapture.LastFrame)
				return;

			_frameCapture.LastFrame = Time.frameCount;
			_frameCapture.FrameMilliseconds.Add(Time.unscaledDeltaTime * 1000f);
			if (_frameCapture.FrameMilliseconds.Count < FramesToCapture)
				return;

			float maximumFrameMilliseconds = 0f;
			for (int i = 0; i < _frameCapture.FrameMilliseconds.Count; i++)
				maximumFrameMilliseconds = Mathf.Max(maximumFrameMilliseconds, _frameCapture.FrameMilliseconds[i]);

			WriteResponse(
				_frameCapture.Command,
				true,
				_frameCapture.Command == "capture-frames" ? "Baseline frames captured." : "Debug character spawned and subsequent frames captured.",
				_frameCapture.SynchronousMilliseconds,
				_frameCapture.MonoMemoryDeltaBytes,
				maximumFrameMilliseconds,
				_frameCapture.FrameMilliseconds.ToArray());
			_frameCapture = null;
		}

		private static string GetEditorStatus()
		{
			Scene scene = SceneManager.GetActiveScene();
			List<string> details = new List<string>
			{
				$"Playing={EditorApplication.isPlaying}",
				$"Paused={EditorApplication.isPaused}",
				$"Compiling={EditorApplication.isCompiling}",
				$"Scene={scene.name}",
				$"TimeScale={Time.timeScale:F2}"
			};

			TwitchConnectionPanelView connectionPanel = FindRuntimeComponent<TwitchConnectionPanelView>();
			details.Add($"ConnectionPanel={(connectionPanel != null && connectionPanel.IsVisible ? "visible" : "hidden")}");
			if (connectionPanel != null && connectionPanel.IsVisible)
				details.Add($"ConnectionCommand={connectionPanel.DisplayedCommand}");

			Camera[] cameras = Resources.FindObjectsOfTypeAll<Camera>();
			int runtimeCameras = 0;
			int activeCameras = 0;
			List<string> activeCameraNames = new List<string>();
			for (int i = 0; i < cameras.Length; i++)
			{
				Camera camera = cameras[i];
				if (camera == null || !camera.gameObject.scene.IsValid())
					continue;

				runtimeCameras++;
				if (camera.isActiveAndEnabled)
				{
					activeCameras++;
					activeCameraNames.Add($"{camera.name}@{camera.gameObject.scene.name}");
				}
			}
			details.Add($"Cameras={activeCameras}/{runtimeCameras}");
			details.Add($"ActiveCameras={string.Join(",", activeCameraNames)}");
			details.Add($"MainCamera={(Camera.main != null ? Camera.main.name : "null")}");

			UnitTextDisplay label = UnityEngine.Object.FindAnyObjectByType<UnitTextDisplay>();
			if (label != null && Camera.main != null)
			{
				Vector3 cameraToLabel = label.transform.position - Camera.main.transform.position;
				float labelAngle = cameraToLabel.sqrMagnitude > Mathf.Epsilon
					? Quaternion.Angle(label.transform.rotation, Quaternion.LookRotation(cameraToLabel, Camera.main.transform.up))
					: 0f;
				details.Add($"LabelFacingAngle={labelAngle:F2}");
			}

			AstarPathComponent astar = AstarPathComponent.active;
			int graphCount = 0;
			if (astar?.data?.graphs != null)
			{
				for (int i = 0; i < astar.data.graphs.Length; i++)
				{
					if (astar.data.graphs[i] != null)
						graphCount++;
				}
			}
			details.Add($"Astar={(astar != null ? "present" : "null")}");
			details.Add($"Graphs={graphCount}");

			Container projectContainer = Container.ProjectContainer;
			if (projectContainer != null)
			{
				if (projectContainer.HasBinding(typeof(Twitch.TwitchClientProcessor)))
				{
					Twitch.TwitchClientProcessor twitchClient = projectContainer.Resolve<Twitch.TwitchClientProcessor>();
					details.Add($"TwitchClient={(twitchClient.IsConnected ? "connected" : twitchClient.IsConnecting ? "connecting" : "disconnected")}");
					details.Add($"TwitchObjectActiveSelf={twitchClient.gameObject.activeSelf}");
					details.Add($"TwitchObjectActiveInHierarchy={twitchClient.gameObject.activeInHierarchy}");
					details.Add($"TwitchObjectScene={twitchClient.gameObject.scene.name}");
					details.Add($"TwitchObjectEntityId={twitchClient.GetEntityId()}");
					details.Add($"TwitchChannel={twitchClient.ChannelName}");
					details.Add($"TwitchJoinedChannels={twitchClient.JoinedChannelCount}");
					details.Add($"TwitchStatus={twitchClient.ConnectionStatus}");
				}

				if (projectContainer.HasBinding(typeof(TwitchChatProcessor)))
				{
					TwitchChatProcessor chat = projectContainer.Resolve<TwitchChatProcessor>();
					details.Add($"LastTwitchCommand={chat.LastCommand}");
					details.Add($"LastTwitchCommandUser={chat.LastCommandUser}");
					details.Add($"LastTwitchCommandResult={chat.LastCommandResult}");
				}

				if (projectContainer.HasBinding(typeof(PlayerProcessor)))
				{
					PlayerProcessor players = projectContainer.Resolve<PlayerProcessor>();
					details.Add($"Players={players.PlayerCount()}");
					details.Add($"Recruits={players.RecruitCount()}");
					details.Add($"UserPlayer={players.UserPlayer?.TwitchUser?.Username ?? "null"}");
					details.Add($"Ruler={players.GetRuler()?.TwitchUser?.Username ?? "null"}");
					List<string> recruitRoles = new List<string>();
					for (int i = 0; i < players.Recruits.Count; i++)
						recruitRoles.Add(players.Recruits[i]?.RoleHandler?.CurrentRole.ToString() ?? "null");
					details.Add($"RecruitRoles={string.Join(",", recruitRoles)}");
				}

				Units.HealthHandler[] healthHandlers = UnityEngine.Object.FindObjectsByType<Units.HealthHandler>(
					FindObjectsInactive.Exclude, FindObjectsSortMode.None);
				int playerHealthBars = 0;
				int monsterHealthBars = 0;
				int buildingHealthBars = 0;
				for (int i = 0; i < healthHandlers.Length; i++)
				{
					Units.HealthHandler handler = healthHandlers[i];
					if (handler.GetComponent<Character.RoleHandler>() != null && handler.GetComponentInChildren<Units.UnitHealthBar>(true) != null)
						playerHealthBars++;
					if (handler.GetComponent<Enemies.Enemy>() != null && handler.GetComponentInChildren<Units.UnitHealthBar>(true) != null)
						monsterHealthBars++;
					if (handler.GetComponent<Buildings.BuildingBase>() != null && handler.GetComponentInChildren<UserInterface_BuildingHealthBar>(true) != null)
						buildingHealthBars++;
				}
				details.Add($"HealthBars={playerHealthBars}/{monsterHealthBars}/{buildingHealthBars}");

				Enemies.Enemy[] enemies = UnityEngine.Object.FindObjectsByType<Enemies.Enemy>(
					FindObjectsInactive.Exclude, FindObjectsSortMode.None);
				for (int i = 0; i < enemies.Length; i++)
				{
					Enemies.Enemy enemy = enemies[i];
					if (enemy == null || enemy.EnemyType != Utils.EnemyType.Blargul)
						continue;

					Renderer[] renderers = enemy.GetComponentsInChildren<Renderer>(true);
					float minimumY = float.PositiveInfinity;
					float maximumY = float.NegativeInfinity;
					for (int rendererIndex = 0; rendererIndex < renderers.Length; rendererIndex++)
					{
						Renderer renderer = renderers[rendererIndex];
						if (renderer == null || !renderer.enabled)
							continue;

						minimumY = Mathf.Min(minimumY, renderer.bounds.min.y);
						maximumY = Mathf.Max(maximumY, renderer.bounds.max.y);
					}

					float rootY = enemy.transform.position.y;
					string bounds = float.IsInfinity(minimumY)
						? "no-renderer"
						: $"{minimumY - rootY:F3}/{maximumY - rootY:F3}";
					STStateMachine.StateMachine stateMachine = enemy.GetComponent<STStateMachine.StateMachine>();
					Pathfinding.AIPath aiPath = enemy.GetComponent<Pathfinding.AIPath>();
					details.Add(
						$"BlargulPose={enemy.name}:id={enemy.GetEntityId()},root={rootY:F3},relativeBounds={bounds}," +
						$"state={stateMachine?.CurrentState?.GetType().Name ?? "null"},health={enemy.HealthHandler?.Health ?? -1}," +
						$"path={aiPath?.position.y ?? float.NaN:F3}/{aiPath?.canMove ?? false}/{aiPath?.enabled ?? false}");
					break;
				}

				if (projectContainer.HasBinding(typeof(ResourceProcessor)))
				{
					ResourceProcessor resources = projectContainer.Resolve<ResourceProcessor>();
					details.Add($"Resources={resources.WoodResources.Count}/{resources.OreResources.Count}/{resources.FoodResources.Count}/{resources.GoldResources.Count}/{resources.RecruitResources.Count}");
				}

				if (projectContainer.HasBinding(typeof(FoliageProcessor)))
				{
					FoliageProcessor foliage = projectContainer.Resolve<FoliageProcessor>();
					details.Add($"Foliage={foliage.GetOnLandFoliage().Count}/{foliage.GetUnderWaterFoliage().Count}");
				}

				if (projectContainer.HasBinding(typeof(TechTreeProcessor)))
				{
					TechTreeProcessor techTree = projectContainer.Resolve<TechTreeProcessor>();
					try
					{
						details.Add($"UnlockedTechs={string.Join(",", techTree.GetUnlockedTechIds())}");
						details.Add($"CurrentTech={techTree.CurrentTech?.TechName ?? "null"}");
					}
					catch (Exception)
					{
						details.Add("TechTree=not-initialized");
					}
				}

				if (projectContainer.HasBinding(typeof(GameEventProcessor)))
				{
					GameEventProcessor events = projectContainer.Resolve<GameEventProcessor>();
					GameEvent current = events.CurrentEvent;
					details.Add($"CurrentEvent={(current != null ? current.Event.ToString() : "null")}");
					details.Add($"QueuedEvents={events.EventQueue.Count}");
					details.Add($"RulerVoteTimer={events.TimeTillRulerVote:F1}");
					details.Add($"RulerVoteArmed={events.CanStartNewRulerVote}");
					TechVote vote = current as TechVote;
					if (vote == null)
					{
						foreach (GameEvent queued in events.EventQueue)
						{
							if (queued is TechVote queuedVote)
							{
								vote = queuedVote;
								break;
							}
						}
					}

					if (vote != null)
					{
						List<string> voteTechs = new List<string>();
						foreach (VoteOption option in vote.Options.Values)
						{
							if (option?.OptionData is TechNodeData techData)
								voteTechs.Add(techData.TechName);
						}
						details.Add($"TechVote={string.Join(",", voteTechs)}");
						details.Add($"TechVoteVotes={vote.PlayerVotes.Count}");
					}
				}
			}

			return string.Join("; ", details);
		}

		private static void WriteResponse(
			string command,
			bool success,
			string message,
			double synchronousMilliseconds = 0d,
			long monoMemoryDeltaBytes = 0L,
			float maximumFrameMilliseconds = 0f,
			float[] frameMilliseconds = null)
		{
			var response = new SessionResponse
			{
				Command = command,
				Success = success,
				Message = message,
				UtcTimestamp = DateTime.UtcNow.ToString("O"),
				SynchronousMilliseconds = synchronousMilliseconds,
				MonoMemoryDeltaBytes = monoMemoryDeltaBytes,
				MaximumSubsequentFrameMilliseconds = maximumFrameMilliseconds,
				SubsequentFrameMilliseconds = frameMilliseconds ?? Array.Empty<float>()
			};

			Directory.CreateDirectory(Path.GetDirectoryName(ResponsePath));
			File.WriteAllText(ResponsePath, JsonUtility.ToJson(response, true));
			UnityEngine.Debug.Log($"[SessionBridge] {command}: {(success ? "success" : "failed")} - {message}");
		}

		private sealed class FrameCapture
		{
			public string Command;
			public double SynchronousMilliseconds;
			public long MonoMemoryDeltaBytes;
			public int LastFrame;
			public List<float> FrameMilliseconds;
		}

		[Serializable]
		private sealed class SessionResponse
		{
			public string Command;
			public bool Success;
			public string Message;
			public string UtcTimestamp;
			public double SynchronousMilliseconds;
			public long MonoMemoryDeltaBytes;
			public float MaximumSubsequentFrameMilliseconds;
			public float[] SubsequentFrameMilliseconds;
		}
	}
}
#endif
