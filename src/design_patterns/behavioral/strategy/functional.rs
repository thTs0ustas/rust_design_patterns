pub mod functional {

    pub type RouteStrategy = fn(from: &str, to: &str);

    pub fn walking_strategy(from: &str, to: &str) {
        println!("Walking route from {} to {}: 4 km, 30 min", from, to);
    }

    pub fn public_transport_strategy(from: &str, to: &str) {
        println!(
            "Public transport route from {} to {}: 3 km, 5 min",
            from, to
        );
    }
    pub struct FunctionalNavigator {
        route_strategy: RouteStrategy,
    }

    impl FunctionalNavigator {
        pub fn new(route_strategy: RouteStrategy) -> Self {
            Self { route_strategy }
        }
        pub fn route(&self, from: &str, to: &str) {
            (self.route_strategy)(from, to)
        }
    }
}
