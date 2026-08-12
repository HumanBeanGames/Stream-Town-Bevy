"""Deterministically convert Stream Town FBX sources to self-contained GLB files."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import struct
import sys

import bpy
from mathutils import Matrix


SCHEMA_VERSION = 2
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
    parser.add_argument("--unity-export", type=Path)
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


def unity_model_bounds(export_path: Path | None) -> dict[str, dict[str, list[float]]]:
    if export_path is None or not export_path.is_file():
        return {}
    export = json.loads(export_path.read_text(encoding="utf-8"))
    result: dict[str, dict[str, list[float]]] = {}
    for asset in export.get("Assets", []):
        if asset.get("Kind") != "model" or not asset.get("GameObject"):
            continue
        minimum = [math.inf, math.inf, math.inf]
        maximum = [-math.inf, -math.inf, -math.inf]
        found = False
        for component in asset["GameObject"].get("Components", []):
            for field in component.get("Fields", []):
                if field.get("Path") != "bounds" or not isinstance(field.get("Value"), dict):
                    continue
                bounds = field["Value"]
                center = bounds.get("Center", {})
                size = bounds.get("Size", {})
                try:
                    for axis, key in enumerate(("x", "y", "z")):
                        half = float(size[key]) * 0.5
                        minimum[axis] = min(minimum[axis], float(center[key]) - half)
                        maximum[axis] = max(maximum[axis], float(center[key]) + half)
                    found = True
                except (KeyError, TypeError, ValueError):
                    continue
        if found:
            result[asset["Path"]] = bounds_record(minimum, maximum)
    return result


def bounds_record(minimum: list[float], maximum: list[float]) -> dict[str, list[float]]:
    return {
        "center": [(minimum[index] + maximum[index]) * 0.5 for index in range(3)],
        "size": [maximum[index] - minimum[index] for index in range(3)],
    }


def evaluated_scene_bounds() -> dict[str, list[float]] | None:
    dependency_graph = bpy.context.evaluated_depsgraph_get()
    minimum = [math.inf, math.inf, math.inf]
    maximum = [-math.inf, -math.inf, -math.inf]
    found = False
    for source_object in bpy.context.scene.objects:
        if source_object.type != "MESH":
            continue
        evaluated = source_object.evaluated_get(dependency_graph)
        for vertex in evaluated.data.vertices:
            position = evaluated.matrix_world @ vertex.co
            for axis in range(3):
                minimum[axis] = min(minimum[axis], float(position[axis]))
                maximum[axis] = max(maximum[axis], float(position[axis]))
            found = True
    return bounds_record(minimum, maximum) if found else None


def normalize_to_unity_bounds(target: dict[str, list[float]] | None) -> tuple[float, dict[str, list[float]] | None]:
    imported = evaluated_scene_bounds()
    if target is None or imported is None:
        return 1.0, imported
    imported_extent = max(imported["size"])
    target_extent = max(target["size"])
    if imported_extent <= 1.0e-9 or target_extent <= 1.0e-9:
        return 1.0, imported
    scale = target_extent / imported_extent
    if not math.isfinite(scale) or scale <= 0.0:
        raise ValueError(f"invalid Unity normalization scale {scale}")
    if not math.isclose(scale, 1.0, rel_tol=1.0e-7, abs_tol=1.0e-9):
        bake_uniform_scale(scale)
        bpy.context.view_layer.update()
    return scale, evaluated_scene_bounds()


def bake_uniform_scale(scale: float) -> None:
    """Bake Unity's effective model units into geometry, rigs, and translation curves."""
    transform = Matrix.Scale(scale, 4)
    transformed_data: set[int] = set()
    for source_object in bpy.context.scene.objects:
        source_object.location *= scale
        data = source_object.data
        if data is None or data.as_pointer() in transformed_data:
            continue
        if source_object.type == "MESH":
            data.transform(transform, shape_keys=True)
            transformed_data.add(data.as_pointer())
        elif source_object.type == "ARMATURE":
            data.transform(transform)
            transformed_data.add(data.as_pointer())
    for action in bpy.data.actions:
        for curve in action.fcurves:
            if not curve.data_path.endswith("location"):
                continue
            for keyframe in curve.keyframe_points:
                keyframe.co[1] *= scale
                keyframe.handle_left[1] *= scale
                keyframe.handle_right[1] *= scale


