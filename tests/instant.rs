use embedded_time::{
    self as time,
    duration::{self, *},
    fraction::Fraction,
    Instant,
};
use test_case::test_case;

#[derive(Debug)]
struct Clock;

impl time::Clock for Clock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000);

    fn try_now(&self) -> Result<Instant<Self>, time::clock::Error> {
        unimplemented!()
    }
}

#[test]
fn duration_since() {
    let diff = Instant::<Clock>::new(5).checked_duration_since(&Instant::<Clock>::new(3));
    assert_eq!(
        diff,
        Some(duration::Generic::new(2_u32, Fraction::new(1, 1_000)))
    );

    let diff = Instant::<Clock>::new(5).checked_duration_since(&Instant::<Clock>::new(6));
    assert_eq!(diff, None);
}

#[test]
fn duration_since_epoch() {
    assert_eq!(
        Instant::<Clock>::new(u32::MAX).duration_since_epoch(),
        duration::Generic::from(Milliseconds(u32::MAX))
    );
}

#[test_case(0, u32::MAX/2 => Some(Instant::<Clock>::new(u32::MAX / 2)) ; "Add the maximum allowed duration")]
#[test_case(0, u32::MAX/2 + 1 => None ; "Overflow due to the duration being too large")]
fn checked_add(base: u32, addition: u32) -> Option<Instant<Clock>> {
    Instant::<Clock>::new(base).checked_add(Milliseconds(addition))
}