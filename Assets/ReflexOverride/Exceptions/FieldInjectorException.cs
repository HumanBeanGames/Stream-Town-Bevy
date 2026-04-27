using System;
using UnityEngine;

namespace Reflex.Exceptions
{
    internal sealed class FieldInjectorException : Exception
    {
        public FieldInjectorException(Exception e, object instance, string fieldName) : base($"Cannot inject field '{fieldName}' in '{instance?.GetType().Name}' on GameObject '{(instance as MonoBehaviour)?.gameObject.name ?? "N/A"}': {e.Message}")
        {
        }
    }
}
