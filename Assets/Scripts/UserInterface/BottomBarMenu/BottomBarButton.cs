using System.Collections;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.UI;

namespace UserInterface.BottomBarMenu 
{

	public enum BottomBarContext
	{
		BuildMenu,
		RecruitMenu,
		TechTreeMenu
	}
    public class BottomBarButton : MonoBehaviour
	{
		private BottomBarInterface _bottomBarInterface;
		[SerializeField]
		private Image _arrowImage;

		[SerializeField]
		private BottomBarContext _buttonContext;

		protected bool _active = false;

		public bool Active => _active;

		public BottomBarContext ButtonContext => _buttonContext;

        private void Awake()
		{
			_bottomBarInterface = GetComponentInParent<BottomBarInterface>(true);
		}

        public void ToggleContext()
		{
			if (_bottomBarInterface == null)
				return;

			if (!_active)
				_bottomBarInterface.OnActivatedButton(this);
			else
				_bottomBarInterface.OnDeactivatedButton(this);
		}

		public void OnContextHidden()
		{
			_arrowImage.enabled = false;
			_active = false;
		}

		public void OnContextShown()
		{
			_active = true;
			_arrowImage.enabled = true;
		}

		private void OnEnable()
		{
			StartCoroutine(DelayedAddButton());
		}

		private void OnDisable()
		{
			if (_bottomBarInterface == null)
				return;

			_bottomBarInterface.RemoveButton(this);
		}

		private IEnumerator DelayedAddButton()
		{
			yield return new WaitForSeconds(0.01f);
			if (_bottomBarInterface == null)
				yield break;

			_bottomBarInterface.AddButton(this);
			OnContextHidden();
		}
	}
}
