use iter_zip_sort::PrioritySortIterator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn zip_benchmark(cri: &mut Criterion) {
    let mut va = vec![];
    let mut vb = vec![];
    let mut vc = vec![];

    for i in (0..10000000).step_by(2) {
        va.push(i);
    }
    for i in (1000000..4000000).step_by(10) {
        vb.push(i);
    }
    for i in 0..1000000 {
        vc.push(i);
    }

    cri.bench_function("zip mixed", |b| {
        b.iter(|| {
            let zip = PrioritySortIterator::new(vec![&va, &vb, &vc].as_mut_slice());
            black_box(zip.last());
        })
    });
}



fn zip_creatrion(cri: &mut Criterion) {
    let mut va = vec![];
    let mut vb = vec![];
    let mut vc = vec![];

    for i in (0..10000000).step_by(2) {
        va.push(i);
    }
    for i in (1000000..4000000).step_by(10) {
        vb.push(i);
    }
    for i in 0..1000000 {
        vc.push(i);
    }

    cri.bench_function("zip creation", |b| {
        b.iter(|| {
            let zip = PrioritySortIterator::new(vec![&va, &vb, &vc].as_mut_slice());
            black_box(zip);
        })
    });
}

criterion_group!(benches, zip_benchmark,zip_creatrion);
criterion_main!(benches);
