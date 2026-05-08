using System.Collections.Generic;
using Processors;
using Reflex.Attributes;
using UnityEngine;
using Utils;
using GameResources;
using UnityEngine.Profiling;
using Target;

namespace GridSystem.Partitioning
{
    /// <summary>
    /// Uses cells to partition a world for more efficient lookup of objects and cells.
    /// </summary>
    public class CellSpacePartitioning : MonoBehaviour
    {
        /// <summary>
        /// The origin offset.
        /// </summary>
        [SerializeField]
        private Vector2 _originOffset = new Vector2(-50, -50);

        /// <summary>
        /// The width of the partitioned area.
        /// </summary>
        [SerializeField]
        private float _width = 300;

        /// <summary>
        /// The length of the partitioned area.
        /// </summary>
        [SerializeField]
        private float _length = 300;

        /// <summary>
        /// The width of each cell.
        /// </summary>
        [SerializeField]
        private float _cellWidth = 10;

        /// <summary>
        /// The length of each cell.
        /// </summary>
        [SerializeField]
        private float _cellLength = 10;

        /// <summary>
        /// List of BSP cells.
        /// </summary>
        [SerializeField, HideInInspector]
        private List<BSPCell> _cells = new List<BSPCell>();

        /// <summary>
        /// The number of cells in the X direction.
        /// </summary>
        private int _numCellsX = 0;

        /// <summary>
        /// The number of cells in the Z direction.
        /// </summary>
        private int _numCellsZ = 0;

        /// <summary>
        /// The X offset.
        /// </summary>
        private float _offSetX = 0;

        /// <summary>
        /// The Z offset.
        /// </summary>
        private float _offSetZ = 0;

        /// <summary>
        /// The debug processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Processors.DebugProcessor _debugProcessor;

        /// <summary>
        /// Generates the cells to partition the world.
        /// </summary>
        public void GeneratePartitions()
        {
            _cells = new List<BSPCell>();

            // Calculate size of each cell.
            _numCellsX = (int)(_width / _cellWidth);
            _numCellsZ = (int)(_length / _cellLength);

            _offSetX = -_originOffset.x + transform.position.x;
            _offSetZ = -_originOffset.y + transform.position.z;


            // Create the Cells.
            for (int z = 0; z < _numCellsZ; z++)
            {
                for (int x = 0; x < _numCellsX; x++)
                {
                    float left = (x * _cellWidth) - _offSetX;
                    float right = left + _cellWidth;
                    float top = (z * _cellLength) - _offSetZ;
                    float bottom = top + _cellLength;

                    _cells.Add(new BSPCell(new Vector2(left, top), new Vector2(right, bottom)));
                }
            }

        }

        /// <summary>
        /// Returns a cell index based on a position.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <returns>The cell index.</returns>
        public int PositionToIndex(Vector3 position)
        {
            Vector2 v2Pos = new Vector2(position.x + _offSetX, position.z + _offSetZ);

            if (_debugProcessor != null)
                _debugProcessor.Log(DebugLogCategory.CellSpacePartitioning, $"PositionToIndex: input=({position.x}, {position.z}), adjusted=({v2Pos.x}, {v2Pos.y}), _numCellsX={_numCellsX}, _numCellsZ={_numCellsZ}, _width={_width}, _length={_length}, _offSetX={_offSetX}, _offSetZ={_offSetZ}");

            return PositionToIndex(v2Pos);
        }

        /// <summary>
        /// Returns a cell index based on a position.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <returns>The cell index.</returns>
        public int PositionToIndex(Vector2 position)
        {
            int index = (int)(_numCellsX * position.x / _width);
            index += (int)(_numCellsZ * position.y / _length) * _numCellsX;

            if (index < 0 || index >= _cells.Count)
            {
                if (_debugProcessor != null)
                    _debugProcessor.LogWarning(DebugLogCategory.CellSpacePartitioning, $"PositionToIndex: input=({position.x}, {position.y}), calculated index={index} is out of bounds [0, {_cells.Count})");
            }

            if (index > _cells.Count - 1)
                index = _cells.Count - 1;

            return index;
        }

