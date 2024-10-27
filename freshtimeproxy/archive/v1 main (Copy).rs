use chrono::{Utc, Duration, TimeZone};


/*
            println!("\nCollision detected!");
            println!("Original Timestamp: {} ({})", now_timestamp, Utc.timestamp(now_timestamp as i64, 0));
            println!("Original Freshness Proxy: {}", original_freshness_proxy);
            println!("Colliding Timestamp: {} ({})", posix_timestamp, iteration_time);
            println!("Colliding terse_timestamp: {}", terse_timestamp);
*/
fn main() {
    // 1. Generate the original freshness proxy
    let now_timestamp = Utc::now().timestamp() as u64;
    let original_freshness_proxy = generate_terse_timestamp_freshness_proxy_v2(now_timestamp);

    println!("Original  Timestamp: {} ({})", now_timestamp, Utc.timestamp_opt(now_timestamp as i64, 0).unwrap());
    println!("Original Freshness Proxy: {}", original_freshness_proxy);

    // 2. Iterate through future timestamps
    let start_time = Utc::now();
    let years_to_check = 1; // Check for collisions over the next 10 years
    let end_time = start_time + Duration::days(365 * years_to_check); 

    let mut iteration_time = start_time + Duration::seconds(1); // Start from the next second
    while iteration_time <= end_time {
        let posix_timestamp = iteration_time.timestamp() as u64;
        let terse_timestamp = generate_terse_timestamp_freshness_proxy_v2(posix_timestamp);

        // 3. Compare against the original freshness proxy
        if terse_timestamp == original_freshness_proxy {
            println!("\nCollision detected!");
            println!("Original  Timestamp: {} ({})", now_timestamp, Utc.timestamp_opt(now_timestamp as i64, 0).unwrap());
            println!("Colliding Timestamp: {} ({})", posix_timestamp, iteration_time);
            println!("Original Freshness Proxy:  {}", original_freshness_proxy);
            println!("Colliding terse_timestamp: {}", terse_timestamp);
        }

        iteration_time = iteration_time + Duration::seconds(1);
    }
}


/// Struct for TerseTimestampFreshness
/// u8 = 256 values
/// u16 -> 256 * 256 == 65536 values
///
/// this cannot hold five full digits
/// but it can hold four full digits
/// and an implicit 0 or 1 fifth
/// (or up to 5+9999) so six values in the top digit (but not 7)
///
/// If timestamp is the same, most likely sent in the last minute.
/// If timestamp is not the same, most like not sent in the last minute.
///
/// note: UDP on average takes less that 1 sec to arrive, so there is a
/// sub-second edge case just before each minute marker
/// if the posix-timestamp is sub-second, the timestamp could be rounded up
///
/// e.g. a timestamp of ~2:59:9999 would arive at 3:00:000
///
/// maybe signal send could wait for the next minute if within 2-5 sec...
///
/// This use might avoid 'extraction' and so stay optimal.
///
/// There will be rare 'collision' edge cases where a future terse-freshness-timestamp
/// may collide with an older one and create a false-positive,
/// this would be useful for one minute only and at most send one useless redudant real.
///
/// note: in this version it is assumed that there are python datetime like
/// utilities for getting day of week and month from a posix-epoch timestamp...
/// the input to this or a wrapper function should be the raw timestamp...
///
/// There may be a more direct equivilant way
/// (in some ways perhaps better, if not in othe ways)
/// to use the u64 number directly and not convert into an intermediate cultural callendar:
///
/// This might also be used to 'wait' for the send-file to wait for the next 1.5 min
/// if the seconds are 97-99, wait until n00 to send (giving a 10 sec granularity
/// which should be good for UDP (and useless for spam attacks: 
/// something like a 10 sec window every few years))
///
/// deca-second digit:
/// 0.165 min  = 10 sec
/// 1.5 min    = 90 sec
/// 1.56 min   = 99 sec
///
/// hour-third digit
/// 0.287 hours= 1000 sec
/// 2.5 hours  = 9000 sec
/// 2.78 hours = 9999 sec
///
/// day digit
/// 1.157 days = 100000 seconds
/// 10.4 days  = 900000
/// 11.57 days = 999999 seconds
///
/// half-month digit
/// 0.38 months= 1000000 sec
/// 3.42 months= 9000000 sec
/// 3.8 months = 9999999 sec
///
/// fn is_month_digit_even(num: u64) -> bool {
/// let fourth_digit = (num / 1000000) % 10;
/// fourth_digit % 2 == 0
/// }
/// or
/// 2. Determine odd month
/// let month_digit = ((posix_timestamp / 1000000) % 10) as u16; // 6th digit
/// let odd_month = month_digit % 2 != 0;
///
/// year-third digit
/// 0.3 years  = 10000000 sec
/// 2.85 years = 90000000 sec
/// 3.17 years = 99999999 sec
/// 
/// e.g.
/// 1640995200
/// 2147483647
///   10000000
/// 1000000000
///
/// v2
/// alternate change 
///  || V | |
///  1000000000
///    |*| | |
///    10000000
///      100000
///        1000
///          10
///
///  instead of only using 0-1 to represent 
///  rember the top u16 digit can hold up to 6 values (with the rest 0-9)
///  so, with one to spare, we could count how many even numbers there are
///  in those un-specified five columns
fn generate_terse_timestamp_freshness_proxy_v2(posix_timestamp: u64) -> u16 {
    // 1. Extract relevant digits
    let year_digit = ((posix_timestamp / 100000) % 10) as u16; // 7th digit from the right
    let month_digit = ((posix_timestamp / 10000) % 10) as u16; // 6th digit
    let day_digit =    ((posix_timestamp / 1000) % 10) as u16;    // 5th digit
    let hour_digit =    ((posix_timestamp / 100) % 10) as u16;     // 4th digit
    let ten_sec_digit =  ((posix_timestamp / 10) % 10) as u16;      // 2nd digit

    let tenth_vague_digit = ((posix_timestamp / 10000000) % 10) as u16; // 7th digit from the right
    let ninth_vague_digit = ((posix_timestamp /  1000000) % 10) as u16; // 6th digit
    // let sixth_vague_digit = ((posix_timestamp / 100000) % 10) as u16;    // 5th digit
    // let third_vague_digit = ((posix_timestamp / 100) % 10) as u16;      // 2nd digit
    
    
    // 2. Determine oddnesses
    let odd_month = month_digit % 2 != 0;
    let odd_tenth = ninth_vague_digit % 2 != 0;
    let odd_ninth = tenth_vague_digit % 2 != 0;
    let odd_sixth = ninth_vague_digit % 3 != 0;
    let odd_third = tenth_vague_digit % 3 != 0;

    // 3. combine all the vague areas and months
    // let odd_sum = odd_month + odd_tenth + odd_ninth + odd_sixth + odd_third;
    let odd_sum = (odd_month as u16)
    + odd_tenth as u16 
    + odd_ninth as u16 
    + odd_sixth as u16 
    + odd_third as   u16;
    
    // 4. Pack into u16
    let packed_timestamp = (odd_sum as u16 * 10000)
                            + (year_digit * 1000)
                            + (day_digit * 100)
                            + (hour_digit * 10)
                            + ten_sec_digit;

    packed_timestamp
}





