#if UNITY_EDITOR
using UnityEditor;
using UnityEngine;
using Processors;

[CustomEditor(typeof(SeasonProcessor))]
public class SeasonProcessorEditor : Editor
{
	private SeasonProcessor _targetProcessor;

	private void OnEnable()
	{
		_targetProcessor = (SeasonProcessor)target;
	}

	public override void OnInspectorGUI()
	{
		base.OnInspectorGUI();
		GUILayout.Space(10);
		GUILayout.BeginHorizontal();
		{
			GUILayout.FlexibleSpace();
			if (GUILayout.Button("Next Season"))
			{
				_targetProcessor.ForceSetNextSeason();
			}
			if (GUILayout.Button("Update Current"))
			{
				_targetProcessor.UpdateCurrentSeason();
			}
			GUILayout.FlexibleSpace();
		}
		GUILayout.EndHorizontal();

		GUILayout.BeginHorizontal();
		{
			GUILayout.FlexibleSpace();
			GUILayout.FlexibleSpace();
		}
		GUILayout.EndHorizontal();
		GUILayout.Label($"Current Season: {_targetProcessor.GetCurrentSeason()}");
	}
}
#endif
