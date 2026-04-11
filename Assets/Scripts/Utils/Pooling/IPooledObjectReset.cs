namespace Utils.Pooling
{
	/// <summary>
	/// Interface for pooled objects that need to be reset when retrieved from the pool.
	/// </summary>
	public interface IPooledObjectReset
	{
		/// <summary>
		/// Called when the object is retrieved from the pool and activated.
		/// Use this for initialization that depends on injected fields.
		/// </summary>
		void OnReset();
	}
}