        /// <summary>
        /// Gets all cells in a radius around a position as a reference.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <param name="radius">The radius.</param>
        /// <param name="cells">The list to populate with cells.</param>
        public void GetCellsInRange(Vector3 position, float radius, ref List<BSPCell> cells)
        {
            Vector2 localPosition = new Vector2(position.x, position.z);
            GetCellsInRange(localPosition, radius, ref cells);
        }

        /// <summary>
        /// Gets all cells in a radius around a position as a reference.
        /// </summary>
        /// <param name="position">The position (in local space).</param>
        /// <param name="radius">The radius.</param>
        /// <param name="cells">The list to populate with cells.</param>
        public void GetCellsInRange(Vector2 position, float radius, ref List<BSPCell> cells)
        {
            Profiler.BeginSample("Get Cells In Range");
            Vector2 topLeft = position - new Vector2(radius, radius);
            Vector2 bottomRight = position + new Vector2(radius, radius);

            for (int i = 0; i < _cells.Count; i++)
            {
                bool overlaps = _cells[i].IsOverlapping(topLeft, bottomRight);
                if (overlaps)
                {
                    cells.Add(_cells[i]);
                }
            }
            Profiler.EndSample();
        }

        /// <summary>
        /// Gets all cells that overlap with a rectangular area.
        /// </summary>
        /// <param name="topLeft">The top-left corner of the rectangle.</param>
        /// <param name="bottomRight">The bottom-right corner of the rectangle.</param>
        /// <param name="cells">The list to populate with cells.</param>
        public void GetCellsInRect(Vector2 topLeft, Vector2 bottomRight, ref List<BSPCell> cells)
        {
            Profiler.BeginSample("Get Cells In Rect");
            for (int i = 0; i < _cells.Count; i++)
            {
                bool overlaps = _cells[i].IsOverlapping(topLeft, bottomRight);
                if (overlaps)
                {
                    cells.Add(_cells[i]);
                }
            }
            Profiler.EndSample();
        }

        /// <summary>
        /// Gets all Targetable objects within a radius around a position.
        /// </summary>
        /// <param name="flag">The target mask flag.</param>
        /// <param name="position">The position.</param>
        /// <param name="radius">The radius.</param>
        /// <param name="targetables">The list to populate with targetables.</param>
        public void GetTargetablesInRange(TargetMask flag, Vector3 position, float radius, ref List<Targetable> targetables)
        {
            if (flag == TargetMask.Construction && _debugProcessor != null)
                _debugProcessor.Log(DebugLogCategory.CellSpacePartitioning, $"GetTargetablesInRange for Construction flag at {position}, radius={radius}");

            List<BSPCell> cells = new List<BSPCell>(1500);
            Profiler.BeginSample("Get Targetables In Range");
            GetCellsInRange(position, radius, ref cells);

            if (flag == TargetMask.Construction && _debugProcessor != null)
                _debugProcessor.Log(DebugLogCategory.CellSpacePartitioning, $"Found {cells.Count} cells in range");

            Profiler.EndSample();
            GetTargetablesInCells(flag, ref cells, ref targetables);

            if (flag == TargetMask.Construction && _debugProcessor != null)
                _debugProcessor.Log(DebugLogCategory.CellSpacePartitioning, $"Returning {targetables.Count} construction targets");
        }

        /// <summary>
        /// Gets all Targetable objects within a defined list of cells.
        /// </summary>
        /// <param name="flag">The target mask flag.</param>
        /// <param name="cells">The cells to search.</param>
        /// <param name="targetables">The list to populate with targetables.</param>
        public void GetTargetablesInCells(TargetMask flag, ref List<BSPCell> cells, ref List<Targetable> targetables)
        {
            Profiler.BeginSample("Get Targetables In Cell");

            for (int i = 0; i < cells.Count; i++)
            {
                cells[i].GetTargetsByFlag(flag, ref targetables);
            }
            Profiler.EndSample();
        }

        //TODO: Add a "Get Closest Cell"

