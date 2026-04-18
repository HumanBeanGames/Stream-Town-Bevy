using UnityEditor;
using UnityEngine;
// System may be obsolete.
namespace GridSystem
{
	[CustomEditor(typeof(GridProcessor))]
	public class GridSystemEditor : Editor
	{
		public override void OnInspectorGUI()
		{
			base.OnInspectorGUI();
			GUILayout.Space(10);
			EditorGUILayout.HelpBox("Generate Grid button disabled - GridProcessor now uses dependency injection and requires runtime initialization.", MessageType.Info);
		}
	}
}
