using Reflex.Core;
using ScriptablesProcessorInfrastructure;
using UnityEngine;

namespace Data.Containers
{
	/// <summary>
	/// MonoBehaviour wrapper for FoliageGenSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class FoliageGenSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private FoliageGenSettings _foliageGenSettings;

		public FoliageGenSettings FoliageGenSettings => _foliageGenSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			containerBuilder.AddSingleton(this);
		}
	}
}