        /// <summary>
        /// Returns the cell based on index.
        /// </summary>
        /// <param name="index">The cell index.</param>
        /// <returns>The BSP cell.</returns>
        public BSPCell GetCellAtIndex(int index) => _cells[index];

        /// <summary>
        /// Returns the amount of cells in the world.
        /// </summary>
        /// <returns>The number of cells.</returns>
        public int CellCount() => _cells == null ? 0 : _cells.Count;

        /// <summary>
        /// Returns all cells as an array.
        /// </summary>
        /// <returns>The array of BSP cells.</returns>
        public BSPCell[] GetCells() { return _cells.ToArray(); }

        private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _woodResources;
        private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _oreResources;
        private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _foodResources;
        private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _goldResources;
        private Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> _recruitResources;

        private List<GameResources.FoliageData> _onLandFoliage;
        private List<GameResources.FoliageData> _underWaterFoliage;

        // Cached flattened lists for queries (must match indices stored during population)
        private List<GameResources.ResourceData> _cachedWoodResources;
        private List<GameResources.ResourceData> _cachedOreResources;
        private List<GameResources.ResourceData> _cachedFoodResources;
        private List<GameResources.ResourceData> _cachedGoldResources;
        private List<GameResources.ResourceData> _cachedRecruitResources;

        /// <summary>
        /// Populates cell indices from ResourceProcessor resource dictionaries.
        /// Call this after world generation to enable efficient resource lookups.
        /// </summary>
        /// <param name="resourceProcessor">The resource processor.</param>
        public void PopulateResourceIndices(ResourceProcessor resourceProcessor)
        {
            if (resourceProcessor == null)
            {
                return;
            }

            // Clear existing resource indices.
            for (int i = 0; i < _cells.Count; i++)
            {
                _cells[i].WoodResourceIndices = new List<int>();
                _cells[i].OreResourceIndices = new List<int>();
                _cells[i].FoodResourceIndices = new List<int>();
                _cells[i].GoldResourceIndices = new List<int>();
                _cells[i].RecruitResourceIndices = new List<int>();
            }

            // Cache resource dictionaries
            _woodResources = resourceProcessor.GetWoodResources();
            _oreResources = resourceProcessor.GetOreResources();
            _foodResources = resourceProcessor.GetFoodResources();
            _goldResources = resourceProcessor.GetGoldResources();
            _recruitResources = resourceProcessor.GetRecruitResources();


            // Populate wood resource indices.
            _cachedWoodResources = new List<GameResources.ResourceData>();
            foreach (var kvp in _woodResources)
            {
                _cachedWoodResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < _cachedWoodResources.Count; i++)
            {
                int cellIndex = PositionToIndex(_cachedWoodResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].WoodResourceIndices.Add(i);
                }
            }

            // Populate ore resource indices.
            _cachedOreResources = new List<GameResources.ResourceData>();
            foreach (var kvp in _oreResources)
            {
                _cachedOreResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < _cachedOreResources.Count; i++)
            {
                int cellIndex = PositionToIndex(_cachedOreResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].OreResourceIndices.Add(i);
                }
            }

