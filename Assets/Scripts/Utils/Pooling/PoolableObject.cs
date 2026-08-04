using Buildings;
using Character;
using Enemies;
using GameResources;
using GUIDSystem;
using Processors;
using SavingAndLoading.SavableObjects;
using Target;
using Units;
using UnityEngine;
using Reflex.Attributes;

namespace Utils.Pooling
{
	public enum PoolType
	{
		Other,
		Resource,
		Enemy,
		Player,
		Building,
		Foliage,
		Count
	}

	public class PoolableObject : MonoBehaviour, IPooledObjectReset
	{
		[Inject] private ObjectPoolingProcessor _poolingProcessor;
		[Inject] private GUIDProcessor _guidProcessor;

		[SerializeField]
		private string _poolName;
		private SaveableObject _saveableObject;
		[SerializeField]
		private PoolType _poolType;

		private bool _isReturningToPool = false;

		public PoolType PoolType => _poolType;
		public SaveableObject SaveableObject
		{
			get { return _saveableObject; }
			set { _saveableObject = value; }
		}
		public string PoolName
		{
			get { return _poolName; }
			set { _poolName = value; }
		}

		public void SetReturningToPool(bool returning)
		{
			_isReturningToPool = returning;
		}

		public void Initialize(string name)
		{
			_poolName = name;
			SetupSaveableObject();
		}

		public void OnReset()
		{
			// Reset the returning to pool flag when the object is reused
			_isReturningToPool = false;

			// Call OnReset on all IPooledObjectReset components on this GameObject
			// This ensures components like TargetableBuilding also get their OnReset called
			IPooledObjectReset[] resettables = GetComponents<IPooledObjectReset>();
			foreach (var resettable in resettables)
			{
				if (!ReferenceEquals(resettable, this)) // Don't call OnReset on ourselves again
					resettable.OnReset();
			}
		}

		public void SetupSaveableObject()
		{
			switch (_poolType)
			{
				case PoolType.Other:
					GUIDComponent otherGuid = GetComponent<GUIDComponent>();
					HealthHandler otherHealth = GetComponent<HealthHandler>();
					if (otherGuid != null && otherHealth != null)
					{
						SaveableObject = new SaveableEnemyCamp();
						((SaveableEnemyCamp)SaveableObject).SetVariables(GetComponent<Targetable>(), otherGuid, _poolName, this, otherHealth);
					}
					break;
				case PoolType.Resource:
					SaveableObject = new SaveableResource();
					((SaveableResource)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<ResourceHolder>());
					break;
				case PoolType.Enemy:
					SaveableObject = new SaveableEnemy();
					((SaveableEnemy)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<Enemy>());
					break;
				case PoolType.Player:
					SaveableObject = new SaveablePlayer();
					((SaveablePlayer)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<RoleHandler>());
					break;
				case PoolType.Building:
					SaveableObject = new SaveableBuilding();
					((SaveableBuilding)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<BuildingBase>());
					break;				
				case PoolType.Foliage:
					SaveableObject = new SaveablFoliage();
					((SaveablFoliage)SaveableObject).SetVariables(_poolName, this);
					break;
			}
		}

		private void OnEnable()
		{
			if (_saveableObject != null && _saveableObject.GUIDComponent != null)
				_guidProcessor.CreateGUIDandAddToDictionary(this);
		}

		private void OnDisable()
		{
			if (_saveableObject != null && _saveableObject.GUIDComponent !=null && _guidProcessor != null)
				_guidProcessor.RemoveFromGUID(PoolType, _saveableObject.GUIDComponent.GUID);

			// Only return to pool if not already being returned (prevents double-adding when AddToPool disables the object)
			if (_poolingProcessor != null && !_isReturningToPool && !string.IsNullOrWhiteSpace(_poolName))
				_poolingProcessor.AddToPool(_poolName, this);
		}
	}
}
