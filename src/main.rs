extern crate jemallocator;
extern crate jemalloc_ctl;

use jemalloc_ctl::{stats, epoch};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;
use std::mem;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    let ep = epoch::mib().unwrap();
    let a = stats::allocated::mib().unwrap();
    let r = stats::resident::mib().unwrap();

    let mut vec_deq = VecDeque::new();
    let mut vec_deq2 = VecDeque::new();

    print_mem_usage_mib("At start".to_string(),&ep, &a, &r);
    print_mem_usage("At start".to_string());

    for _ in 0..32 {
        vec_deq.push_front(rand_string(1048576));
    }

    print_mem_usage_mib("After allocating First VecDeque".to_string(), &ep, &a, &r);
    print_mem_usage("After allocating First VecDeque".to_string());

    mem::swap(&mut vec_deq2, &mut vec_deq);

    // print_mem_usage_mib("After mem swap".to_string(), &ep, &a, &r);
    print_mem_usage("After mem swap".to_string());

    let mut vec = vec_deq2.drain(0..vec_deq2.len()).collect::<Vec<String>>();

    print_mem_usage_mib("After draining VecDeq".to_string(), &ep, &a, &r);
    print_mem_usage("After draining VecDeq".to_string());

    vec_deq2.shrink_to_fit();

    print_mem_usage_mib("After shrink to fit".to_string(), &ep, &a, &r);
    print_mem_usage("After shrink to fit".to_string());

    consume_vec(&mut vec);

    print_mem_usage_mib("After using Vec".to_string(), &ep, &a, &r);
    print_mem_usage("After using Vec".to_string())
}

fn consume_vec(v: &mut Vec<String>) {
    v.clear();
    v.shrink_to_fit();
}

fn rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}

fn print_mem_usage(
    message: String
) {
    epoch::advance().unwrap();

    let allocated = stats::allocated::read().unwrap();
    let resident = stats::resident::read().unwrap();
    println!("REG: {} bytes allocated/{} bytes resident. {}.", allocated, resident, message,);
}

fn print_mem_usage_mib(
    message: String,
    e: &jemalloc_ctl::epoch_mib,
    a: &jemalloc_ctl::stats::allocated_mib,
    r: &jemalloc_ctl::stats::resident_mib,
) {
    e.advance().unwrap();
    let allocated = a.read().unwrap();
    let resident = r.read().unwrap();
    println!("MIB: {} bytes allocated/{} bytes resident. {}.", allocated, resident, message,);
}
