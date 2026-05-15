using UnityEngine;

using System;

namespace STStateMachine
{
	/// <summary>
	/// Base class for all States in Stream Town.
	/// </summary>
	public class STStateBase : MonoBehaviour
	{
		protected StateMachine _stateMachine;
		private bool _initialized = false;

		/// <summary>
		/// Called when a state is entered.
		/// </summary>
		public virtual void OnEnter()
		{
			InitializeIfNeeded();
		}

		/// <summary>
		/// Called every frame when a state is updated.
		/// </summary>
		public virtual void OnUpdate()
		{

		}

		/// <summary>
		/// Called when a state is exited.
		/// </summary>
		public virtual void OnExit()
		{

		}

		/// <summary>
		/// Called when a state is Initialized.
		/// </summary>
		protected virtual void OnInit()
		{

		}

		private void InitializeIfNeeded()
		{
			if (_initialized)
				return;

			ResolveStateMachine();
			OnInit();
			_initialized = true;
		}

		private void ResolveStateMachine()
		{
			if (_stateMachine != null)
				return;

			_stateMachine = GetComponentInParent<StateMachine>();
			if (_stateMachine == null)
				throw new InvalidOperationException($"{GetType().Name} on '{gameObject.name}' could not find a parent StateMachine. State components must live on the StateMachine GameObject or one of its children.");
		}

		private void Awake()
		{
			InitializeIfNeeded();
		}
	}
}
