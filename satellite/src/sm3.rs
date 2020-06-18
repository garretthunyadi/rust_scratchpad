/*
 The main requirement of the state machine is to make error states unrepresentable.

 the state machine must have no possibility for run time errors.
  -> States need to be separate objects, consumed

  In the initial, and "pretty" version, there is a match on the state and the event and
  the match allows runtime combos that require a panic.  Therefore, we were able to
  "represent error states"
*/

/*
    1. In order to make error states unrepresentatable, we operate directly on the states, which are consumed.


*/

mod power {
    pub type Percent = u8; // would like to have a dependent type here
    struct BatteryState(PowerLevel, PowerLevel);

    #[derive(Clone, Debug, PartialEq)]
    enum PowerLevel {
        Nominal(Percent),
        Low(Percent),
    }
    impl PowerLevel {
        fn from(perc: Percent) -> PowerLevel {
            if perc > 10 {
                PowerLevel::Nominal(perc)
            } else {
                PowerLevel::Low(perc)
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct LowPowerState(PowerLevel, PowerLevel);
    // impl LowPowerState {
    // fn new((Percent,Percent)) -> LowPowerState {
    //     // YUCK! runtime error!
    //     //  instead, let's not allow this to be created from a value, instrad, create on of multiple states, given the values.
    //     LowPowerState
    // }
    // }
    #[derive(Clone, Debug, PartialEq)]
    struct NominalState(PowerLevel, PowerLevel);

    #[derive(Clone, Debug, PartialEq)]
    struct System<S> {
        state: S,
        command_log: Vec<Command>,
    }

    impl<S> System<S> {
        fn emit(&mut self, command: Command) {
            self.command_log.push(command)
        }
    }
    impl System<NominalState> {
        fn from_nominal_values((b1, b2): (&PowerLevel, &PowerLevel)) -> System<NominalState> {
            match (b1, b2) {
                (PowerLevel::Nominal(_), PowerLevel::Nominal(_)) => System {
                    state: NominalState(b1.clone(), b2.clone()),
                    command_log: vec![],
                },
                _ => panic!("Invalid state :("),
            }
        }

        // Cannot perform mission unless in a normal state
        fn perform_mission(&mut self) {
            self.emit(Command::PerformMission)
        }
    }
    impl System<LowPowerState> {
        fn from_low_power_values((b1, b2): (&PowerLevel, &PowerLevel)) -> System<LowPowerState> {
            match (b1, b2) {
                (PowerLevel::Low(_), PowerLevel::Low(_)) => System {
                    state: LowPowerState(b1.clone(), b2.clone()),
                    command_log: vec![],
                },
                (PowerLevel::Nominal(_), PowerLevel::Low(_)) => System {
                    state: LowPowerState(b1.clone(), b2.clone()),
                    command_log: vec![],
                },
                (PowerLevel::Low(_), PowerLevel::Nominal(_)) => System {
                    state: LowPowerState(b1.clone(), b2.clone()),
                    command_log: vec![],
                },
                (PowerLevel::Nominal(_), PowerLevel::Nominal(_)) => panic!("Invalid state :("),
            }
        }

        // when (and only when) in a low power state can a power save command be issues
        fn enter_power_save_mode(&mut self) {
            self.emit(Command::PowerSave)
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum SystemState {
        Nominal(System<NominalState>),
        LowPower(System<LowPowerState>),
    }
    impl SystemState {
        fn step(self, event: Event) -> SystemState {
            match (self.clone(), event) {
                (SystemState::Nominal(_), Event::BatteryState(_, _)) => self,
                _ => self,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum Command {
        PowerSave,
        PerformMission,
    }

    #[derive(Clone, Debug, PartialEq)]
    enum BatteryId {
        Battery1,
        Battery2,
    }

    #[derive(Clone, Debug, PartialEq)]
    enum Event {
        BatteryState(BatteryId, PowerLevel),
    }

    #[test]
    fn test_sm3() {
        let low_power = PowerLevel::from(10);
        let nominal_power = PowerLevel::from(30);
        let mut ok = System::from_nominal_values((&nominal_power, &nominal_power));
        let _low = System::from_low_power_values((&low_power, &low_power));
        let _low = System::from_low_power_values((&nominal_power, &low_power));
        let mut low = System::from_low_power_values((&low_power, &nominal_power));

        ok.perform_mission();
        ok.perform_mission();
        assert_eq!(
            ok.command_log,
            vec![Command::PerformMission, Command::PerformMission]
        );
        low.enter_power_save_mode();
        assert_eq!(low.command_log, vec![Command::PowerSave]);

        // in order to use polymorphism (where necessary) we go back to an enum, but an enum containing
        // a state that must be obtained through a valid transformation.
        let sys_state = SystemState::Nominal(ok);

        // create an event - low power
        let event = Event::BatteryState(BatteryId::Battery1, PowerLevel::from(10));
        let sys_state = sys_state.step(event);
        // let _x = sys_state.step(event); // good, neither event nor sys state can be used
        match sys_state {
            SystemState::LowPower(_) => {}
            SystemState::Nominal(_) => panic!("should be in low power state"),
        }

        // ok.perform_mission(); // good, we can no longer use the var
    }
}
