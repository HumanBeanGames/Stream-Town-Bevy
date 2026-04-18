using UnityEditor;
using UnityEngine;
using Processors;

namespace World.Generation
{
	[CustomEditor(typeof(WorldGenProcessor))]
	public class ProceduralTerrainEditor : Editor
	{
		WorldGenProcessor _t;

		private void OnEnable()
		{
			_t = (WorldGenProcessor)target;
		}

		public override void OnInspectorGUI()
		{
			base.OnInspectorGUI();

			GUILayout.Space(10);
			GUILayout.BeginHorizontal();
			{
				GUILayout.FlexibleSpace();
				if (GUILayout.Button("Generate"))
				{
					_t.GenerateTerrain();
				}
				GUILayout.FlexibleSpace();
			}
			GUILayout.EndHorizontal();

			GUILayout.BeginHorizontal();
			{
				GUILayout.FlexibleSpace();
				if (GUILayout.Button("Generate Fake Resources"))
				{
					_t.MainMenuGenerateWorld();
				}
				GUILayout.FlexibleSpace();
			}
			GUILayout.EndHorizontal();

			GUILayout.Space(10);
			EditorGUILayout.LabelField("Runtime Regeneration", EditorStyles.boldLabel);
			EditorGUILayout.HelpBox("These buttons are intended for Play Mode testing.", MessageType.Info);

			using (new EditorGUI.DisabledScope(!Application.isPlaying))
			{
				if (GUILayout.Button("Regenerate Terrain + World (Runtime)"))
				{
					_t.RegenerateTerrainAndWorldRuntime();
				}

				if (GUILayout.Button("Regenerate Resources + Foliage (Runtime)"))
				{
					_t.RegenerateResourcesAndFoliageRuntime();
				}
			}
		}
	}
}
