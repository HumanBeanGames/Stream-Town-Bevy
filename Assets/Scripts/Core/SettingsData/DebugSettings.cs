using UnityEngine;
using UserInterface;
using System.Collections.Generic;
using Processors;
using Sirenix.OdinInspector;
using Sirenix.Serialization;

#if UNITY_EDITOR
using UnityEditor;
#endif

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores debug system settings for the game.
	/// Contains references to debug UI components.
	/// </summary>
	[CreateAssetMenu(fileName = "DebugSettings", menuName = "Scriptables/Debug Settings")]
	public class DebugSettings : SerializedScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Reference to the debug user interface component.
		/// Used to display debug information and controls.
		/// </summary>
		[SerializeField]
		private UserInterface_Debug _debugUI;

		[OdinSerialize]
		[DictionaryDrawerSettings(IsReadOnly = false)]
		private Dictionary<DebugLogCategory, bool> _categoryFilters = new Dictionary<DebugLogCategory, bool>();

		private static DebugSettings _activeInstance;

		/// <summary>
		/// Gets the debug user interface component.
		/// </summary>
		public UserInterface_Debug DebugUI => _debugUI;

		public static DebugSettings ActiveInstance
		{
			get
			{
				if (_activeInstance != null)
					return _activeInstance;

				DebugSettings[] loadedSettings = Resources.FindObjectsOfTypeAll<DebugSettings>();
				if (loadedSettings.Length > 0)
					_activeInstance = loadedSettings[0];

				return _activeInstance;
			}
		}

		private void OnEnable()
		{
			_activeInstance = this;
			EnsureAllCategoriesRegistered();
		}

		private void EnsureAllCategoriesRegistered()
		{
			bool changed = false;

			foreach (DebugLogCategory category in System.Enum.GetValues(typeof(DebugLogCategory)))
			{
				if (_categoryFilters.ContainsKey(category))
					continue;

				_categoryFilters[category] = true;
				changed = true;
			}

			if (changed)
				MarkDirty();
		}

		public void RegisterCategory(DebugLogCategory category)
		{
			if (_categoryFilters.ContainsKey(category))
				return;

			_categoryFilters[category] = true;
			MarkDirty();
		}

		public bool ShouldPublish(DebugLogCategory category)
		{
			return IsCategoryEnabled(category);
		}

		public bool IsCategoryEnabled(DebugLogCategory category)
		{
			return !_categoryFilters.TryGetValue(category, out bool enabled) || enabled;
		}

		private void MarkDirty()
		{
		#if UNITY_EDITOR
			EditorUtility.SetDirty(this);
		#endif
		}
	}
}
