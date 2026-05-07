use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    let nums: Vec<i64> = (0..200_000).collect();
    let bytes: Vec<u8> = (0..200_000)
        .map(|i| if i % 3 == 0 { 0 } else { (i % 251) as u8 })
        .collect();

    let text_for_normalize = " Hello   World \t Rust Profiling ".repeat(20_000);
    let dedup_data: Vec<u64> = (0..10_000).flat_map(|n| [n, n]).collect();

    let mut acc_i64 = 0_i64;
    let mut acc_usize = 0_usize;
    let mut acc_u64 = 0_u64;
    let mut acc_len = 0_usize;

    for _ in 0..30 {
        acc_i64 += sum_even(&nums);
        acc_usize += leak_buffer(&bytes);
        acc_len += normalize(&text_for_normalize).len();
        acc_u64 += algo::slow_fib(32);
        acc_len += algo::slow_dedup(&dedup_data).len();
    }

    println!(
        "ignore: {} {} {} {}",
        acc_i64, acc_usize, acc_u64, acc_len
    );
}
