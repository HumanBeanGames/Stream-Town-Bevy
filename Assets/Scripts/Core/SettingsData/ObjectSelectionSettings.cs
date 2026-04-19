using UnityEngine;
using UserInterface;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// ScriptableObject that stores object selection system settings for the game.
	/// Contains references to the object selection UI component.
	/// </summary>
	[CreateAssetMenu(fileName = "ObjectSelectionSettings", menuName = "Scriptables/Object Selection Settings")]
	public class ObjectSelectionSettings : ScriptableObject, IDataScriptable
	{
		/// <summary>
		/// Reference to the object selection user interface component.
		/// Used to display selection feedback and controls.
		/// </summary>
		[SerializeField]
		private UserInterface_ObjectSelection _selectionUI;

		/// <summary>
		/// Gets the object selection user interface component.
		/// </summary>
		public UserInterface_ObjectSelection SelectionUI => _selectionUI;
	}
}
