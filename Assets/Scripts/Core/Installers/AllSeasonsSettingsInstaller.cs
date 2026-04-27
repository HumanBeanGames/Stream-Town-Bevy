using Reflex.Core;
using System;
using UnityEngine;

namespace ScriptablesProcessorInfrastructure
{
	/// <summary>
	/// MonoBehaviour wrapper for AllSeasonSettings that implements IInstaller.
	/// References the serialized asset created in-editor.
	/// </summary>
	public class AllSeasonsSettingsInstaller : MonoBehaviour, IInstaller
	{
		[SerializeField]
		private AllSeasonSettings _allSeasonSettings;

		public AllSeasonSettings AllSeasonSettings => _allSeasonSettings;

		public void InstallBindings(ContainerBuilder containerBuilder)
		{
			if (_allSeasonSettings == null)
				throw new InvalidOperationException($"{nameof(AllSeasonsSettingsInstaller)} on '{gameObject.name}' is missing an {nameof(AllSeasonSettings)} reference.");

			containerBuilder.AddSingleton(_allSeasonSettings);
		}
	}
}