            // Populate food resource indices.
            _cachedFoodResources = new List<GameResources.ResourceData>();
            foreach (var kvp in _foodResources)
            {
                _cachedFoodResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < _cachedFoodResources.Count; i++)
            {
                int cellIndex = PositionToIndex(_cachedFoodResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].FoodResourceIndices.Add(i);
                }
            }

            // Populate gold resource indices.
            _cachedGoldResources = new List<GameResources.ResourceData>();
            foreach (var kvp in _goldResources)
            {
                _cachedGoldResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < _cachedGoldResources.Count; i++)
            {
                int cellIndex = PositionToIndex(_cachedGoldResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].GoldResourceIndices.Add(i);
                }
            }

            // Populate recruit resource indices.
            _cachedRecruitResources = new List<GameResources.ResourceData>();
            foreach (var kvp in _recruitResources)
            {
                _cachedRecruitResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < _cachedRecruitResources.Count; i++)
            {
                int cellIndex = PositionToIndex(_cachedRecruitResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].RecruitResourceIndices.Add(i);
                }
            }

        }

        /// <summary>
        /// Populates cell indices from FoliageProcessor foliage arrays.
        /// Call this after world generation to enable efficient foliage lookups.
        /// </summary>
        /// <param name="foliageProcessor">The foliage processor.</param>
        public void PopulateFoliageIndices(Processors.FoliageProcessor foliageProcessor)
        {
            if (foliageProcessor == null)
            {
                return;
            }

            // Clear existing foliage indices.
            for (int i = 0; i < _cells.Count; i++)
            {
                _cells[i].OnLandFoliageIndices = new List<int>();
                _cells[i].UnderWaterFoliageIndices = new List<int>();
            }

            // Cache foliage arrays
            _onLandFoliage = foliageProcessor.GetOnLandFoliage();
            _underWaterFoliage = foliageProcessor.GetUnderWaterFoliage();


            // Populate on-land foliage indices.
            for (int i = 0; i < _onLandFoliage.Count; i++)
            {
                int cellIndex = PositionToIndex(_onLandFoliage[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].OnLandFoliageIndices.Add(i);
                }
            }

            // Populate underwater foliage indices.
            for (int i = 0; i < _underWaterFoliage.Count; i++)
            {
                int cellIndex = PositionToIndex(_underWaterFoliage[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].UnderWaterFoliageIndices.Add(i);
                }
            }

        }

        /// <summary>
        /// Gets all foliage within a radius of a position.
        /// Uses cached foliage lists that match the indices stored during population.
        /// </summary>
        /// <param name="position">The center position.</param>
        /// <param name="radius">The search radius.</param>
        /// <param name="isUnderwater">Whether to get underwater foliage (false for on-land).</param>
        /// <param name="foliage">The list to populate with foliage.</param>
        public void GetFoliageInRange(Vector3 position, float radius, bool isUnderwater, ref List<GameResources.FoliageData> foliage)
        {
            List<BSPCell> cells = new List<BSPCell>();
            GetCellsInRange(position, radius, ref cells);

            List<GameResources.FoliageData> foliageList = isUnderwater ? _underWaterFoliage : _onLandFoliage;
            if (foliageList == null)
                return;

            for (int i = 0; i < cells.Count; i++)
            {
                List<int> indices = isUnderwater ? cells[i].UnderWaterFoliageIndices : cells[i].OnLandFoliageIndices;
                if (indices != null)
                {
                    for (int j = 0; j < indices.Count; j++)
                    {
                        int index = indices[j];
                        if (index >= 0 && index < foliageList.Count)
                        {
                            GameResources.FoliageData f = foliageList[index];
                            float distance = Vector3.Distance(position, f.Position);
                            if (distance <= radius)
                            {
                                foliage.Add(f);
                            }
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Gets all resources of a specific type within a radius of a position.
        /// Returns the actual ResourceData objects, not just indices.
        /// Uses cached flattened lists to ensure indices match those stored during population.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="position">The position.</param>
        /// <param name="radius">The radius.</param>
        /// <param name="resourceProcessor">The resource processor.</param>
        /// <param name="resources">The list to populate with resources.</param>
        public void GetResourcesInRange(global::Utils.Resource resourceType, Vector3 position, float radius, Processors.ResourceProcessor resourceProcessor, ref List<GameResources.ResourceData> resources)
        {
            List<BSPCell> cells = new List<BSPCell>();
            GetCellsInRange(position, radius, ref cells);


            // Get the cached resource list for this type
            List<GameResources.ResourceData> resourceList;
            switch (resourceType)
            {
                case global::Utils.Resource.Wood:
                    resourceList = _cachedWoodResources;
                    break;
                case global::Utils.Resource.Ore:
                    resourceList = _cachedOreResources;
                    break;
                case global::Utils.Resource.Food:
                    resourceList = _cachedFoodResources;
                    break;
                case global::Utils.Resource.Gold:
                    resourceList = _cachedGoldResources;
                    break;
                case global::Utils.Resource.Recruit:
                    resourceList = _cachedRecruitResources;
                    break;
                default:
                    return;
            }

            if (resourceList == null)
                return;

            for (int i = 0; i < cells.Count; i++)
            {
                List<int> indices = GetResourceIndicesForCell(cells[i], resourceType);
                if (indices != null)
                {
                    for (int j = 0; j < indices.Count; j++)
                    {
                        int index = indices[j];
                        if (index >= 0 && index < resourceList.Count)
                        {
                            GameResources.ResourceData resource = resourceList[index];
                            float distance = Vector3.Distance(position, resource.Position);
                            if (distance <= radius)
                            {
                                resources.Add(resource);
                            }
                        }
                    }
                }
            }
        }

        /// <summary>
        /// Gets the resource indices for a specific resource type from a cell.
        /// </summary>
        /// <param name="cell">The BSP cell.</param>
        /// <param name="resourceType">The resource type.</param>
        /// <returns>The list of resource indices.</returns>
        public List<int> GetResourceIndicesForCell(BSPCell cell, global::Utils.Resource resourceType)
        {
            switch (resourceType)
            {
                case global::Utils.Resource.Wood:
                    return cell.WoodResourceIndices;
                case global::Utils.Resource.Ore:
                    return cell.OreResourceIndices;
                case global::Utils.Resource.Food:
                    return cell.FoodResourceIndices;
                case global::Utils.Resource.Gold:
                    return cell.GoldResourceIndices;
                case global::Utils.Resource.Recruit:
                    return cell.RecruitResourceIndices;
                default:
                    return null;
            }
        }

        /// <summary>
        /// Gets the cached resource list for a specific resource type.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <returns>The cached resource list, or null if not found.</returns>
        public List<GameResources.ResourceData> GetResourceListForType(global::Utils.Resource resourceType)
        {
            switch (resourceType)
            {
                case global::Utils.Resource.Wood:
                    return _cachedWoodResources;
                case global::Utils.Resource.Ore:
                    return _cachedOreResources;
                case global::Utils.Resource.Food:
                    return _cachedFoodResources;
                case global::Utils.Resource.Gold:
                    return _cachedGoldResources;
                case global::Utils.Resource.Recruit:
                    return _cachedRecruitResources;
                default:
                    return null;
            }
        }

        // Unity Functions.
        // Note: CellSpacePartitioning is ProjectScope, so Awake/Start are not called.
        // GeneratePartitions is called from GridProcessor.InstallBindings during container construction.

        /// <summary>
        /// Draws gizmos when the object is selected.
        /// </summary>
        private void OnDrawGizmosSelected()
        {
            if (_cells == null)
                return;

            Color prevColor = Gizmos.color;
            Gizmos.color = Color.red;

            for (int i = 0; i < _cells.Count; i++)
            {
                BSPCell cell = _cells[i];
                if (cell.Searched)
                {
                    Gizmos.color = Color.blue;
                    //Gizmos.DrawWireCube(new Vector3(cell.Center.x,0,cell.Center.y), new Vector3(_cellWidth, 0.5f, _cellLength));
                }
                else
                    Gizmos.color = Color.red;

                Vector3 line1 = new Vector3(cell.Left, 0, cell.Top);
                Vector3 line2 = new Vector3(cell.Right, 0, cell.Top);

                Vector3 line3 = new Vector3(cell.Left, 0, cell.Bottom);
                Vector3 line4 = new Vector3(cell.Right, 0, cell.Bottom);

                Vector3 line5 = new Vector3(cell.Left, 0, cell.Top);
                Vector3 line6 = new Vector3(cell.Left, 0, cell.Bottom);

                Vector3 line7 = new Vector3(cell.Right, 0, cell.Top);
                Vector3 line8 = new Vector3(cell.Right, 0, cell.Bottom);

                Gizmos.DrawLine(line1, line2);
                Gizmos.DrawLine(line3, line4);
                Gizmos.DrawLine(line5, line6);
                Gizmos.DrawLine(line7, line8);
            }

            Gizmos.color = prevColor;
        }
    }
}