def preserve_unity_vertex_colors() -> None:
    """Make Unity's authored mask the glTF primary color attribute.

    The Blender FBX importer can synthesize a constant-white color layer before
    the actual FBX vertex-color layer. glTF names those layers COLOR_0 and
    COLOR_1, while Bevy intentionally imports only COLOR_0. Unity reads the
    authored layer as Mesh.colors, so discard any constant-white prefix and
    move the first meaningful layer to the active/render slot before export.
    """
    processed: set[int] = set()
    for source_object in bpy.context.scene.objects:
        if source_object.type != "MESH" or source_object.data.as_pointer() in processed:
            continue
        mesh = source_object.data
        processed.add(mesh.as_pointer())
        colors = list(mesh.color_attributes)
        if not colors:
            continue
        meaningful = next(
            (
                color
                for color in colors
                if any(
                    any(abs(float(channel) - 1.0) > 1.0e-6 for channel in datum.color)
                    for datum in color.data
                )
            ),
            None,
        )
        if meaningful is None:
            continue
        if meaningful.domain == "CORNER":
            totals = [[0.0, 0.0, 0.0, 0.0] for _ in mesh.vertices]
            counts = [0 for _ in mesh.vertices]
            for loop, datum in zip(mesh.loops, meaningful.data):
                for channel in range(4):
                    totals[loop.vertex_index][channel] += float(datum.color[channel])
                counts[loop.vertex_index] += 1
            vertex_colors = [
                tuple(channel / max(counts[index], 1) for channel in totals[index])
                for index in range(len(mesh.vertices))
            ]
        else:
            vertex_colors = [tuple(float(channel) for channel in datum.color) for datum in meaningful.data]
        for color in list(mesh.color_attributes):
            mesh.color_attributes.remove(color)
        primary = mesh.color_attributes.new(
            name="UnityColor",
            type="FLOAT_COLOR",
            domain="POINT",
        )
        for datum, value in zip(primary.data, vertex_colors):
            datum.color = value
        mesh.color_attributes.active_color = primary
        mesh.color_attributes.render_color_index = 0


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


def promote_primary_vertex_colors(path: Path) -> None:
    """Map Blender's exported authored color set to glTF COLOR_0.

    Blender 4.2 emits a constant-white COLOR_0 before an FBX `colorSet1` as
    COLOR_1. Unity exposes `colorSet1` as Mesh.colors, and Bevy loads only the
    standard COLOR_0 semantic. Rewrite the semantic table without touching the
    accessor payload, and drop higher color semantics that Bevy cannot consume.
    """
    payload = path.read_bytes()
    json_length, json_type = struct.unpack_from("<II", payload, 12)
    if json_type != 0x4E4F534A:
        raise ValueError("GLB does not begin with a JSON chunk")
    document = json.loads(payload[20 : 20 + json_length].rstrip(b"\x00 ").decode("utf-8"))
    changed = False
    for mesh in document.get("meshes", []):
        for primitive in mesh.get("primitives", []):
            attributes = primitive.get("attributes", {})
            if "COLOR_1" not in attributes:
                continue
            attributes["COLOR_0"] = attributes["COLOR_1"]
            for semantic in list(attributes):
                if semantic.startswith("COLOR_") and semantic != "COLOR_0":
                    del attributes[semantic]
            changed = True
    if not changed:
        return
    encoded = json.dumps(document, separators=(",", ":"), ensure_ascii=True).encode("utf-8")
    encoded += b" " * ((4 - len(encoded) % 4) % 4)
    remaining_chunks = payload[20 + json_length :]
    total_length = 20 + len(encoded) + len(remaining_chunks)
    rewritten = (
        struct.pack("<4sII", b"glTF", 2, total_length)
        + struct.pack("<II", len(encoded), 0x4E4F534A)
        + encoded
        + remaining_chunks
    )
    path.write_bytes(rewritten)


def convert(
    source: Path,
    output: Path,
    target_bounds: dict[str, list[float]] | None,
) -> dict[str, object]:
    reset_scene()
    bpy.ops.import_scene.fbx(filepath=str(source), use_anim=True)
    preserve_unity_vertex_colors()
    normalization_scale, output_bounds = normalize_to_unity_bounds(target_bounds)
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
    promote_primary_vertex_colors(temporary)
    metadata = inspect_glb(temporary)
    os.replace(temporary, output)
    return {
        **metadata,
        "normalization_scale": normalization_scale,
        "unity_bounds": target_bounds,
        "output_bounds": output_bounds,
    }


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
    bounds_by_source = unity_model_bounds(
        args.unity_export.resolve() if args.unity_export is not None else None
    )
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
            metadata = convert(source, output, bounds_by_source.get(source_name))
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
