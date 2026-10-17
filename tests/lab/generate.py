#!/usr/bin/env python3
"""Generates the Parallaxe test lab: a complete library of assets and traps, built locally.

Nothing is downloaded, so every file is free to use and the lab can be rebuilt at will.
Requires Python 3.10+, Pillow and ffmpeg (with libx264, libvpx, libmp3lame, libopus, flac).

Usage:
    python3 tests/lab/generate.py OUTPUT_DIR [--force]
"""

import argparse
import json
import math
import os
import random
import shutil
import struct
import subprocess
import sys
import zipfile
import zlib
from io import BytesIO
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont, ImageOps

BULK_COUNT = 3000
DISK_NAME = "Parallaxe Test Disk"

random.seed(42)


class Lab:
    def __init__(self, root: Path):
        self.root = root
        self.library = root / "Castle Kit Library"
        self.external = root / "Godot Project - Dungeon"
        self.outside = root / "Outside Library"
        self.disk_image = root / "External Disk.sparseimage"


def font(size):
    return ImageFont.load_default(size=size)


def ffmpeg(*args):
    subprocess.run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-y", *map(str, args)], check=True)


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data if isinstance(data, bytes) else data.encode())


def png_bytes(img):
    buf = BytesIO()
    img.save(buf, format="PNG")
    return buf.getvalue()


# --- images


def texture_wood(size=1024):
    img = Image.new("RGB", (size, size))
    draw = ImageDraw.Draw(img)
    for y in range(size):
        shade = 110 + int(30 * math.sin(y / 9) + 15 * math.sin(y / 37))
        draw.line([(0, y), (size, y)], fill=(shade + 40, shade, shade - 50))
    return img.filter(ImageFilter.GaussianBlur(1.2))


