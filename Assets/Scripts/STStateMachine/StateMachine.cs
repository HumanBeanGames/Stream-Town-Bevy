using STStateMachine.Helpers;
using System.Collections.Generic;
using UnityEngine;

namespace STStateMachine
{
	/// <summary>
	/// State Machine that handles the switching of states.
	/// </summary>
	public class StateMachine : MonoBehaviour
	{
		//TODO:: Potentially add all common components here
		public List<STStateHolder> _states = new List<STStateHolder>();
		public List<STSM_HelperBase> _helpers = new List<STSM_HelperBase>();

		[SerializeField]
		private STStateBase _globalState;
		[SerializeField]
		private STStateBase _startingState;

		private STStateBase _previousState;
		[SerializeField]
		private STStateBase _currentState;

		private bool _interruptable = true;

		[SerializeField]
		private bool _debugMessages = true;

		// Per-character resource target data (to avoid sharing state data across characters)
		private uint _resourceTargetGUID;
		private Utils.Resource _resourceTargetType = Utils.Resource.None;
		private bool _hasResourceTarget = false;

		// Properties.
		public List<STStateHolder> States => _states;
		public STStateBase CurrentState => _currentState;
		public STStateBase PreviousState => _previousState;
		public STStateBase GlobalState => _globalState;

		// Resource target properties
		public uint ResourceTargetGUID => _resourceTargetGUID;
		public Utils.Resource ResourceTargetType => _resourceTargetType;
		public bool HasResourceTarget => _hasResourceTarget;

		/// <summary>
		/// Sets the resource target for this character's state machine.
		/// </summary>
		/// <param name="guid">GUID of the resource.</param>
		/// <param name="resourceType">Type of resource.</param>
		public void SetResourceTarget(uint guid, Utils.Resource resourceType)
		{
			_resourceTargetGUID = guid;
			_resourceTargetType = resourceType;
			_hasResourceTarget = true;
		}

		/// <summary>
		/// Clears the resource target for this character's state machine.
		/// </summary>
		public void ClearResourceTarget()
		{
			_resourceTargetGUID = 0;
			_resourceTargetType = Utils.Resource.None;
			_hasResourceTarget = false;
		}

		/// <summary>
		/// Requests the state machine to swap to the desired state.
		/// </summary>
		/// <param name="state"></param>
		/// <param name="forceInterrupt"></param>
		public void RequestStateChange(STStateBase state, bool forceInterrupt = false)
		{
			if (!_interruptable && !forceInterrupt)
				return;

			if (state == null)
				return;

			// Exit Current State.
			if (_currentState != null)
			{
				_currentState.OnExit();

				if (_debugMessages)
					Debug.Log($"{gameObject.name} exiting state: {_currentState.GetType().Name}");
			}

			// Set the previous state to the current state
			_previousState = _currentState;

			// Set the new current state
			_currentState = state;

			_currentState.OnEnter();

			if (_debugMessages)
				Debug.Log($"[ResourceGathering] {gameObject.name} entering state: {_currentState.GetType().Name}");
		}

		/// <summary>
		/// Requests the State Machine to switch the desired state by name.
		/// </summary>
		/// <param name="stateName"></param>
		public void RequestStateChange(string stateName)
		{
			RequestStateChange(GetStateByName(stateName));
		}

		/// <summary>
		/// Returns the STStageBase by a given name.
		/// </summary>
		/// <param name="stateName"></param>
		/// <returns></returns>
		public STStateBase GetStateByName(string stateName)
		{
			for (int i = 0; i < _states.Count; i++)
			{
				if (_states[i].StateName == stateName)
					return _states[i].State;
			}
			Debug.LogWarning($"[ResourceGathering] GetStateByName('{stateName}') not found!");
			return null;
		}

		/// <summary>
		/// Returns the STSM_HelperBase by the given name.
		/// </summary>
		/// <param name="helperName"></param>
		/// <returns></returns>
		public STSM_HelperBase GetHelperByName(string helperName)
		{
			for (int i = 0; i < _helpers.Count; i++)
			{
				if (_helpers[i].HelperName == helperName)
					return _helpers[i];
			}

			return null;
		}

		/// <summary>
		/// Invokes a helper's function.
		/// </summary>
		/// <param name="helper"></param>
		public void InvokeHelper(STSM_HelperBase helper)
		{
			helper.InvokeHelper();
		}

		/// <summary>
		/// Sets the global state of the State Machine.
		/// </summary>
		/// <param name="state"></param>
		public void SetGlobalState(STStateBase state)
		{
			_globalState = state;
		}

		/// <summary>
		/// Sets if the State can be interrupted or not.
		/// </summary>
		/// <param name="value"></param>
		public void SetInterruptable(bool value)
		{
			_interruptable = value;
		}

		// Unity Functions
		private void Start()
		{
			if (_startingState != null)
			{
				Debug.Log($"[ResourceGathering] Starting state: {_startingState.GetType().Name}");
				RequestStateChange(_startingState);
			}
			else
			{
				Debug.LogWarning($"[ResourceGathering] No starting state set!");
			}
		}

		public void Update()
		{
			if (_currentState)
				_currentState.OnUpdate();

			if (_globalState)
				_globalState.OnUpdate();
		}
	}

	// TODO:: Move this to its own class.
	[System.Serializable]
	public class STStateHolder
	{
		public string StateName;
		public STStateBase State;
	}
}
