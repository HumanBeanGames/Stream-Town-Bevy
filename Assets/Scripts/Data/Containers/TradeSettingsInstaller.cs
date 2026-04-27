using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for TradeSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class TradeSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private TradeSettings _tradeSettings;

		public TradeSettings TradeSettings => _tradeSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(_tradeSettings);
		}
	}
}
