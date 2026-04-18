using Utils;

namespace GameResources
{
	/// <summary>
	/// An interface for any class that requires holding resources.
	/// </summary>
	public interface IResourceHolder
	{
        /// <summary>
        /// Adds a resource of the specified type and amount.
        /// </summary>
        /// <param name="type">The resource type.</param>
        /// <param name="amount">The amount to add.</param>
		public void AddResource(Utils.Resource type, int amount);

        /// <summary>
        /// Removes a resource of the specified type and amount.
        /// </summary>
        /// <param name="type">The resource type.</param>
        /// <param name="amount">The amount to remove.</param>
		public void RemoveResource(Utils.Resource type, int amount);

        /// <summary>
        /// Checks if the resource of the specified type is full.
        /// </summary>
        /// <param name="type">The resource type.</param>
        /// <returns>True if the resource is full.</returns>
		public bool ResourceFull(Utils.Resource type);
	}
}
