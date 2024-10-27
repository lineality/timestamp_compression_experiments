use chrono::{Utc, Duration, TimeZone};

fn generate_terse_timestamp_freshness_proxy_v3(posix_timestamp: u64) -> [u8; 4] {
    // 1. Extract relevant digits
    let digit_2 = ((posix_timestamp / 10) % 10) as u8;
    let digit_3 = ((posix_timestamp / 100) % 10) as u8;
    let digit_4 = ((posix_timestamp / 1000) % 10) as u8;
    let digit_5 = ((posix_timestamp / 10000) % 10) as u8;
    let digit_6 = ((posix_timestamp / 100000) % 10) as u8;
    let digit_7 = ((posix_timestamp / 1000000) % 10) as u8;
    let digit_8 = ((posix_timestamp / 10000000) % 10) as u8;
    let digit_9 = ((posix_timestamp / 100000000) % 10) as u8;
    let digit_10 = ((posix_timestamp / 1000000000) % 10) as u8;

    // 2. Determine 10th digit fragments
    let fragment_1 = digit_10 % 2 != 0;
    let fragment_2 = digit_10 % 3 != 0;
    let fragment_3 = digit_10 != 0 && digit_10 != 4;
    let fragment_4 = is_prime(digit_10);

    // 3. Pack into u8 array
    let packed_timestamp = [
        (digit_2 << 4) | (digit_3 << 1) | (fragment_1 as u8),
        (digit_4 << 4) | (digit_5 << 1) | (fragment_2 as u8),
        (digit_6 << 4) | (digit_7 << 1) | (fragment_3 as u8),
        (digit_8 << 4) | (digit_9 << 1) | (fragment_4 as u8),
    ];

    packed_timestamp
}

fn is_prime(n: u8) -> bool {
    match n {
        2 | 3 | 5 | 7 => true,
        _ => false,
    }
}

fn main() {
    // 1. Generate the original freshness proxy
    let now_timestamp = Utc::now().timestamp() as u64;
    let original_freshness_proxy = generate_terse_timestamp_freshness_proxy_v3(now_timestamp);

    println!("Original  Timestamp: {} ({})", now_timestamp, Utc.timestamp_opt(now_timestamp as i64, 0).unwrap());
    println!("Original Freshness Proxy: {:?}", original_freshness_proxy);

    // 2. Iterate through future timestamps
    let start_time = Utc::now();
    let years_to_check = 1; // Check for collisions over the next 1 year
    let end_time = start_time + Duration::days(365 * years_to_check);

    let mut iteration_time = start_time + Duration::seconds(1); // Start from the next second
    while iteration_time <= end_time {
        let posix_timestamp = iteration_time.timestamp() as u64;
        let terse_timestamp = generate_terse_timestamp_freshness_proxy_v3(posix_timestamp);

        // 3. Compare against the original freshness proxy
        if terse_timestamp == original_freshness_proxy {
            println!("\nCollision detected!");
            println!("Original  Timestamp: {} ({})", now_timestamp, Utc.timestamp_opt(now_timestamp as i64, 0).unwrap());
            println!("Colliding Timestamp: {} ({})", posix_timestamp, iteration_time);
            println!("Original Freshness Proxy:  {:?}", original_freshness_proxy);
            println!("Colliding terse_timestamp: {:?}", terse_timestamp);
        }

        iteration_time = iteration_time + Duration::seconds(1);
    }
}
