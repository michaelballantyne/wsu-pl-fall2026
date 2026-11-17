#[macro_export]
macro_rules! state_machine {
    (   $self:ident,
        events: $event_type:ty,
        initial: $initial_state:ident,

        $(
            data $field:ident: $field_type:ty = $rhs:expr;
        )*
        
        $(
            state $state:ident {
                $(
                    $event_pattern:pat $(if $guard:expr)? => $next_state:ident {
                        $($action:stmt)*
                    }
                )*
            }
        )*
    ) => {
        {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum State {
                $($state,)*
            }

            struct StateMachine {
                state: State,
                $($field: $field_type,)*
            }

            impl StateMachine {
                fn new() -> Self {
                    Self {
                        state: State::$initial_state,
                        $($field: $rhs,)*
                    }
                }

                fn get_state(&self) -> State {
                    self.state
                }

                fn process_event(&mut $self, event: $event_type) {
                    match $self.state {
                        $(
                            State::$state => {
                                match event {
                                    $(
                                        $event_pattern $(if $guard)? => {
                                            $($action)*
                                            $self.state = State::$next_state;
                                        }
                                    ),*
                                    _ => { panic!("unhandled event")}
                                }
                            }
                        ),*
                    }
                }
            }

            StateMachine::new()
        }
    };
}