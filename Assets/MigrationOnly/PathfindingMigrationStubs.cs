using System;
using UnityEngine;

// Migration-only surface stubs for the removed commercial A* Pathfinding
// Project package. These exist solely so Unity can compile the asset exporter.
// They deliberately provide no routing, scanning, or graph semantics.
namespace Pathfinding
{
    [AddComponentMenu("")]
    public sealed class AIPath : MonoBehaviour
    {
        public bool canMove = true;
        public bool pathPending;
        public float remainingDistance = -1f;
        public float slowdownDistance = 1f;
        public float endReachedDistance = 0.2f;
        public float maxAcceleration;
        public float maxSpeed;
        public Vector3 destination;
        public Vector3 gravity;
        public Vector3 velocity;
        public Vector3 position => transform.position;

        public void Teleport(Vector3 position, bool clearPath = true)
        {
            transform.position = position;
            if (clearPath)
                remainingDistance = -1f;
        }
    }

    [AddComponentMenu("")]
    public sealed class AstarPath : MonoBehaviour
    {
        public static AstarPath active;
        public AstarData data { get; } = new AstarData();

        private void OnEnable()
        {
            active = this;
        }

        public NNInfo GetNearest(Vector3 position, NNConstraint constraint)
        {
            return new NNInfo(null, position);
        }

        public void Scan()
        {
        }

        public void Scan(NavGraph graph)
        {
        }

        public void UpdateGraphs(GraphUpdateObject update)
        {
        }
    }

    public sealed class AstarData
    {
        private GridGraph _migrationGraph;
        public NavGraph[] graphs => _migrationGraph == null
            ? Array.Empty<NavGraph>()
            : new NavGraph[] { _migrationGraph };

        public NavGraph FindGraphOfType(Type type)
        {
            return type == typeof(GridGraph) ? _migrationGraph : null;
        }

        public NavGraph AddGraph(Type type)
        {
            if (type != typeof(GridGraph))
                return null;
            _migrationGraph = new GridGraph();
            return _migrationGraph;
        }
    }

    public abstract class NavGraph
    {
        public string name;
    }

    public sealed class GridGraph : NavGraph
    {
        public Vector3 center;
        public float maxClimb;
        public GraphCollision collision = new GraphCollision();

        public void SetDimensions(int width, int depth, float nodeSize)
        {
        }

        public void GetNodes(Action<GraphNode> visitor)
        {
        }

        public NNInfo GetNearest(Vector3 position, NNConstraint constraint)
        {
            return new NNInfo(null, position);
        }
    }

    public sealed class GraphCollision
    {
        public bool use2D;
        public int mask;
        public float height;
        public float diameter;
    }

    public class GraphNode
    {
        public bool Walkable;
        public Int3 position;
    }

    public readonly struct Int3
    {
        private readonly Vector3 _value;

        public Int3(Vector3 value)
        {
            _value = value;
        }

        public static explicit operator Vector3(Int3 value)
        {
            return value._value;
        }
    }

    public readonly struct NNInfo
    {
        public readonly GraphNode node;
        public readonly Vector3 position;

        public NNInfo(GraphNode node, Vector3 position)
        {
            this.node = node;
            this.position = position;
        }
    }

    public sealed class NNConstraint
    {
        public static readonly NNConstraint Default = new NNConstraint();
    }

    public sealed class GraphUpdateObject
    {
        public bool modifyWalkability;
        public bool setWalkability;
        public bool modifyTag;
        public int setTag;

        public GraphUpdateObject(Bounds bounds)
        {
        }
    }

    public static class PathUtilities
    {
        public static bool IsPathPossible(GraphNode from, GraphNode to)
        {
            return false;
        }
    }
}
