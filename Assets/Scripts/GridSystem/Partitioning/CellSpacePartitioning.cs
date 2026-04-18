using System.Collections.Generic;
using Processors;
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
        private Vector2 _originOffset;

        /// <summary>
        /// The width of the partitioned area.
        /// </summary>
        [SerializeField]
        private float _width = 100;

        /// <summary>
        /// The length of the partitioned area.
        /// </summary>
        [SerializeField]
        private float _length = 100;

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
        /// Generates the cells to partition the world.
        /// </summary>
        public void GeneratePartitions()
        {
            _cells = new List<BSPCell>();

            // Calculate size of each cell.
            _numCellsX = (int)(_width / _cellWidth);
            _numCellsZ = (int)(_length / _cellLength);

            _offSetX = _numCellsX * _cellWidth / 2 + _originOffset.x - transform.position.x;
            _offSetZ = _numCellsZ * _cellLength / 2 + _originOffset.y - transform.position.z;

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
            GetCellsInRange(new Vector2(position.x, position.z), radius, ref cells);
        }

        /// <summary>
        /// Gets all cells in a radius around a position as a reference.
        /// </summary>
        /// <param name="position">The position.</param>
        /// <param name="radius">The radius.</param>
        /// <param name="cells">The list to populate with cells.</param>
        public void GetCellsInRange(Vector2 position, float radius, ref List<BSPCell> cells)
        {
            Profiler.BeginSample("Get Cells In Range");
            Vector2 topLeft = position - new Vector2(radius, radius);
            Vector2 bottomRight = position + new Vector2(radius, radius);

            for (int i = 0; i < _cells.Count; i++)
            {
                if (_cells[i].IsOverlapping(topLeft, bottomRight))
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
            List<BSPCell> cells = new List<BSPCell>(1500);
            Profiler.BeginSample("Get Targetables In Range");
            GetCellsInRange(position, radius, ref cells);
            Profiler.EndSample();
            GetTargetablesInCells(flag, ref cells, ref targetables);
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

        /// <summary>
        /// Populates cell indices from ResourceProcessor resource arrays.
        /// Call this after world generation to enable efficient resource lookups.
        /// </summary>
        /// <param name="resourceProcessor">The resource processor.</param>
        public void PopulateResourceIndices(ResourceProcessor resourceProcessor)
        {
            if (resourceProcessor == null)
                return;

            // Clear existing resource indices.
            for (int i = 0; i < _cells.Count; i++)
            {
                _cells[i].WoodResourceIndices = new List<int>();
                _cells[i].OreResourceIndices = new List<int>();
                _cells[i].FoodResourceIndices = new List<int>();
                _cells[i].GoldResourceIndices = new List<int>();
                _cells[i].RecruitResourceIndices = new List<int>();
            }

            // Populate wood resource indices.
            var woodResourcesDict = resourceProcessor.GetWoodResources();
            List<GameResources.ResourceData> woodResources = new List<GameResources.ResourceData>();
            foreach (var kvp in woodResourcesDict)
            {
                woodResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < woodResources.Count; i++)
            {
                int cellIndex = PositionToIndex(woodResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].WoodResourceIndices.Add(i);
                }
            }

            // Populate ore resource indices.
            var oreResourcesDict = resourceProcessor.GetOreResources();
            List<GameResources.ResourceData> oreResources = new List<GameResources.ResourceData>();
            foreach (var kvp in oreResourcesDict)
            {
                oreResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < oreResources.Count; i++)
            {
                int cellIndex = PositionToIndex(oreResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].OreResourceIndices.Add(i);
                }
            }

            // Populate food resource indices.
            var foodResourcesDict = resourceProcessor.GetFoodResources();
            List<GameResources.ResourceData> foodResources = new List<GameResources.ResourceData>();
            foreach (var kvp in foodResourcesDict)
            {
                foodResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < foodResources.Count; i++)
            {
                int cellIndex = PositionToIndex(foodResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].FoodResourceIndices.Add(i);
                }
            }

            // Populate gold resource indices.
            var goldResourcesDict = resourceProcessor.GetGoldResources();
            List<GameResources.ResourceData> goldResources = new List<GameResources.ResourceData>();
            foreach (var kvp in goldResourcesDict)
            {
                goldResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < goldResources.Count; i++)
            {
                int cellIndex = PositionToIndex(goldResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].GoldResourceIndices.Add(i);
                }
            }

            // Populate recruit resource indices.
            var recruitResourcesDict = resourceProcessor.GetRecruitResources();
            List<GameResources.ResourceData> recruitResources = new List<GameResources.ResourceData>();
            foreach (var kvp in recruitResourcesDict)
            {
                recruitResources.AddRange(kvp.Value);
            }
            for (int i = 0; i < recruitResources.Count; i++)
            {
                int cellIndex = PositionToIndex(recruitResources[i].Position);
                if (cellIndex >= 0 && cellIndex < _cells.Count)
                {
                    _cells[cellIndex].RecruitResourceIndices.Add(i);
                }
            }
        }

        /// <summary>
        /// Gets all resources of a specific type within a radius of a position.
        /// Returns the actual ResourceData objects, not just indices.
        /// </summary>
        /// <param name="resourceType">The resource type.</param>
        /// <param name="position">The position.</param>
        /// <param name="radius">The radius.</param>
        /// <param name="resourceProcessor">The resource processor.</param>
        /// <param name="resources">The list to populate with resources.</param>
        public void GetResourcesInRange(global::Utils.Resource resourceType, Vector3 position, float radius, ResourceProcessor resourceProcessor, ref List<GameResources.ResourceData> resources)
        {
            if (resourceProcessor == null)
                return;

            List<BSPCell> cells = new List<BSPCell>();
            GetCellsInRange(position, radius, ref cells);

            List<GameResources.ResourceData> resourceList = new List<GameResources.ResourceData>();
            Dictionary<(int meshIndex, int materialIndex), GameResources.ResourceData[]> resourceDict = null;

            switch (resourceType)
            {
                case global::Utils.Resource.Wood:
                    resourceDict = resourceProcessor.GetWoodResources();
                    break;
                case global::Utils.Resource.Ore:
                    resourceDict = resourceProcessor.GetOreResources();
                    break;
                case global::Utils.Resource.Food:
                    resourceDict = resourceProcessor.GetFoodResources();
                    break;
                case global::Utils.Resource.Gold:
                    resourceDict = resourceProcessor.GetGoldResources();
                    break;
                case global::Utils.Resource.Recruit:
                    resourceDict = resourceProcessor.GetRecruitResources();
                    break;
                default:
                    return;
            }

            foreach (var kvp in resourceDict)
            {
                resourceList.AddRange(kvp.Value);
            }

            GameResources.ResourceData[] resourceArray = resourceList.ToArray();

            for (int i = 0; i < cells.Count; i++)
            {
                List<int> indices = GetResourceIndicesForCell(cells[i], resourceType);
                if (indices != null && resourceArray != null)
                {
                    for (int j = 0; j < indices.Count; j++)
                    {
                        int index = indices[j];
                        if (index >= 0 && index < resourceArray.Length)
                        {
                            GameResources.ResourceData resource = resourceArray[index];
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
        private List<int> GetResourceIndicesForCell(BSPCell cell, global::Utils.Resource resourceType)
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

        // Unity Functions.
        /// <summary>
        /// Initializes the cell space partitioning.
        /// </summary>
        private void Awake()
        {
            //Call to initialize targetflags.
            if (TargetFlagHelper.TargetFlags != null) { }
            GeneratePartitions();
        }

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