def texture_brick(size=1024):
    img = Image.new("RGB", (size, size), (70, 62, 58))
    draw = ImageDraw.Draw(img)
    bh, bw = 64, 128
    for row in range(size // bh + 1):
        offset = (row % 2) * bw // 2
        for col in range(-1, size // bw + 2):
            x, y = col * bw + offset, row * bh
            tone = random.randint(-20, 20)
            draw.rectangle([x + 4, y + 4, x + bw - 4, y + bh - 4], fill=(150 + tone, 70 + tone, 50 + tone))
    return img


def texture_normal(size=1024):
    img = Image.new("RGB", (size, size), (128, 128, 255))
    draw = ImageDraw.Draw(img)
    for i in range(0, size, 64):
        draw.line([(i, 0), (i, size)], fill=(100, 128, 230), width=6)
        draw.line([(0, i), (size, i)], fill=(128, 100, 230), width=6)
    return img.filter(ImageFilter.GaussianBlur(3))


def texture_noise(size=1024, mean=128):
    return Image.effect_noise((size, size), 48).point(lambda v: max(0, min(255, v - 128 + mean))).convert("RGB")


def checker(size=256):
    img = Image.new("RGB", (size, size))
    draw = ImageDraw.Draw(img)
    cell = size // 8
    for y in range(8):
        for x in range(8):
            shade = (220, 220, 220) if (x + y) % 2 else (60, 60, 60)
            draw.rectangle([x * cell, y * cell, (x + 1) * cell, (y + 1) * cell], fill=shade)
    return img


def oriented_photo(orientation):
    """A photo whose arrow points up once its EXIF orientation is applied."""
    upright = Image.new("RGB", (800, 600), (40, 90, 140))
    draw = ImageDraw.Draw(upright)
    draw.polygon([(400, 80), (300, 260), (500, 260)], fill=(250, 220, 90))
    draw.rectangle([370, 260, 430, 480], fill=(250, 220, 90))
    draw.text((40, 520), f"UP - EXIF orientation {orientation}", fill="white", font=font(36))
    stored = {
        1: upright,
        3: upright.transpose(Image.Transpose.ROTATE_180),
        6: upright.transpose(Image.Transpose.ROTATE_90),
        8: upright.transpose(Image.Transpose.ROTATE_270),
    }[orientation]
    exif = Image.Exif()
    exif[0x0112] = orientation
    return stored, exif


def png_header_only(width, height):
    """A PNG whose header claims huge dimensions: a decompression bomb in a few bytes."""

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(b"\x00" * 64)) + chunk(b"IEND", b"")


def make_images(lab):
    textures = lab.library / "Images" / "Textures"
    textures.mkdir(parents=True, exist_ok=True)
    texture_wood().save(textures / "oak_albedo.png")
    texture_brick().save(textures / "red_brick_albedo.png")
    texture_normal().save(textures / "red_brick_normal.png")
    texture_noise(mean=160).convert("L").save(textures / "red_brick_roughness.png")
    texture_noise(mean=90).save(textures / "moss_albedo.jpg", quality=88)
    texture_wood(2048).save(textures / "oak_albedo_2k.png")

    photos = lab.library / "Images" / "Photos"
    photos.mkdir(parents=True, exist_ok=True)
    for orientation in (1, 3, 6, 8):
        stored, exif = oriented_photo(orientation)
        path = photos / f"reference_orientation_{orientation}.jpg"
        stored.save(path, quality=90, exif=exif)
        assert ImageOps.exif_transpose(Image.open(path)).size == (800, 600)
    panorama = Image.linear_gradient("L").resize((8000, 2000)).convert("RGB")
    ImageDraw.Draw(panorama).text((100, 900), "Panorama 8000 x 2000", fill="white", font=font(120))
    panorama.save(photos / "rampart_panorama.jpg", quality=85)

    ui = lab.library / "Images" / "Interface"
    ui.mkdir(parents=True, exist_ok=True)
    coin = Image.new("RGBA", (512, 512), (0, 0, 0, 0))
    ImageDraw.Draw(coin).ellipse([64, 64, 448, 448], fill=(230, 180, 40, 255))
    coin.save(ui / "gold_coin_transparent.png")
    coin.save(ui / "gold_coin_transparent.webp", quality=80)
    Image.new("RGB", (16, 16), (200, 40, 40)).save(ui / "icon_16px.png")
    frames = []
    for i in range(12):
        frame = Image.new("RGB", (128, 128), (20, 20, 30))
        ImageDraw.Draw(frame).ellipse([20 + i * 6, 44, 60 + i * 6, 84], fill=(255, 140, 0))
        frames.append(frame)
    frames[0].save(ui / "torch_animated.gif", save_all=True, append_images=frames[1:], duration=80, loop=0)


# --- audio and video


def make_audio(lab):
    effects = lab.library / "Audio" / "Effects"
    ambiences = lab.library / "Audio" / "Ambiences"
    effects.mkdir(parents=True, exist_ok=True)
    ambiences.mkdir(parents=True, exist_ok=True)
    ffmpeg("-f", "lavfi", "-i", "sine=frequency=880:duration=0.08", "-ac", "1", "-ar", "44100", effects / "ui_click.wav")
    ffmpeg("-f", "lavfi", "-i", "sine=frequency=220:duration=2", "-ac", "1", "-c:a", "pcm_s16le", effects / "door_creak_mono.wav")
    ffmpeg("-f", "lavfi", "-i", "anoisesrc=color=pink:duration=3", "-ac", "2", "-ar", "48000", "-c:a", "pcm_s24le",
           effects / "sword_stereo_24bit.wav")
    ffmpeg("-f", "lavfi", "-i", "sine=frequency=440:duration=4", "-c:a", "libmp3lame", "-b:a", "192k", effects / "bell_cbr.mp3")
    ffmpeg("-f", "lavfi", "-i", "anoisesrc=color=brown:duration=180", "-c:a", "libmp3lame", "-q:a", "2",
           ambiences / "night_forest_3min_vbr.mp3")
    ffmpeg("-f", "lavfi", "-i", "anoisesrc=color=white:duration=20:amplitude=0.2", "-c:a", "flac", ambiences / "rain.flac")
    ffmpeg("-f", "lavfi", "-i", "sine=frequency=330:duration=6", "-c:a", "libopus", ambiences / "cave_wind.ogg")


def make_video(lab):
    out = lab.library / "Video"
    out.mkdir(parents=True, exist_ok=True)
    source = ["-f", "lavfi", "-i", "testsrc2=size=1280x720:rate=30:duration=5",
              "-f", "lavfi", "-i", "sine=frequency=500:duration=5"]
    ffmpeg(*source, "-c:v", "libx264", "-pix_fmt", "yuv420p", "-c:a", "aac", "-shortest", out / "cutscene_h264.mp4")
    ffmpeg(*source, "-c:v", "libvpx-vp9", "-b:v", "1M", "-c:a", "libopus", "-shortest", out / "cutscene_vp9.webm")
    ffmpeg("-f", "lavfi", "-i", "testsrc2=size=640x360:rate=24:duration=4", "-c:v", "mpeg4", out / "old_codec_mpeg4.mp4")
    ffmpeg("-f", "lavfi", "-i", "testsrc2=size=1080x1920:rate=30:duration=3", "-c:v", "libx264", "-pix_fmt", "yuv420p",
           "-an", out / "vertical_no_audio.mp4")


# --- 3D


def cube_geometry():
    faces = [
        ((0, 0, 1), [(-1, -1, 1), (1, -1, 1), (1, 1, 1), (-1, 1, 1)]),
        ((0, 0, -1), [(1, -1, -1), (-1, -1, -1), (-1, 1, -1), (1, 1, -1)]),
        ((1, 0, 0), [(1, -1, 1), (1, -1, -1), (1, 1, -1), (1, 1, 1)]),
        ((-1, 0, 0), [(-1, -1, -1), (-1, -1, 1), (-1, 1, 1), (-1, 1, -1)]),
        ((0, 1, 0), [(-1, 1, 1), (1, 1, 1), (1, 1, -1), (-1, 1, -1)]),
        ((0, -1, 0), [(-1, -1, -1), (1, -1, -1), (1, -1, 1), (-1, -1, 1)]),
    ]
    positions, normals, uvs, indices = [], [], [], []
    for normal, corners in faces:
        base = len(positions)
        positions += corners
        normals += [normal] * 4
        uvs += [(0, 1), (1, 1), (1, 0), (0, 0)]
        indices += [base, base + 1, base + 2, base, base + 2, base + 3]
    return positions, normals, uvs, indices


def grid_geometry(n):
    """A wavy plane of n x n quads: 2 n² triangles."""
    positions, normals, uvs, indices = [], [], [], []
    for j in range(n + 1):
        for i in range(n + 1):
            x, z = i / n * 4 - 2, j / n * 4 - 2
            positions.append((x, 0.2 * math.sin(x * 3) * math.cos(z * 3), z))
            normals.append((0, 1, 0))
            uvs.append((i / n, j / n))
    for j in range(n):
        for i in range(n):
            a = j * (n + 1) + i
            indices += [a, a + n + 1, a + 1, a + 1, a + n + 1, a + n + 2]
    return positions, normals, uvs, indices


class Gltf:
    """A minimal glTF 2.0 writer, enough for valid test files."""

    def __init__(self):
        self.doc = {"asset": {"version": "2.0", "generator": "Parallaxe test lab"}, "buffers": [], "bufferViews": [],
                    "accessors": [], "meshes": [], "nodes": [], "scenes": [], "scene": 0}
        self.bin = bytearray()

    def _view(self, data, target=None):
        while len(self.bin) % 4:
            self.bin.append(0)
        view = {"buffer": 0, "byteOffset": len(self.bin), "byteLength": len(data)}
        if target:
            view["target"] = target
        self.bin += data
        self.doc["bufferViews"].append(view)
        return len(self.doc["bufferViews"]) - 1

    def _accessor(self, values, kind, component, fmt, target, with_bounds=False):
        flat = [c for v in values for c in (v if isinstance(v, tuple) else (v,))]
        view = self._view(struct.pack(f"<{len(flat)}{fmt}", *flat), target)
        accessor = {"bufferView": view, "componentType": component, "count": len(values), "type": kind}
        if with_bounds:
            accessor["min"] = [min(v[i] for v in values) for i in range(3)]
            accessor["max"] = [max(v[i] for v in values) for i in range(3)]
        self.doc["accessors"].append(accessor)
        return len(self.doc["accessors"]) - 1

    def mesh(self, geometry, material=None, mode=4, name="mesh"):
        positions, normals, uvs, indices = geometry
        attributes = {"POSITION": self._accessor(positions, "VEC3", 5126, "f", 34962, True)}
        if normals:
            attributes["NORMAL"] = self._accessor(normals, "VEC3", 5126, "f", 34962)
        if uvs:
            attributes["TEXCOORD_0"] = self._accessor(uvs, "VEC2", 5126, "f", 34962)
        primitive = {"attributes": attributes, "mode": mode}
        if indices:
            large = len(positions) > 65535
            primitive["indices"] = self._accessor(indices, "SCALAR", 5125 if large else 5123, "I" if large else "H", 34963)
        if material is not None:
            primitive["material"] = material
        self.doc["meshes"].append({"name": name, "primitives": [primitive]})
        return len(self.doc["meshes"]) - 1

    def node(self, mesh=None, translation=None, name="node"):
        node = {"name": name}
        if mesh is not None:
            node["mesh"] = mesh
        if translation:
            node["translation"] = list(translation)
        self.doc["nodes"].append(node)
        return len(self.doc["nodes"]) - 1

    def scene(self, nodes, name="Scene"):
        self.doc["scenes"].append({"name": name, "nodes": nodes})

    def material(self, color=(0.8, 0.8, 0.8, 1), texture_uri=None, embedded_png=None):
        pbr = {"baseColorFactor": list(color), "metallicFactor": 0, "roughnessFactor": 0.8}
        if texture_uri or embedded_png:
            if texture_uri:
                image = {"uri": texture_uri}
            else:
                image = {"bufferView": self._view(embedded_png), "mimeType": "image/png"}
            self.doc.setdefault("images", []).append(image)
            self.doc.setdefault("textures", []).append({"source": len(self.doc["images"]) - 1})
            pbr["baseColorTexture"] = {"index": len(self.doc["textures"]) - 1}
        self.doc.setdefault("materials", []).append({"pbrMetallicRoughness": pbr})
        return len(self.doc["materials"]) - 1

    def rotation_animation(self, node):
        times = self._accessor([0.0, 1.0, 2.0], "SCALAR", 5126, "f", None)
        self.doc["accessors"][times].update({"min": [0.0], "max": [2.0]})
        rotations = self._accessor([(0, 0, 0, 1), (0, 0.7071, 0, 0.7071), (0, 1, 0, 0)], "VEC4", 5126, "f", None)
        self.doc["animations"] = [{
            "name": "rotation",
            "samplers": [{"input": times, "output": rotations}],
            "channels": [{"sampler": 0, "target": {"node": node, "path": "rotation"}}],
        }]

    def save_glb(self, path):
        self.doc["buffers"] = [{"byteLength": len(self.bin)}]
        js = json.dumps(self.doc).encode()
        js += b" " * (-len(js) % 4)
        binary = bytes(self.bin) + b"\x00" * (-len(self.bin) % 4)
        body = struct.pack("<I4s", len(js), b"JSON") + js + struct.pack("<I4s", len(binary), b"BIN\x00") + binary
        write(path, struct.pack("<4sII", b"glTF", 2, 12 + len(body)) + body)

    def save_gltf(self, path, bin_name):
        self.doc["buffers"] = [{"uri": bin_name, "byteLength": len(self.bin)}]
        write(path.parent / bin_name, bytes(self.bin))
        write(path, json.dumps(self.doc, indent=2))


def single_cube(texture_uri=None, color=(0.8, 0.8, 0.8, 1)):
    gltf = Gltf()
    gltf.scene([gltf.node(gltf.mesh(cube_geometry(), gltf.material(color, texture_uri=texture_uri)))])
    return gltf


def make_3d(lab):
    base = lab.library / "3D"
    gltf_dir, obj_dir, stl_dir, other_dir = base / "glTF", base / "OBJ", base / "STL", base / "Other formats"
    for folder in (gltf_dir / "textures", obj_dir / "textures", stl_dir, other_dir):
        folder.mkdir(parents=True, exist_ok=True)

    g = Gltf()
    g.scene([g.node(g.mesh(cube_geometry(), g.material(embedded_png=png_bytes(checker()))), name="Crate")])
    g.save_glb(gltf_dir / "crate_embedded_texture.glb")

    texture_wood(512).save(gltf_dir / "textures" / "crate_wood.png")
    single_cube("textures/crate_wood.png").save_gltf(gltf_dir / "crate_external.gltf", "crate_external.bin")

    g = Gltf()
    pillar = g.mesh(cube_geometry(), g.material((0.6, 0.6, 0.65, 1)), name="Pillar")
    g.scene([g.node(pillar, (x * 3, 0, z * 3), name=f"Pillar_{x}_{z}") for x in range(8) for z in range(8)])
    g.save_glb(gltf_dir / "colonnade_64_instances.glb")

    g = Gltf()
    red = g.node(g.mesh(cube_geometry(), g.material((0.9, 0.2, 0.2, 1))), name="Red")
    ground = g.node(g.mesh(grid_geometry(10), g.material((0.2, 0.9, 0.2, 1))), name="Ground")
    g.scene([red], "Scene A")
    g.scene([ground], "Scene B")
    g.save_glb(gltf_dir / "two_scenes.glb")

    g = Gltf()
    cloud = [(random.uniform(-2, 2), random.uniform(-2, 2), random.uniform(-2, 2)) for _ in range(500)]
    g.scene([g.node(g.mesh((cloud, [], [], []), mode=0, name="Cloud"), name="Points")])
    g.save_glb(gltf_dir / "point_cloud_no_triangles.glb")

    g = Gltf()
    spinning = g.node(g.mesh(cube_geometry(), g.material((0.3, 0.5, 0.9, 1))), name="Spinning")
    g.scene([spinning])
    g.rotation_animation(spinning)
    g.save_glb(gltf_dir / "animated_cube.glb")

    g = Gltf()
    g.scene([g.node(g.mesh(grid_geometry(500), g.material((0.5, 0.45, 0.4, 1)), name="Terrain"), name="Terrain")])
    g.save_glb(gltf_dir / "heavy_terrain_500k_triangles.glb")

    single_cube("textures/absent.png").save_gltf(gltf_dir / "missing_texture.gltf", "missing_texture.bin")
    # resolves to the real secret file: a viewer must refuse it, not merely fail to find it
    single_cube("../../../Outside Library/secret.png").save_gltf(gltf_dir / "path_escape.gltf", "path_escape.bin")
    single_cube("https://example.com/texture.png").save_gltf(gltf_dir / "remote_texture.gltf", "remote_texture.bin")
    valid = (gltf_dir / "crate_embedded_texture.glb").read_bytes()
    write(gltf_dir / "truncated.glb", valid[: len(valid) // 2])

    make_barrel(obj_dir)
    write(obj_dir / "tetrahedron_triangulated.obj",
          "v 0 0 0\nv 1 0 0\nv 0 1 0\nv 0 0 1\nf 1 3 2\nf 1 2 4\nf 1 4 3\nf 2 3 4\n")
    write(obj_dir / "statue_missing_mtl.obj",
          "mtllib nowhere.mtl\nusemtl Stone\nv 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n")

    tower = tower_triangles()
    write(stl_dir / "tower_ascii.stl", stl_ascii(tower))
    binary = stl_binary(tower)
    write(stl_dir / "tower_binary.stl", binary)
    write(stl_dir / "truncated.stl", binary[: len(binary) // 3])

    write(other_dir / "character.fbx", b"Kaydara FBX Binary  \x00\x1a\x00" + os.urandom(2048))
    write(other_dir / "scene.blend", b"BLENDER-v402" + os.urandom(4096))
    write(other_dir / "part.step", "ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n")
    write(other_dir / "set.usda", '#usda 1.0\ndef Xform "Set" {}\n')


def make_barrel(folder, segments=32):
    lines = ["# Barrel made of quads, triangulated on import", "mtllib barrel.mtl", "usemtl Wood"]
    for y in (0, 2):
        for s in range(segments):
            angle = 2 * math.pi * s / segments
            lines.append(f"v {math.cos(angle):.4f} {y} {math.sin(angle):.4f}")
    for s in range(segments + 1):
        lines += [f"vt {s / segments:.4f} 0", f"vt {s / segments:.4f} 1"]
    for s in range(segments):
        a, b = s + 1, (s + 1) % segments + 1
        lines.append(f"f {a}/{2 * s + 1} {b}/{2 * s + 3} {b + segments}/{2 * s + 4} {a + segments}/{2 * s + 2}")
    write(folder / "barrel_quads.obj", "\n".join(lines) + "\n")
    write(folder / "barrel.mtl", "newmtl Wood\nKd 0.6 0.4 0.2\nmap_Kd textures/barrel_diffuse.png\n")
    texture_wood(512).save(folder / "textures" / "barrel_diffuse.png")


def tower_triangles():
    triangles = []
    for level in range(5):
        size = 1.0 - level * 0.15
        positions, _, _, indices = cube_geometry()
        vertices = [(x * size, (y + 1) / 2 + level, z * size) for x, y, z in positions]
        triangles += [tuple(vertices[indices[i + k]] for k in range(3)) for i in range(0, len(indices), 3)]
    return triangles


def stl_ascii(triangles):
    out = ["solid tower"]
    for triangle in triangles:
        out += ["  facet normal 0 0 0", "    outer loop", *(f"      vertex {x} {y} {z}" for x, y, z in triangle),
                "    endloop", "  endfacet"]
    return "\n".join(out + ["endsolid tower\n"])


def stl_binary(triangles):
    data = bytearray(b"Parallaxe test lab".ljust(80, b" ") + struct.pack("<I", len(triangles)))
    for triangle in triangles:
        data += struct.pack("<3f", 0, 0, 0) + b"".join(struct.pack("<3f", *v) for v in triangle) + b"\x00\x00"
    return bytes(data)


# --- misc and traps


def make_misc(lab):
    out = lab.library / "Misc"
    out.mkdir(parents=True, exist_ok=True)
    write(out / "production_notes.txt", "Assets left to finish for the dungeon.\n")
    write(out / "LICENSE", "CC0, generated for Parallaxe tests.\n")
    write(out / "palette.ase", os.urandom(512))
    write(out / "mockup.psd", b"8BPS" + os.urandom(1024))
    with zipfile.ZipFile(out / "texture_pack.zip", "w") as archive:
        archive.writestr("inside_the_archive.png", png_bytes(checker(64)))


def make_traps(lab):
    out = lab.library / "Traps"
    out.mkdir(parents=True, exist_ok=True)
    unlock(lab)
    valid = png_bytes(checker(128))
    write(out / "empty_0_bytes.png", b"")
    write(out / "truncated.png", valid[: len(valid) // 3])
    write(out / "not_an_image.png", "This is text, not an image.\n")
    write(out / "real_image_without_extension", valid)
    write(out / "UPPERCASE.PNG", valid)
    write(out / "jpeg_disguised_as.png", (lab.library / "Images" / "Textures" / "moss_albedo.jpg").read_bytes())
    write(out / "not_music.mp3", "not a sound\n")
    write(out / "too_wide_20000px.png", png_header_only(20000, 20))
    write(out / "bomb_40000x40000.png", png_header_only(40000, 40000))
    write(out / "été 2026 — référence spéciale.png", valid)
    write(out / "纹理_中文.png", valid)
    write(out / "palette 🎨 emoji.png", valid)
    write(out / ("very_long_name_" + "x" * 180 + ".png"), valid)
    write(out.joinpath(*[f"level_{i}" for i in range(12)]) / "deep_down.png", valid)
    write(out / ".DS_Store", b"\x00" * 16)
    write(out / "Thumbs.db", b"\x00" * 16)
    write(out / ".hidden_cache" / "invisible.png", valid)
    write(out / ".parallaxe" / "future_portable_folder.json", "{}")

    links = [("loop_link", "."), ("link_outside_library", str(lab.outside)),
             ("link_to_texture.png", "../Images/Textures/oak_albedo.png")]
    for name, target in links:
        path = out / name
        if path.is_symlink() or path.exists():
            path.unlink()
        try:
            path.symlink_to(target)
        except OSError:
            print(f"  symlink skipped (not allowed here): {name}")

    locked = out / "Locked folder"
    write(locked / "unreachable.png", valid)
    if os.name == "posix":
        locked.chmod(0o000)


def unlock(lab):
    locked = lab.library / "Traps" / "Locked folder"
    if locked.exists():
        locked.chmod(0o755)


def make_bulk(lab):
    for i in range(BULK_COUNT):
        folder = lab.library / "Bulk" / f"batch_{i // 250:02d}"
        folder.mkdir(parents=True, exist_ok=True)
        hue = int(((i * 37) % 360) / 360 * 255)
        tile = Image.new("HSV", (64, 64), (hue, 160, 200)).convert("RGB")
        ImageDraw.Draw(tile).text((6, 22), f"{i:04d}", fill="white", font=font(18))
        tile.save(folder / f"tile_{i:04d}.png")


def make_external(lab):
    assets = lab.external / "assets"
    write(lab.external / "project.godot", '[application]\nconfig/name="Dungeon"\n')
    write(lab.external / ".godot" / "imported" / "cache.ctex", os.urandom(256))
    (assets / "walls").mkdir(parents=True, exist_ok=True)
    (assets / "sounds").mkdir(parents=True, exist_ok=True)
    texture_brick(512).save(assets / "walls" / "dungeon_wall.png")
    single_cube(color=(0.4, 0.3, 0.2, 1)).save_glb(assets / "chest.glb")
    ffmpeg("-f", "lavfi", "-i", "sine=frequency=660:duration=1", assets / "sounds" / "chest_open.wav")


def make_outside(lab):
    lab.outside.mkdir(parents=True, exist_ok=True)
    write(lab.outside / "secret.txt", "If you can see this file in Parallaxe, it is a security bug.\n")
    texture_noise(256).save(lab.outside / "secret.png")


def make_disk(lab):
    """A sparse disk image to plug and unplug, macOS only."""
    if sys.platform != "darwin":
        print("  skipped: disk images need macOS (use a USB stick elsewhere)")
        return
    if lab.disk_image.exists():
        return
    subprocess.run(["hdiutil", "create", "-quiet", "-type", "SPARSE", "-size", "200m", "-fs", "APFS",
                    "-volname", DISK_NAME, str(lab.disk_image.with_suffix(""))], check=True)
    subprocess.run(["hdiutil", "attach", "-quiet", str(lab.disk_image)], check=True)
    volume = Path("/Volumes") / DISK_NAME
    try:
        (volume / "Sound bank").mkdir(parents=True, exist_ok=True)
        for i, frequency in enumerate((300, 450, 600, 750, 900), start=1):
            ffmpeg("-f", "lavfi", "-i", f"sine=frequency={frequency}:duration=1", volume / "Sound bank" / f"impact_{i}.wav")
        texture_noise(512).save(volume / "external_disk_texture.png")
    finally:
        subprocess.run(["hdiutil", "detach", "-quiet", str(volume)], check=True)


EXPECTED = {
    "scan": {
        "Castle Kit Library": {"files": 3073, "skipped_symlinks": 3, "unreadable_folders": 1},
        "Godot Project - Dungeon": {"files": 4},
        "Parallaxe Test Disk": {"files": 6},
        "never_cataloged": ["Outside Library/secret.txt", "Outside Library/secret.png"],
    },
    "models": {
        "3D/glTF/crate_embedded_texture.glb": {"triangles": 12, "vertices": 24, "materials": 1, "textures": 1},
        "3D/glTF/crate_external.gltf": {"triangles": 12, "textures": 1, "missing_references": []},
        "3D/glTF/colonnade_64_instances.glb": {"meshes": 1, "instances": 64, "stored_triangles": 12,
                                               "rendered_triangles": 768},
        "3D/glTF/two_scenes.glb": {"scenes": 2},
        "3D/glTF/point_cloud_no_triangles.glb": {"triangles": 0, "points": 500},
        "3D/glTF/animated_cube.glb": {"animations": 1},
        "3D/glTF/heavy_terrain_500k_triangles.glb": {"triangles": 500000},
        "3D/glTF/missing_texture.gltf": {"missing_references": ["textures/absent.png"]},
        "3D/glTF/path_escape.gltf": {"refused_references": ["../../../Outside Library/secret.png"]},
        "3D/glTF/remote_texture.gltf": {"refused_references": ["https://example.com/texture.png"]},
        "3D/glTF/truncated.glb": {"error": True},
        "3D/OBJ/barrel_quads.obj": {"triangles_after_triangulation": 64},
        "3D/OBJ/statue_missing_mtl.obj": {"missing_references": ["nowhere.mtl"]},
        "3D/STL/tower_ascii.stl": {"triangles": 60, "materials": "not_applicable", "animations": "not_applicable"},
        "3D/STL/tower_binary.stl": {"triangles": 60, "materials": "not_applicable", "animations": "not_applicable"},
        "3D/STL/truncated.stl": {"error": True},
    },
    "images": {
        "Images/Photos/reference_orientation_6.jpg": {"stored": [600, 800], "displayed": [800, 600]},
        "Images/Photos/reference_orientation_8.jpg": {"stored": [600, 800], "displayed": [800, 600]},
        "Traps/bomb_40000x40000.png": {"thumbnail": False},
        "Traps/too_wide_20000px.png": {"thumbnail": False},
        "Traps/jpeg_disguised_as.png": {"thumbnail": True},
    },
    "media": {
        "Audio/Ambiences/night_forest_3min_vbr.mp3": {"duration_s": 180},
        "Audio/Effects/sword_stereo_24bit.wav": {"channels": 2, "sample_rate": 48000, "bits_per_sample": 24},
        "Video/old_codec_mpeg4.mp4": {"video_codec": "mpeg4", "playable_in_webview": False},
    },
}


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("output", type=Path, help="folder that will contain the lab")
    parser.add_argument("--force", action="store_true", help="delete and rebuild the library folders")
    args = parser.parse_args()
    if shutil.which("ffmpeg") is None:
        sys.exit("ffmpeg is required: https://ffmpeg.org/download.html")

    lab = Lab(args.output.resolve())
    if args.force:
        unlock(lab)
        for path in (lab.library, lab.external, lab.outside):
            shutil.rmtree(path, ignore_errors=True)

    steps = [("outside library", make_outside), ("images", make_images), ("audio", make_audio),
             ("video", make_video), ("3D", make_3d), ("misc", make_misc), ("traps", make_traps),
             (f"bulk ({BULK_COUNT} files)", make_bulk), ("external Godot project", make_external),
             ("external disk", make_disk)]
    for label, step in steps:
        print(f"-> {label}")
        step(lab)
    write(lab.root / "expected.json", json.dumps(EXPECTED, indent=2, ensure_ascii=False) + "\n")
    print(f"Lab ready: {lab.root}")


if __name__ == "__main__":
    main()
