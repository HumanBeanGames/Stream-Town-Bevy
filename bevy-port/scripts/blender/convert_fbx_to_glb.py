"""Deterministically convert Stream Town FBX sources to self-contained GLB files."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import struct
import sys

import bpy


SCHEMA_VERSION = 1
EXCLUDED_PARTS = {
    "astarpathfindingproject",
    "migrationonly",
    "plugins",
    "reflexoverride",
    "textmesh pro",
}


def arguments() -> argparse.Namespace:
    values = sys.argv[sys.argv.index("--") + 1 :] if "--" in sys.argv else []
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--only", action="append", default=[])
    return parser.parse_args(values)


def normalized_relative(path: Path, root: Path) -> str:
    return path.resolve().relative_to(root.resolve()).as_posix()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def discover_sources(repo_root: Path, only: list[str]) -> list[Path]:
    assets_root = repo_root / "Assets"
    if only:
        sources = []
        for value in only:
            candidate = Path(value)
            if not candidate.is_absolute():
                candidate = repo_root / candidate
            candidate = candidate.resolve()
            candidate.relative_to(assets_root.resolve())
            if candidate.suffix.lower() != ".fbx" or not candidate.is_file():
                raise ValueError(f"not an FBX source file: {candidate}")
            sources.append(candidate)
    else:
        sources = [
            path
            for path in assets_root.rglob("*.fbx")
            if not any(part.lower() in EXCLUDED_PARTS for part in path.parts)
        ]
    return sorted(set(sources), key=lambda path: normalized_relative(path, repo_root))


def reset_scene() -> None:
    bpy.ops.wm.read_factory_settings(use_empty=True)


def inspect_glb(path: Path) -> dict[str, int]:
    payload = path.read_bytes()
    if len(payload) < 20:
        raise ValueError("GLB is shorter than its required header and JSON chunk")
    magic, version, declared_length = struct.unpack_from("<4sII", payload, 0)
    if magic != b"glTF" or version != 2 or declared_length != len(payload):
        raise ValueError("GLB header magic, version, or declared length is invalid")
    json_length, json_type = struct.unpack_from("<II", payload, 12)
    if json_type != 0x4E4F534A or 20 + json_length > len(payload):
        raise ValueError("GLB does not begin with a valid JSON chunk")
    document = json.loads(payload[20 : 20 + json_length].rstrip(b"\x00 ").decode("utf-8"))
    if document.get("asset", {}).get("version") != "2.0":
        raise ValueError("GLB JSON does not declare glTF 2.0")
    return {
        "meshes": len(document.get("meshes", [])),
        "skins": len(document.get("skins", [])),
        "animations": len(document.get("animations", [])),
        "materials": len(document.get("materials", [])),
        "images": len(document.get("images", [])),
    }


def convert(source: Path, output: Path) -> dict[str, object]:
    reset_scene()
    bpy.ops.import_scene.fbx(filepath=str(source), use_anim=True)
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_name(f"{output.stem}.tmp.glb")
    bpy.ops.export_scene.gltf(
        filepath=str(temporary),
        export_format="GLB",
        export_yup=True,
        export_animations=True,
        export_skins=True,
        export_morph=True,
        export_materials="EXPORT",
    )
    metadata = inspect_glb(temporary)
    os.replace(temporary, output)
    return metadata


def write_report(path: Path, report: dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(
        json.dumps(report, indent=2, sort_keys=True, ensure_ascii=True) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    os.replace(temporary, path)


def main() -> int:
    args = arguments()
    repo_root = args.repo_root.resolve()
    assets_root = (repo_root / "Assets").resolve()
    output_root = args.output_root.resolve()
    sources = discover_sources(repo_root, args.only)
    entries: list[dict[str, object]] = []
    failures: list[str] = []
    for index, source in enumerate(sources, start=1):
        source_relative_assets = source.relative_to(assets_root)
        output = output_root / source_relative_assets.with_suffix(".glb")
        source_name = normalized_relative(source, repo_root)
        output_name = normalized_relative(output, repo_root)
        print(f"[{index}/{len(sources)}] {source_name}", flush=True)
        try:
            metadata = convert(source, output)
            entries.append(
                {
                    "source": source_name,
                    "output": output_name,
                    "source_sha256": sha256(source),
                    "output_sha256": sha256(output),
                    "output_bytes": output.stat().st_size,
                    **metadata,
                }
            )
        except Exception as error:  # Blender exceptions vary by importer/exporter.
            failures.append(f"{source_name}: {type(error).__name__}: {error}")
    report = {
        "schema_version": SCHEMA_VERSION,
        "blender_version": bpy.app.version_string,
        "source_model_count": len(sources),
        "entries": entries,
        "failures": sorted(failures),
    }
    write_report(args.report.resolve(), report)
    print(
        f"Converted {len(entries)}/{len(sources)} FBX files; {len(failures)} failures",
        flush=True,
    )
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
