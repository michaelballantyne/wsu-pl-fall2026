mod state_machine;

enum TurnstileEvent {
    Coin(i32),
    Pass(),
    Timeout()
}

fn main() {
    const FARE: i32 = 150; // Fare required in cents
    
    let mut turnstile = state_machine!(self,
        events: TurnstileEvent,
        initial: Locked,

        data accumulated_value: i32 = 0;
        
        state Unlocked {
            TurnstileEvent::Pass() | TurnstileEvent::Timeout() => Locked {
                println!("Locking turnstile")
            }
        }

        state Locked {
            TurnstileEvent::Coin(value) if (value >= (FARE - self.accumulated_value)) => Unlocked {
                self.accumulated_value = 0;
                println!("Unlocking turnstile")
            }

            TurnstileEvent::Coin(value) => Locked {
                println!("Received {} cents. Need {} more.", 
                    value, FARE - (self.accumulated_value + value))
                self.accumulated_value += value
            }
        }
    );
    
    // Test the turnstile
    println!("Initial state: {:?}", turnstile.get_state());
    
    // // Try to pass through when locked
    // println!("\nAttempting to pass through turnstile...");
    // turnstile.process_event(TurnstileEvent::Pass());
    // println!("State after attempted pass: {:?}", turnstile.get_state());
    
    // Insert some coins
    println!("\nInserting 50 cents...");
    turnstile.process_event(TurnstileEvent::Coin(50));
    println!("State after 50 cents: {:?}", turnstile.get_state());
    println!("Accumulated value: {}", turnstile.accumulated_value);
    
    println!("\nInserting 50 cents more...");
    turnstile.process_event(TurnstileEvent::Coin(50));
    println!("State after another 50 cents: {:?}", turnstile.get_state());
    println!("Accumulated value: {}", turnstile.accumulated_value);
    
    println!("\nInserting 60 cents more...");
    turnstile.process_event(TurnstileEvent::Coin(60));
    println!("State after 60 more cents: {:?}", turnstile.get_state());
    println!("Accumulated value: {}", turnstile.accumulated_value);

    // Pass through unlocked turnstile
    println!("\nPassing through turnstile...");
    turnstile.process_event(TurnstileEvent::Pass());
    println!("State after passing: {:?}", turnstile.get_state());
    println!("Accumulated value: {}", turnstile.accumulated_value);
    
}