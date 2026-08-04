using System;

namespace SavingAndLoading.SavableObjects
{
	/// <summary>
	/// Legacy adapter retained so existing serialized references remain valid.
	/// Resource snapshots now belong exclusively to SaveProcessor.
	/// </summary>
	[Obsolete("Resolve SaveProcessor instead of invoking per-system saveable objects.")]
	public class SaveableResourceProcessor : SaveableObject
	{
	}
}
