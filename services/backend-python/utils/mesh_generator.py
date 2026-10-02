# File: services/backend-python/utils/mesh_generator.py

def generate_cube(dimensions=None, color='#ffffff'):
    """Генерирует примитивный OBJ куб."""
    obj_str = '''# Pepakura Cube Generator
o MyCube

# Vertices
v -0.5 -0.5 0.5
v 0.5 -0.5 0.5
v -0.5 0.5 0.5
v 0.5 0.5 0.5
v -0.5 -0.5 -0.5
v 0.5 -0.5 -0.5
v -0.5 0.5 -0.5
v 0.5 0.5 -0.5

# Texture Coordinates
vt 0.000000 0.000000
vt 1.000000 0.000000
vt 1.000000 1.000000
vt 0.000000 1.000000

# Normals
vn 0.0000 0.0000 1.0000
vn 0.0000 0.0000 -1.0000

# Faces
f 1/1/1 2/2/1 4/3/1 3/4/1
f 5/1/2 6/2/2 8/3/2 7/4/2'''

    return obj_str


import math

def generate_cylinder(dimensions=None, color="#ffffff"):
    """Генерация цилиндра с указанными параметрами"""
    segments = 16
    height = dimensions.get("height", 2.0) if dimensions else 2.0
    radius = dimensions.get("radius", 1.0) if dimensions else 1.0

    vertices = []
    normals = []
    texcoords = []

    # Точки верхней и нижней окружностей
    for i in range(segments):
        angle = 2 * 3.14159 * i / segments
        x = radius * math.cos(angle)
        z = radius * math.sin(angle)
        
        vertices.append(f"v {x:.6f} {-height/2:.6f} {z:.6f}")
        vertices.append(f"v {x:.6f} {height/2:.6f} {z:.6f}")

        normal_x = x / radius
        normal_z = z / radius
        normals.append(f"vn {normal_x:.6f} 0.000000 {normal_z:.6f}")
        normals.append(f"vn {normal_x:.6f} 0.000000 {normal_z:.6f}")

        u = i / (segments - 1)
        texcoords.append(f"vt {u:.6f} 0.000000")
        texcoords.append(f"vt {u:.6f} 1.000000")

    faces = []
    for i in range(segments):
        base_idx = i * 2 + 1
        next_base_idx = ((i + 1) % segments) * 2 + 1
        face = (
            f"f {base_idx}/{base_idx}/{base_idx} "
            f"{next_base_idx}/{next_base_idx}/{next_base_idx} "
            f"{next_base_idx+1}/{next_base_idx+1}/{next_base_idx+1} "
            f"{base_idx+1}/{base_idx+1}/{base_idx+1}"
        )
        faces.append(face)

    obj_lines = ["# Pepakura Generated Cylinder", "o MyCylinder"]
    obj_lines.extend(vertices)
    obj_lines.extend(normals)
    obj_lines.extend(texcoords)
    obj_lines.extend(faces)
    
    return "\n".join(obj_lines)


def generate_pyramid(dimensions=None, color="#ffffff"):
    """Генерация пирамиды"""
    half_width = dimensions.get("width", 1.0) / 2 if dimensions else 0.5
    height = dimensions.get("height", 2.0) if dimensions else 2.0
    depth = dimensions.get("depth", 1.0) if dimensions else 1.0

    vertices = [
        f"v {-half_width} 0 {-depth}",
        f"v {half_width} 0 {-depth}",
        f"v 0 {height} 0",

        f"v {-half_width} 0 {depth}",
        f"v {half_width} 0 {depth}",
    ]

    faces = [
        "f 1 2 3",  # Передняя
        "f 2 5 3",  # Правая
        "f 5 4 3",  # Задняя
        "f 4 1 3",  # Левая
        "f 1 2 5",  # Основание #1
        "f 1 5 4",  # Основание #2
    ]

    obj_lines = ["# Pepakura Generated Pyramid", "o MyPyramid"]
    obj_lines.extend(vertices)
    obj_lines.extend(faces)
    
    return "\n".join(obj_lines)


def generate_sphere(dimensions=None, color="#ffffff"):
    import math

    latitude_bands = 16
    longitude_bands = 16
    radius = dimensions.get("radius", 1.0) if dimensions else 1.0

    vertices = []
    normals = []
    texcoords = []

    for lat in range(latitude_bands + 1):
        theta = lat * math.pi / latitude_bands
        sinTheta = math.sin(theta)
        cosTheta = math.cos(theta)

        for long in range(longitude_bands + 1):
            phi = long * 2 * math.pi / longitude_bands
            sinPhi = math.sin(phi)
            cosPhi = math.cos(phi)

            x = cosPhi * sinTheta
            y = cosTheta
            z = sinPhi * sinTheta

            u = 1.0 - (long / longitude_bands)
            v = 1.0 - (lat / latitude_bands)

            vertices.append(f"v {x * radius:.6f} {y * radius:.6f} {z * radius:.6f}")
            normals.append(f"vn {x:.6f} {y:.6f} {z:.6f}")
            texcoords.append(f"vt {u:.6f} {v:.6f}")

    indices = []
    for lat in range(latitude_bands):
        for long in range(longitude_bands):
            first = (lat * (longitude_bands + 1)) + long
            second = first + longitude_bands + 1
            indices.append(f"f {first + 1} {second + 1} {first + 2}")
            indices.append(f"f {second + 1} {second + 2} {first + 2}")

    obj_lines = ["# Pepakura Generated Sphere", "o MySphere"]
    obj_lines.extend(vertices)
    obj_lines.extend(normals)
    obj_lines.extend(texcoords)
    obj_lines.extend(indices)

    return "\n".join(obj_lines)

shape_generators = {
    'box': generate_cube,
    'cylinder': generate_cylinder,
    'pyramid': generate_pyramid,
    'sphere': generate_sphere,
}
