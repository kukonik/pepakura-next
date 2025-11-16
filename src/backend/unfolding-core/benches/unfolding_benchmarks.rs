// benches/unfolding_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pepakura_unfolding_core::{UnfoldingCore, UnfoldingRequest, UnfoldingConfig, Mesh, Vector3};

fn create_test_cube() -> Mesh {
    let vertices = vec![
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 1.0, y: 0.0, z: 0.0 },
        Vector3 { x: 1.0, y: 1.0, z: 0.0 },
        Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        Vector3 { x: 1.0, y: 0.0, z: 1.0 },
        Vector3 { x: 1.0, y: 1.0, z: 1.0 },
        Vector3 { x: 0.0, y: 1.0, z: 1.0 },
    ];
    
    let faces = vec![
        vec![0, 1, 2, 3],
        vec![4, 7, 6, 5],
        vec![0, 4, 5, 1],
        vec![2, 6, 7, 3],
        vec![0, 3, 7, 4],
        vec![1, 5, 6, 2],
    ];

    Mesh::new(vertices, faces)
}

fn benchmark_unfolding(c: &mut Criterion) {
    let mesh = create_test_cube();
    let request = UnfoldingRequest {
        mesh,
        config: UnfoldingConfig::default(),
    };
    let core = UnfoldingCore::with_default_config();

    c.bench_function("unfold_cube", |b| {
        b.iter(|| {
            core.unfold_mesh(black_box(&request)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_unfolding);
criterion_main!(benches);