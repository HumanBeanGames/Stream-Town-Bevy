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
			Initialize(_stateMachine);
		}

		/// <summary>
		/// Establishes explicit ownership before a StateMachine enters this state.
		/// Hierarchy discovery remains a fallback for direct state calls.
		/// </summary>
		internal void Initialize(StateMachine stateMachine)
		{
			if (stateMachine == null)
				throw new ArgumentNullException(nameof(stateMachine));

			if (_initialized)
			{
				if (_stateMachine != stateMachine)
					throw new InvalidOperationException($"{GetType().Name} on '{gameObject.name}' is already owned by another StateMachine.");
				return;
			}

			_stateMachine = stateMachine;
			OnInit();
			_initialized = true;
		}

		private void ResolveStateMachine()
		{
			if (_stateMachine != null)
				return;

			// Resolve the common case explicitly and include inactive parents so
			// pooled activation order cannot affect state-machine discovery.
			_stateMachine = GetComponent<StateMachine>();
			if (_stateMachine == null)
				_stateMachine = GetComponentInParent<StateMachine>(true);
			if (_stateMachine == null)
				throw new InvalidOperationException($"{GetType().Name} on '{gameObject.name}' could not find a parent StateMachine. State components must live on the StateMachine GameObject or one of its children.");
		}

		private void Awake()
		{
			// Do not fail during pooled prefab activation if hierarchy discovery is
			// temporarily unavailable. The owning StateMachine binds the state before
			// its first transition.
			_stateMachine = GetComponent<StateMachine>();
			if (_stateMachine == null)
				_stateMachine = GetComponentInParent<StateMachine>(true);

			if (_stateMachine != null)
				Initialize(_stateMachine);
		}
	}
}
