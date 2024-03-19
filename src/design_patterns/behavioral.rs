pub mod strategy {
    pub mod conceptual;
    pub mod functional;
    use conceptual::conceptual::{ConceptualNavigator, PublicTransportStrategy, WalkingStrategy};
    use functional::functional::{
        public_transport_strategy, walking_strategy, FunctionalNavigator,
    };

    pub fn strategy() {
        let navigator = ConceptualNavigator::new(WalkingStrategy);
        navigator.route("Home", "Club");
        let navigator = ConceptualNavigator::new(PublicTransportStrategy);
        navigator.route("Club", "Work");

        let fn_navigator = FunctionalNavigator::new(walking_strategy);
        fn_navigator.route("Home", "Club");
        let fn_navigator = FunctionalNavigator::new(public_transport_strategy);
        fn_navigator.route("Club", "Work");
    }
}

pub mod chain_of_responsibility {
    mod department;
    mod patient;

    use department::{Cashier, Department, Doctor, Medical, Reception};
    use patient::Patient;

    pub fn chain_of_responsibility() {
        let cashier = Cashier::default();
        let medical = Medical::new(cashier);
        let doctor = Doctor::new(medical);
        let mut reception = Reception::new(doctor);

        let mut patient = patient::Patient {
            name: "John".into(),
            ..Patient::default()
        };

        // Reception handles a patient passing him to the next link in the chain.
        // Reception -> Doctor -> Medical -> Cashier.
        reception.execute(&mut patient);

        println!("\nThe patient has been already handled:\n");

        reception.execute(&mut patient);
    }
}
