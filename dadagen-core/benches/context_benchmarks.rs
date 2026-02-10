//! Performance benchmarks for dadagen-core Context operations
//!
//! Run with: cargo bench --bench context_benchmarks

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use dadagen_core::{Context, ContextPool};

/// Benchmark basic context creation
fn bench_context_creation(c: &mut Criterion) {
    c.bench_function("context_new", |b| {
        b.iter(|| {
            let context = black_box(Context::new());
            drop(context);
        });
    });

    c.bench_function("context_with_seed", |b| {
        b.iter(|| {
            let context = black_box(Context::with_seed(12345));
            drop(context);
        });
    });
}

/// Benchmark field state operations
fn bench_field_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("field_operations");

    // Benchmark insert
    group.bench_function("insert_string", |b| {
        let context = Context::new();
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            context
                .insert_field_state(format!("field_{}", counter), "value".to_string())
                .unwrap();
        });
    });

    // Benchmark get (existing field)
    group.bench_function("get_existing", |b| {
        let context = Context::new();
        context
            .insert_field_state("test".to_string(), "value".to_string())
            .unwrap();
        b.iter(|| {
            let _value: Option<String> = black_box(context.get_field_state("test").unwrap());
        });
    });

    // Benchmark get (missing field)
    group.bench_function("get_missing", |b| {
        let context = Context::new();
        b.iter(|| {
            let _value: Option<String> = black_box(context.get_field_state("missing").unwrap());
        });
    });

    // Benchmark type mismatch
    group.bench_function("get_type_mismatch", |b| {
        let context = Context::new();
        context
            .insert_field_state("test".to_string(), 42i64)
            .unwrap();
        b.iter(|| {
            let _value: Option<String> = black_box(context.get_field_state("test").unwrap());
        });
    });

    group.finish();
}

/// Benchmark iteration operations
fn bench_iteration_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration_operations");

    group.bench_function("increment", |b| {
        let context = Context::new();
        b.iter(|| {
            black_box(context.increment_iteration().unwrap());
        });
    });

    group.bench_function("current", |b| {
        let context = Context::new();
        b.iter(|| {
            black_box(context.current_iteration().unwrap());
        });
    });

    group.bench_function("reset", |b| {
        let context = Context::new();
        // Increment first so reset does something
        context.increment_iteration().unwrap();
        b.iter(|| {
            black_box(context.reset_iteration().unwrap());
            context.increment_iteration().unwrap();
        });
    });

    group.finish();
}

/// Benchmark context cloning
fn bench_context_cloning(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_cloning");

    // Benchmark shallow clone (cheap - just Arc clones)
    group.bench_function("shallow_clone", |b| {
        let context = Context::new();
        context
            .insert_field_state("field1".to_string(), "value1".to_string())
            .unwrap();
        context
            .insert_field_state("field2".to_string(), 42i64)
            .unwrap();
        b.iter(|| {
            let cloned = black_box(context.clone());
            drop(cloned);
        });
    });

    // Benchmark deep clone (expensive - full copy)
    group.bench_function("deep_clone", |b| {
        let context = Context::new();
        context
            .insert_field_state("field1".to_string(), "value1".to_string())
            .unwrap();
        context
            .insert_field_state("field2".to_string(), 42i64)
            .unwrap();
        b.iter(|| {
            let cloned = black_box(context.deep_clone().unwrap());
            drop(cloned);
        });
    });

    // Benchmark snapshot creation (serialization)
    group.bench_function("create_snapshot", |b| {
        let context = Context::new();
        context
            .insert_field_state("field1".to_string(), "value1".to_string())
            .unwrap();
        context
            .insert_field_state("field2".to_string(), 42i64)
            .unwrap();
        context
            .register_field_dependency(
                "field3".to_string(),
                vec!["field1".to_string(), "field2".to_string()],
            )
            .unwrap();
        b.iter(|| {
            let snapshot = black_box(context.create_snapshot().unwrap());
            drop(snapshot);
        });
    });

    group.finish();
}

/// Benchmark context pool operations
fn bench_context_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_pool");

    // Benchmark pool acquisition (new context)
    group.bench_function("acquire_new", |b| {
        b.iter(|| {
            let pool = ContextPool::new(100);
            let context = black_box(pool.acquire().unwrap());
            drop(context);
        });
    });

    // Benchmark pool acquisition (reuse)
    group.bench_function("acquire_reuse", |b| {
        let pool = ContextPool::new(1);
        let context = pool.acquire().unwrap();
        pool.release(context).unwrap();

        b.iter(|| {
            let context = black_box(pool.acquire().unwrap());
            pool.release(context).unwrap();
        });
    });

    // Benchmark pool release
    group.bench_function("release", |b| {
        let pool = ContextPool::new(100);
        b.iter(|| {
            let context = pool.acquire().unwrap();
            black_box(pool.release(context).unwrap());
        });
    });

    group.finish();
}

/// Benchmark with varying workloads
fn bench_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");

    // Benchmark with different numbers of fields
    for field_count in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_fields", field_count)),
            field_count,
            |b, &field_count| {
                b.iter(|| {
                    let context = Context::new();
                    for i in 0..field_count {
                        context
                            .insert_field_state(format!("field_{}", i), format!("value_{}", i))
                            .unwrap();
                    }
                    // Access all fields
                    for i in 0..field_count {
                        let _: Option<String> =
                            context.get_field_state(&format!("field_{}", i)).unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark dependency tracking
fn bench_dependency_tracking(c: &mut Criterion) {
    let mut group = c.benchmark_group("dependency_tracking");

    group.bench_function("register_dependency", |b| {
        let context = Context::new();
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            context
                .register_field_dependency(
                    format!("field_{}", counter),
                    vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()],
                )
                .unwrap();
        });
    });

    group.bench_function("get_dependencies", |b| {
        let context = Context::new();
        context
            .register_field_dependency(
                "field".to_string(),
                vec!["dep1".to_string(), "dep2".to_string(), "dep3".to_string()],
            )
            .unwrap();
        b.iter(|| {
            let deps = black_box(context.get_field_dependencies("field").unwrap());
            drop(deps);
        });
    });

    group.bench_function("all_dependencies", |b| {
        let context = Context::new();
        for i in 0..10 {
            context
                .register_field_dependency(
                    format!("field_{}", i),
                    vec![format!("dep_{}_1", i), format!("dep_{}_2", i)],
                )
                .unwrap();
        }
        b.iter(|| {
            let all = black_box(context.all_dependencies().unwrap());
            drop(all);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_context_creation,
    bench_field_operations,
    bench_iteration_operations,
    bench_context_cloning,
    bench_context_pool,
    bench_scalability,
    bench_dependency_tracking,
);
criterion_main!(benches);
