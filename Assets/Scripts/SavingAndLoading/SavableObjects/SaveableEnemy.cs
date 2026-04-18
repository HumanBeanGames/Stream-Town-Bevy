using Enemies;
using GUIDSystem;
using Processors;
using Reflex.Attributes;
using SavingAndLoading.Structs;
using Target;
using Utils;
using Utils.Pooling;

namespace SavingAndLoading.SavableObjects 
{
    /// <summary>
    /// Handles saving and loading for enemies.
    /// </summary>
    public class SaveableEnemy : SaveableObject
	{
        /// <summary>
        /// Gets or sets the enemy.
        /// </summary>
        public Enemy Enemy { get; set; }

        /// <summary>
        /// The GUID processor. Injected via Reflex dependency injection.
        /// </summary>
		[Inject] private GUIDProcessor _guidProcessor;

        /// <summary>
        /// Saves the enemy data.
        /// </summary>
        /// <returns>The enemy save data.</returns>
		public override object SaveData()
		{
			return (object)new EnemySaveData(Enemy.transform, Enemy.EnemyType.ToString(), Enemy.HealthHandler.Health, _guidProcessor.CreateGUIDandAddToDictionary(PoolableObject));
		}

        /// <summary>
        /// Loads the enemy data.
        /// </summary>
        /// <param name="data">The enemy save data.</param>
		public override void LoadData(object data)
		{
			EnemySaveData enemyData = (EnemySaveData)data;
			Enemy.transform.position = Vector3SaveData.ToUnityVec3(enemyData.Transform.Position);
			Enemy.transform.eulerAngles = Vector3SaveData.ToUnityVec3(enemyData.Transform.Rotation);
			Enemy.transform.localScale = Vector3SaveData.ToUnityVec3(enemyData.Transform.LossyScale);
			Enemy.gameObject.SetActive(true);
			Enemy.HealthHandler.SetHealth(enemyData.Health);

			GUIDComponent.SetGUID(enemyData.GUID);
			_guidProcessor.AddToDictionary(PoolableObject);
		}

        /// <summary>
        /// Sets the enemy variables.
        /// </summary>
        /// <param name="target">The targetable object.</param>
        /// <param name="component">The GUID component.</param>
        /// <param name="poolName">The pool name.</param>
        /// <param name="poolableObject">The poolable object.</param>
        /// <param name="enemy">The enemy.</param>
		public void SetVariables(Targetable target, GUIDComponent component, string poolName, PoolableObject poolableObject, Enemy enemy)
		{
			Enemy = enemy;
			base.SetVariables(target, component, poolName, poolableObject);
		}
	}
}
