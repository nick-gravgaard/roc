use crate::helpers::{example_file, run_cmd, run_roc};
use criterion::{BenchmarkGroup, black_box, measurement::{Measurement, WallTime}};
use std::path::Path;

// run without optimization, without input
fn exec_bench_simple(
    file: &Path,
    executable_filename: &str,
    expected_ending: &str,
    bench_group_opt: Option<&mut BenchmarkGroup<WallTime>>,
) {
    exec_bench_w_input(
        file,
        "",
        executable_filename,
        expected_ending,
        true,
        bench_group_opt,
    )
}

fn exec_bench_w_input<T: Measurement>(
    file: &Path,
    stdin_str: &str,
    executable_filename: &str,
    expected_ending: &str,
    run_optimized: bool,
    bench_group_opt: Option<&mut BenchmarkGroup<T>>,
) {
    let flags: &[&str] = if run_optimized { &["--optimize"] } else { &[] };

    let compile_out = run_roc(&[&["build", file.to_str().unwrap()], flags].concat());

    if !compile_out.stderr.is_empty() {
        panic!("{}", compile_out.stderr);
    }

    assert!(
        compile_out.status.success(),
        "build ended with bad status {:?}",
        compile_out
    );

    check_cmd_output(file, stdin_str, executable_filename, expected_ending);

    bench_cmd(file, stdin_str, executable_filename, bench_group_opt);
}

fn check_cmd_output(
    file: &Path,
    stdin_str: &str,
    executable_filename: &str,
    expected_ending: &str,
) {
    let out = run_cmd(
        file.with_file_name(executable_filename).to_str().unwrap(),
        stdin_str,
        &[],
    );

    if !&out.stdout.ends_with(expected_ending) {
        panic!(
            "expected output to end with {:?} but instead got {:#?}",
            expected_ending, out
        );
    }
    assert!(out.status.success());
}

fn bench_cmd<T: Measurement>(
    file: &Path,
    stdin_str: &str,
    executable_filename: &str,
    bench_group_opt: Option<&mut BenchmarkGroup<T>>,
) {
    if let Some(bench_group) = bench_group_opt {
        bench_group.bench_function(&format!("Benchmarking {:?}", executable_filename), |b| {
            b.iter(|| {
                run_cmd(
                    black_box(file.with_file_name(executable_filename).to_str().unwrap()),
                    black_box(stdin_str),
                    &[],
                )
            })
        });
    } else {
        run_cmd(
            black_box(file.with_file_name(executable_filename).to_str().unwrap()),
            black_box(stdin_str),
            &[],
        );
    }
}

pub fn bench_nqueens<T: Measurement>(bench_group_opt: Option<&mut BenchmarkGroup<T>>) {
    exec_bench_w_input(
        &example_file("benchmarks", "NQueens.roc"),
        "11",
        "nqueens",
        "2680\n",
        true,
        bench_group_opt,
    );
}

pub fn bench_cfold(bench_group_opt: Option<&mut BenchmarkGroup<WallTime>>) {
    exec_bench_simple(
        &example_file("benchmarks", "CFold.roc"),
        "cfold",
        "10426 & 10426\n",
        bench_group_opt,
    );
}

pub fn bench_deriv(bench_group_opt: Option<&mut BenchmarkGroup<WallTime>>) {
    exec_bench_simple(
        &example_file("benchmarks", "Deriv.roc"),
        "deriv",
        "1 count: 6\n2 count: 22\n3 count: 90\n4 count: 420\n5 count: 2202\n6 count: 12886\n7 count: 83648\n",
        bench_group_opt,
    );
}

pub fn bench_rbtree_ck(bench_group_opt: Option<&mut BenchmarkGroup<WallTime>>) {
    exec_bench_simple(
        &example_file("benchmarks", "RBTreeCk.roc"),
        "rbtree-ck",
        "560\n",
        bench_group_opt,
    );
}

pub fn bench_rbtree_delete(bench_group_opt: Option<&mut BenchmarkGroup<WallTime>>) {
    exec_bench_simple(
        &example_file("benchmarks", "RBTreeDel.roc"),
        "rbtree-del",
        "420\n",
        bench_group_opt,
    );
}

// TODO quicksort
