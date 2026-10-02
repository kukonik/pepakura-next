use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pepakura_core::geometry::mesh::{Mesh, Face};
use pepakura_core::geometry::vertex::Vertex;
use pepakura_core::unfold::mds::{unfold_mds, UnfoldConfig};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Generate a random mesh with n vertices and approximately 2n faces.
fn random_mesh(n: usize, seed: u64) -> Mesh {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut vertices = Vec::with_capacity(n);
    for i in 0..n {
        vertices.push(Vertex::new(
            i,
            [
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ],
        ));
    }

    // Create faces by triangulating random triplets (simplified, not watertight)
    let face_count = n * 2;
    let mut faces = Vec::with_capacity(face_count);
    for _ in 0..face_count {
        let a = rng.gen_range(0..n);
        let b = rng.gen_range(0..n);
        let c = rng.gen_range(0..n);
        faces.push(Face::new(a, b, c));
    }

    Mesh::new(vertices, faces).unwrap()
}

fn bench_unfold_small(c: &mut Criterion) {
    let mesh = random_mesh(100, 42);
    let config = UnfoldConfig::default();
    c.bench_function("unfold_mds_100_vertices", |b| {
        b.iter(|| unfold_mds(black_box(&mesh), black_box(&config)))
    });
}

fn bench_unfold_medium(c: &mut Criterion) {
    let mesh = random_mesh(500, 123);
    let config = UnfoldConfig::default();
    c.bench_function("unfold_mds_500_vertices", |b| {
        b.iter(|| unfold_mds(black_box(&mesh), black_box(&config)))
    });
}

fn bench_unfold_large(c: &mut Criterion) {
    let mesh = random_mesh(1000, 456);
    let config = UnfoldConfig::default();
    c.bench_function("unfold_mds_1000_vertices", |b| {
        b.iter(|| unfold_mds(black_box(&mesh), black_box(&config)))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10); // reduce time
    targets = bench_unfold_small, bench_unfold_medium, bench_unfold_large
);
criterion_main!(benches);