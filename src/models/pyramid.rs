use nalgebra as na;



type V3 = na::Vector3<f32>;



// TODO: make this const
pub fn create_pyramid() -> ([V3; 18], [V3; 18]) {
    let vertices = [
        V3::new(1.0, 0.0, -1.0),
        V3::new(-1.0, 0.0, -1.0),
        V3::new(1.0, 0.0, 1.0),
        V3::new(-1.0, 0.0, -1.0),
        V3::new(-1.0, 0.0, 1.0),
        V3::new(1.0, 0.0, 1.0),
        V3::new(1.0, 0.0, -1.0),
        V3::new(0.0, 1.0, 0.0),
        V3::new(-1.0, 0.0, -1.0),
        V3::new(1.0, 0.0, 1.0),
        V3::new(0.0, 1.0, 0.0),
        V3::new(1.0, 0.0, -1.0),
        V3::new(-1.0, 0.0, 1.0),
        V3::new(0.0, 1.0, 0.0),
        V3::new(1.0, 0.0, 1.0),
        V3::new(-1.0, 0.0, -1.0),
        V3::new(0.0, 1.0, 0.0),
        V3::new(-1.0, 0.0, 1.0),
    ];

    let q = 1.0 / 5.0_f32.sqrt();
    let p = q * 2.0;

    let normals = [
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, -1.0, 0.0),
        V3::new(0.0, q, -p),
        V3::new(0.0, q, -p),
        V3::new(0.0, q, -p),
        V3::new(p, q, 0.0),
        V3::new(p, q, 0.0),
        V3::new(p, q, 0.0),
        V3::new(0.0, q, p),
        V3::new(0.0, q, p),
        V3::new(0.0, q, p),
        V3::new(-p, q, 0.0),
        V3::new(-p, q, 0.0),
        V3::new(-p, q, 0.0),
    ];

    (vertices, normals)
}