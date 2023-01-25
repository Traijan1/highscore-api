use rocket_governor::{Method, Quota, RocketGovernable};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        // Allow 100 Calls per second
        Quota::per_second(Self::nonzero(100u32))
    }
}
