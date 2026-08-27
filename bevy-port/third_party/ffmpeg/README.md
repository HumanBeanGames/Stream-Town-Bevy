# FFmpeg runtime and relinking

Stream Town's Windows direct-Twitch feature dynamically links the following
shared libraries; it does not invoke or bundle the FFmpeg command-line program:

- FFmpeg 8.1.1: `avcodec-62.dll`, `avformat-62.dll`, `avutil-60.dll`,
  `swresample-6.dll`, and `swscale-9.dll`
- OpenH264 2.6.0: `openh264-7.dll`

The pinned `vcpkg.json` builds FFmpeg without the `gpl` or `nonfree` features.
It enables AMD AMF H.264 acceleration, with Windows Media Foundation hardware
encoding and BSD-licensed OpenH264 as fallbacks. The built-in FFmpeg AAC encoder
completes Twitch's H.264/AAC ingest pair.
The shared libraries remain separate beside the executable so recipients can
replace/rebuild them.

## Reproduce the shipped DLLs

1. Check out Microsoft vcpkg at commit
   `e0b785fa42e9b3ee27f62e54f65e41358a1d2671`.
2. Install current Visual Studio Build Tools with the MSVC C++ workload, NASM,
   and a Rust 1.95 toolchain.
3. From `bevy-port`, run:

   ```powershell
   $env:VCPKG_ROOT = "C:\path\to\vcpkg"
   & "$env:VCPKG_ROOT\vcpkg.exe" install --triplet x64-windows
   $env:VCPKGRS_DYNAMIC = "1"
   cargo build --release -p stream_town_game -p stream_town_tools
   ```

4. Copy the six DLLs listed above from
   `vcpkg_installed\x64-windows\bin` beside the executables. With classic-mode
   vcpkg they are under `%VCPKG_ROOT%\installed\x64-windows\bin`.

The Windows packager includes the exact FFmpeg, OpenH264, and AMD AMF header
source archives used by vcpkg, the vcpkg port recipes and patches, installed
SPDX manifests, and license texts in `StreamTown/third_party/source`. The source
archive hashes pinned by the vcpkg ports are:

- FFmpeg n8.1.1 SHA-512:
  `e858e92e5eb08d562302cde371af55917df6e1fe53994e18462a3c929a40ede1828c2bd53c2a7d65a2cfd791782ead3cd94efb2def904f49cb5dd8ab5cd4256f`
- OpenH264 v2.6.0 SHA-512:
  `26a03acde7153a6b40b99f00641772433a244c72a3cc4bca6d903cf3b770174d028369a2fb73b2f0774e1124db0e269758eed6d88975347a815e0366c820d247`
- AMD AMF headers v1.5.2 SHA-512:
  `b992d4a1f59f7b1c789d03e7bd9876417a569fb239bfe2e2178f2434ae18653bbacc912de2b8a5f8ff0a85fad28b0c1091c2a8d3417407a37c22c1e907e4c159`

FFmpeg is licensed under LGPL-2.1-or-later for this build. OpenH264 is
BSD-2-Clause and the AMD AMF headers are MIT-licensed. See the packaged
copyright files and SPDX manifests for all notices. Stream Town itself remains
GPL-3.0-only.
