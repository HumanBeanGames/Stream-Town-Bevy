using UnityEngine;

namespace Reflex.Core
{
    // Marker class for ProjectScope to detect and handle instantiation logic
    public class InstantiationBarrier : MonoBehaviour, IInstaller
    {
        public void InstallBindings(ContainerBuilder containerBuilder)
        {
            // ProjectScope handles the instantiation logic when it detects this barrier
        }
    }
}
