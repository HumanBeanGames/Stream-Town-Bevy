using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TradeSettingsScriptable that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TradeSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TradeSettingsScriptable _tradeSettingsScriptable;

		public TradeSettingsScriptable TradeSettingsScriptable => _tradeSettingsScriptable;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
