struct Transformation {
    position: [f32; 3],
    rotation: [f32; 4],
}

enum Visual {
    Mesh {
        name: String,
        transformation: Transformation,
        vertices: Vec<[f32; 3]>,
        indices: Vec<u32>,
    },
    Sphere {
        name: String,
        transformation: Transformation,
        radius: f32,
    },
    Cylinder {
        name: String,
        transformation: Transformation,
        radius: f32,
        height: f32,
    },
    Box {
        name: String,
        transformation: Transformation,
        size: [f32; 3],
    },
    Plane {
        name: String,
        transformation: Transformation,
        size: [f32; 2],
    },
}

struct Body {
    name: String,
    transformation: Transformation,
    inertia: [f32; 9],
    joints: Vec<Joint>,
    visuals: Vec<Visual>,
}

struct Joint {
    name: String,
    transformation: Transformation,
}
