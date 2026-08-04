using UnityEngine;
using UnityEngine.UI;

namespace Units
{
    /// <summary>
    /// A health bar that appears above a unit when damaged and hides when at full health.
    /// </summary>
    [DisallowMultipleComponent]
    public class UnitHealthBar : MonoBehaviour
    {
        [SerializeField]
        private Slider _healthBar;

        [SerializeField]
        private GameObject _displayUI;

        [SerializeField]
        private float _hideDelay = 3f;

        private HealthHandler _healthHandler;
        private float _hideTimer;
        private bool _wasDamaged = false;
        private Camera _mainCamera;

        public bool IsVisible => _displayUI != null && _displayUI.activeSelf;
        public float DisplayedHealth => _healthBar != null ? _healthBar.value : 0f;

        private void Awake()
        {
            _healthHandler = GetComponentInParent<HealthHandler>();
            if (_healthHandler == null)
            {
                Debug.LogWarning($"UnitHealthBar on {gameObject.name} could not find HealthHandler in parent.");
                enabled = false;
                return;
            }

            if (_healthBar == null)
                _healthBar = GetComponentInChildren<Slider>();

            if (_displayUI == null && transform.childCount > 0)
                _displayUI = transform.GetChild(0).gameObject;

            if (_displayUI == null || _healthBar == null)
                CreateFallbackHealthBar();

            _mainCamera = Camera.main;

            // Start hidden
            if (_displayUI != null)
                _displayUI.SetActive(false);
        }

        private void OnEnable()
        {
            if (_healthHandler != null)
            {
                _healthHandler.OnHealthChange += OnHealthChanged;
                _healthHandler.OnTookDamage += OnTookDamage;
            }
        }

        private void OnDisable()
        {
            if (_healthHandler != null)
            {
                _healthHandler.OnHealthChange -= OnHealthChanged;
                _healthHandler.OnTookDamage -= OnTookDamage;
            }
        }

        private void Start()
        {
            UpdateHealthBar();
        }

        private void LateUpdate()
        {
            // Billboarding - make the health bar face the camera
            if (_mainCamera == null)
                _mainCamera = Camera.main;

            if (_mainCamera != null && _displayUI != null && _displayUI.activeSelf)
            {
                Vector3 awayFromCamera = _displayUI.transform.position - _mainCamera.transform.position;
                if (awayFromCamera.sqrMagnitude > 0.0001f)
                    _displayUI.transform.rotation = Quaternion.LookRotation(awayFromCamera, _mainCamera.transform.up);
            }

            // Handle auto-hide after damage
            if (_wasDamaged && _healthHandler.HealthPercentage >= 1f)
            {
                _hideTimer -= Time.deltaTime;
                if (_hideTimer <= 0f)
                {
                    HideHealthBar();
                }
            }
        }

        private void OnTookDamage(Target.Targetable attacker)
        {
            _wasDamaged = true;
            _hideTimer = _hideDelay;
            ShowHealthBar();
            UpdateHealthBar();
        }

        private void OnHealthChanged(HealthHandler handler)
        {
            UpdateHealthBar();

            // If health is full, start the hide timer
            if (handler.HealthPercentage >= 1f)
            {
                _hideTimer = _hideDelay;
            }
            else if (_wasDamaged)
            {
                // Health changed while already displayed (damage followed by healing).
                _hideTimer = _hideDelay;
                ShowHealthBar();
            }
        }

        private void UpdateHealthBar()
        {
            if (_healthBar != null && _healthHandler != null)
            {
                _healthBar.value = _healthHandler.HealthPercentage;
            }
        }

        private void CreateFallbackHealthBar()
        {
            _displayUI = new GameObject("Unit Health Bar");
            _displayUI.transform.SetParent(transform, false);
            _displayUI.transform.localPosition = new Vector3(0f, ResolveLocalHeight(), 0f);
            _displayUI.transform.localScale = Vector3.one * 0.01f;

            Canvas canvas = _displayUI.AddComponent<Canvas>();
            canvas.renderMode = RenderMode.WorldSpace;
            canvas.worldCamera = Camera.main;
            canvas.overrideSorting = true;
            canvas.sortingOrder = 100;

            RectTransform canvasRect = _displayUI.GetComponent<RectTransform>();
            canvasRect.sizeDelta = new Vector2(90f, 10f);

            _healthBar = _displayUI.AddComponent<Slider>();
            _healthBar.minValue = 0f;
            _healthBar.maxValue = 1f;
            _healthBar.value = 1f;
            _healthBar.interactable = false;
            _healthBar.transition = Selectable.Transition.None;

            GameObject background = new GameObject("Background");
            background.transform.SetParent(_displayUI.transform, false);
            RectTransform backgroundRect = background.AddComponent<RectTransform>();
            backgroundRect.anchorMin = Vector2.zero;
            backgroundRect.anchorMax = Vector2.one;
            backgroundRect.offsetMin = Vector2.zero;
            backgroundRect.offsetMax = Vector2.zero;
            Image backgroundImage = background.AddComponent<Image>();
            backgroundImage.color = new Color(0f, 0f, 0f, 0.75f);

            GameObject fill = new GameObject("Fill");
            fill.transform.SetParent(_displayUI.transform, false);
            RectTransform fillRect = fill.AddComponent<RectTransform>();
            fillRect.anchorMin = Vector2.zero;
            fillRect.anchorMax = Vector2.one;
            fillRect.offsetMin = new Vector2(1f, 1f);
            fillRect.offsetMax = new Vector2(-1f, -1f);
            Image fillImage = fill.AddComponent<Image>();
            fillImage.color = new Color(0.85f, 0.1f, 0.1f, 1f);
            _healthBar.fillRect = fillRect;
        }

        private float ResolveLocalHeight()
        {
            float highestWorldPoint = transform.position.y + 2.5f;
            bool foundBounds = false;

            Collider[] colliders = GetComponentsInChildren<Collider>(true);
            for (int i = 0; i < colliders.Length; i++)
            {
                if (colliders[i] == null || colliders[i].isTrigger)
                    continue;

                highestWorldPoint = foundBounds
                    ? Mathf.Max(highestWorldPoint, colliders[i].bounds.max.y)
                    : colliders[i].bounds.max.y;
                foundBounds = true;
            }

            if (!foundBounds)
            {
                Renderer[] renderers = GetComponentsInChildren<Renderer>(true);
                for (int i = 0; i < renderers.Length; i++)
                {
                    if (renderers[i] == null)
                        continue;

                    highestWorldPoint = foundBounds
                        ? Mathf.Max(highestWorldPoint, renderers[i].bounds.max.y)
                        : renderers[i].bounds.max.y;
                    foundBounds = true;
                }
            }

            if (!foundBounds)
                return 2.5f;

            Vector3 localTop = transform.InverseTransformPoint(
                new Vector3(transform.position.x, highestWorldPoint + 0.35f, transform.position.z));
            return localTop.y;
        }

        private void ShowHealthBar()
        {
            if (_displayUI != null && !_displayUI.activeSelf)
                _displayUI.SetActive(true);
        }

        private void HideHealthBar()
        {
            if (_displayUI != null && _displayUI.activeSelf)
                _displayUI.SetActive(false);

            _wasDamaged = false;
        }
    }
}
