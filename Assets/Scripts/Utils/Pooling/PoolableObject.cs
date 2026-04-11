using Buildings;
using Character;
using Enemies;
using GameResources;
using GUIDSystem;
using Managers;
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
		private ObjectPoolingManager _poolingManager;
		[Inject] private GUIDManager _guidManager;

		[SerializeField]
		private string _poolName;
		private object _saveableObject;
		[SerializeField]
		private PoolType _poolType;

		public PoolType PoolType => _poolType;
		public object SaveableObject
		{
			get { return _saveableObject; }
			set { _saveableObject = value; }
		}
		public string PoolName
		{
			get { return _poolName; }
			set { _poolName = value; }
		}

		public void Initialize(string name, ObjectPoolingManager poolingManager)
		{
			_poolName = name;
			_poolingManager = poolingManager;
			SetupSaveableObject();
		}

		public void OnReset()
		{
			// Default implementation does nothing
		}

		public void SetupSaveableObject()
		{
			switch (_poolType)
			{
				case PoolType.Other:
					break;
				case PoolType.Resource:
					SaveableObject = (object)new SaveableResource();
					((SaveableResource)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<ResourceHolder>());
					break;
				case PoolType.Enemy:
					SaveableObject = (object)new SaveableEnemy();
					((SaveableEnemy)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<Enemy>());
					break;
				case PoolType.Player:
					SaveableObject = (object)new SaveablePlayer();
					((SaveablePlayer)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<RoleHandler>());
					break;
				case PoolType.Building:
					SaveableObject = (object)new SaveableBuilding();
					((SaveableBuilding)SaveableObject).SetVariables(gameObject.GetComponent<Targetable>(), gameObject.GetComponent<GUIDComponent>(), _poolName, this, GetComponent<BuildingBase>());
					break;				
				case PoolType.Foliage:
					SaveableObject = (object)new SaveablFoliage();
					((SaveablFoliage)SaveableObject).SetVariables(_poolName, this);
					break;
			}
		}

		private void OnEnable()
		{
			if (_guidManager != null && _saveableObject != null && ((SaveableObject)_saveableObject).GUIDComponent != null)
				_guidManager.CreateGUIDandAddToDictionary(this);
		}

		private void OnDisable()
		{
			if (_guidManager != null && _saveableObject != null && ((SaveableObject)_saveableObject).GUIDComponent !=null)
				_guidManager.RemoveFromGUID(PoolType, ((SaveableObject)_saveableObject).GUIDComponent.GUID);

			if (_poolingManager != null)
				_poolingManager.AddToPool(_poolName, this);
		}
	}
}