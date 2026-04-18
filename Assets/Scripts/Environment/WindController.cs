using UnityEngine;

namespace Environment
{
    /// <summary>
    /// Wind Controller will modify the wind direction and strength in each assigned material
    /// </summary>
    public class WindController : MonoBehaviour
    {
        /// <summary>
        /// Array of materials to apply wind effects to.
        /// </summary>
        [SerializeField]
        private Material[] _materials;

        /// <summary>
        /// Wind speed.
        /// </summary>
        [SerializeField, Space(10), Range(0.0f, 5.0f)]
        private float _speed = 1.0f;

        /// <summary>
        /// Texture size for wind effect.
        /// </summary>
        [SerializeField]
        private float _textureSize = 1.0f;

        /// <summary>
        /// Converts radians to a Vector2.
        /// </summary>
        /// <param name="_radian">The angle in radians.</param>
        /// <returns>A Vector2 representation of the angle.</returns>
        private static Vector2 RadianToVector2(float _radian)
        {
            // Convert radians to a Vector2 using cosine and sine functions.
            return new Vector2(Mathf.Cos(_radian), Mathf.Sin(_radian));
        }
      
        /// <summary>
        /// Converts an object's rotation (in degrees) to a Vector2.
        /// </summary>
        /// <param name="_degree">The angle in degrees.</param>
        /// <returns>A Vector2 representation of the angle.</returns>
        private static Vector2 DegreeToVector2(float _degree)
        {
            // Convert degrees to radians and then to a Vector2.
            return RadianToVector2(_degree * Mathf.Deg2Rad);
        }

        /// <summary>
        /// Aligns materials wind direction.
        /// </summary>
        public void AlignWind()
        {
            // Find the object's rotation and convert it to a Vector2.
            Vector2 _direction = DegreeToVector2(transform.eulerAngles.y);
            if (_materials.Length > 0)
            {
                for (int i = 0; i < _materials.Length; i++)
                {
                    // Set the wind direction for each material.
                    _materials[i].SetVector("_windDirection", _direction);
                }
            }
        }

        /// <summary>
        /// Changes materials wind strength.
        /// </summary>
        public void ChangeWindStrength()
        {
            for (int i = 0; i < _materials.Length; i++)
            {
                // Set the wind strength for each material.
                _materials[i].SetFloat("_windStrength", _speed);
            }
        }

        /// <summary>
        /// Changes materials wind texture size.
        /// </summary>
        public void ChangeWindTextureSize()
        {
            for (int i = 0; i < _materials.Length; i++)
            {
                // Set the texture size for each material.
                _materials[i].SetFloat("_textureSize", _textureSize);
            }
        }

        /// <summary>
        /// Used to display the direction of the wind in the editor.
        /// </summary>
        private void OnDrawGizmos()
        {
            // Draw a line to represent the wind direction.
            Gizmos.DrawLine(new Vector3(transform.localPosition.x, transform.localPosition.y, transform.localPosition.z), transform.localPosition + transform.forward);
            // Draw additional lines to create an arrow shape.
            Gizmos.DrawLine(new Vector3(transform.localPosition.x, transform.localPosition.y + 0.1f, transform.localPosition.z), transform.localPosition + transform.forward);
            Gizmos.DrawLine(new Vector3(transform.localPosition.x, transform.localPosition.y - 0.1f, transform.localPosition.z), transform.localPosition + transform.forward);
        }
    }
}
