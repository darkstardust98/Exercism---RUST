use clock::Clock;

//
// Clock Creation
//

#[test]
fn on_the_hour() {
    assert_eq!(Clock::new().to_string(), "08:00");
}

#[test]
#[ignore]
fn past_the_hour() {
    assert_eq!(Clock::new().to_string(), "11:09");
}

#[test]
#[ignore]
fn midnight_is_zero_hours() {
    assert_eq!(Clock::new().to_string(), "00:00");
}

#[test]
#[ignore]
fn hour_rolls_over() {
    assert_eq!(Clock::new().to_string(), "01:00");
}

#[test]
#[ignore]
fn hour_rolls_over_continuously() {
    assert_eq!(Clock::new().to_string(), "04:00");
}

#[test]
#[ignore]
fn sixty_minutes_is_next_hour() {
    assert_eq!(Clock::new().to_string(), "02:00");
}

#[test]
#[ignore]
fn minutes_roll_over() {
    assert_eq!(Clock::new().to_string(), "02:40");
}

#[test]
#[ignore]
fn minutes_roll_over_continuously() {
    assert_eq!(Clock::new().to_string(), "04:43");
}

#[test]
#[ignore]
fn hours_and_minutes_roll_over() {
    assert_eq!(Clock::new().to_string(), "03:40");
}

#[test]
#[ignore]
fn hours_and_minutes_roll_over_continuously() {
    assert_eq!(Clock::new().to_string(), "11:01");
}

#[test]
#[ignore]
fn hours_and_minutes_roll_over_to_exactly_midnight() {
    assert_eq!(Clock::new().to_string(), "00:00");
}

#[test]
#[ignore]
fn negative_hour() {
    assert_eq!(Clock::new().to_string(), "23:15");
}

#[test]
#[ignore]
fn negative_hour_roll_over() {
    assert_eq!(Clock::new().to_string(), "23:00");
}

#[test]
#[ignore]
fn negative_hour_roll_over_continuously() {
    assert_eq!(Clock::new().to_string(), "05:00");
}

#[test]
#[ignore]
fn negative_minutes() {
    assert_eq!(Clock::new().to_string(), "00:20");
}

#[test]
#[ignore]
fn negative_minutes_roll_over() {
    assert_eq!(Clock::new().to_string(), "22:20");
}

#[test]
#[ignore]
fn negative_minutes_roll_over_continuously() {
    assert_eq!(Clock::new().to_string(), "16:40");
}

#[test]
#[ignore]
fn negative_sixty_minutes_is_prev_hour() {
    assert_eq!(Clock::new().to_string(), "01:00");
}

#[test]
#[ignore]
fn negative_one_twenty_minutes_is_two_prev_hours() {
    assert_eq!(Clock::new().to_string(), "23:00");
}

#[test]
#[ignore]
fn negative_hour_and_minutes_both_roll_over() {
    assert_eq!(Clock::new().to_string(), "20:20");
}

#[test]
#[ignore]
fn negative_hour_and_minutes_both_roll_over_continuously() {
    assert_eq!(Clock::new().to_string(), "22:10");
}

#[test]
#[ignore]
fn zero_hour_and_negative_minutes() {
    assert_eq!(Clock::new().to_string(), "23:38");
}

//
// Clock Math
//

#[test]
#[ignore]
fn add_minutes() {
    let clock = Clock::new().add_minutes(3);
    assert_eq!(clock.to_string(), "10:03");
}

#[test]
#[ignore]
fn add_no_minutes() {
    let clock = Clock::new().add_minutes(0);
    assert_eq!(clock.to_string(), "06:41");
}

#[test]
#[ignore]
fn add_to_next_hour() {
    let clock = Clock::new().add_minutes(40);
    assert_eq!(clock.to_string(), "01:25");
}

#[test]
#[ignore]
fn add_more_than_one_hour() {
    let clock = Clock::new().add_minutes(61);
    assert_eq!(clock.to_string(), "11:01");
}

#[test]
#[ignore]
fn add_more_than_two_hours_with_carry() {
    let clock = Clock::new().add_minutes(160);
    assert_eq!(clock.to_string(), "03:25");
}

#[test]
#[ignore]
fn add_across_midnight() {
    let clock = Clock::new().add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");
}

#[test]
#[ignore]
fn add_more_than_one_day() {
    let clock = Clock::new().add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");
}

#[test]
#[ignore]
fn add_more_than_two_days() {
    let clock = Clock::new().add_minutes(3500);
    assert_eq!(clock.to_string(), "11:21");
}

#[test]
#[ignore]
fn subtract_minutes() {
    let clock = Clock::new().add_minutes(-3);
    assert_eq!(clock.to_string(), "10:00");
}

#[test]
#[ignore]
fn subtract_to_previous_hour() {
    let clock = Clock::new().add_minutes(-30);
    assert_eq!(clock.to_string(), "09:33");
}

#[test]
#[ignore]
fn subtract_more_than_an_hour() {
    let clock = Clock::new().add_minutes(-70);
    assert_eq!(clock.to_string(), "08:53");
}

#[test]
#[ignore]
fn subtract_across_midnight() {
    let clock = Clock::new().add_minutes(-4);
    assert_eq!(clock.to_string(), "23:59");
}

#[test]
#[ignore]
fn subtract_more_than_two_hours() {
    let clock = Clock::new().add_minutes(-160);
    assert_eq!(clock.to_string(), "21:20");
}

#[test]
#[ignore]
fn subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new().add_minutes(-160);
    assert_eq!(clock.to_string(), "03:35");
}

#[test]
#[ignore]
fn subtract_more_than_one_day() {
    let clock = Clock::new().add_minutes(-1500);
    assert_eq!(clock.to_string(), "04:32");
}

#[test]
#[ignore]
fn subtract_more_than_two_days() {
    let clock = Clock::new().add_minutes(-3000);
    assert_eq!(clock.to_string(), "00:20");
}

//
// Test Equality
//

#[test]
#[ignore]
fn compare_clocks_for_equality() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_a_minute_apart() {
    assert_ne!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_an_hour_apart() {
    assert_ne!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_hour_overflow() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_hour_overflow_by_several_days() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_hour() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_hour_that_wraps_multiple_times() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_minutes_overflow() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_minutes_overflow_by_several_days() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_minute() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_minute_that_wraps() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_minute_that_wraps_multiple() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_hours_and_minutes() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_clocks_with_negative_hours_and_minutes_that_wrap() {
    assert_eq!(Clock::new(), Clock::new());
}

#[test]
#[ignore]
fn compare_full_clock_and_zeroed_clock() {
    assert_eq!(Clock::new(), Clock::new());
}
