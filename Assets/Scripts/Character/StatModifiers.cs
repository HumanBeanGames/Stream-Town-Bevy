using System.Collections.Generic;
using Utils;

namespace Character 
{
    /// <summary>
    /// Holds stat modifiers for a character.
    /// </summary>
    public class StatModifiers 
	{
        /// <summary>
        /// Dictionary of stat type to modifier value.
        /// </summary>
		private Dictionary<StatType, int> _modifiers;

        /// <summary>
        /// Initializes a new stat modifiers instance.
        /// </summary>
		public StatModifiers()
		{
			_modifiers = new Dictionary<StatType, int>();

			for(int i = 0; i < (int)StatType.Count;i++)
			{
				_modifiers.Add((StatType)i, 0);
			}
		}

        /// <summary>
        /// Gets the modifier for a stat type.
        /// </summary>
        /// <param name="stat">The stat type.</param>
        /// <returns>The modifier value.</returns>
		public int GetModifier(StatType stat)
		{
			return _modifiers[stat];
		}

        /// <summary>
        /// Adds to the modifier for a stat type.
        /// </summary>
        /// <param name="stat">The stat type.</param>
        /// <param name="amount">The amount to add.</param>
		public void AddToModifier(StatType stat, int amount)
		{
			_modifiers[stat] += amount;
		}

        /// <summary>
        /// Removes from the modifier for a stat type.
        /// </summary>
        /// <param name="stat">The stat type.</param>
        /// <param name="amount">The amount to remove.</param>
		public void RemoveFromModifier(StatType stat, int amount)
		{
			_modifiers[stat] -= amount;
		}

        /// <summary>
        /// Sets the modifier for a stat type.
        /// </summary>
        /// <param name="stat">The stat type.</param>
        /// <param name="value">The value to set.</param>
		public void SetModifier(StatType stat, int value)
		{
			_modifiers[stat] = value;
		}
    }
}
