use std::thread;
use std::time::Duration;

// Small program just to play with enum and match statements.
fn main() {
    let mut light = TrafficLightState::Red;
    for _ in 0..10 {
        let duration = light.get_duration();
        print_traffic_light_state(&light);
        sleep(duration);
        light = light.next();
    }
}

enum TrafficLightState {
    Red,
    RedYellow,
    Green,
    Yellow,
}

impl TrafficLightState {
    fn next(&self) -> Self {
        match self {
            Self::Red => Self::RedYellow,
            Self::RedYellow => Self::Green,
            Self::Green => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }

    fn get_duration(&self) -> u64 {
        match self {
            Self::Red => 3,
            Self::RedYellow => 1,
            Self::Green => 5,
            Self::Yellow => 1,
        }
    }
}

fn sleep(duration: u64) {
    thread::sleep(Duration::from_secs(duration));
}


fn print_traffic_light_state(state: &TrafficLightState) {
    clear_lines(9);
    let (a,b,c) = match state {
        TrafficLightState::Red => ("🔴", "  ", "  "),
        TrafficLightState::RedYellow => ("🔴", "🟡", "  "),
        TrafficLightState::Green => ("  ", "  ", "🟢"),
        TrafficLightState::Yellow => ("  ", "🟡", "  "),
    };
    println!("┌────┐");
    println!("│ {} │", a);
    println!("└────┘");
    println!("┌────┐");
    println!("│ {} │", b);
    println!("└────┘");
    println!("┌────┐");
    println!("│ {} │", c);
    println!("└────┘");
}


fn clear_lines(n: usize) {
    // Move up n lines and clear from there
    print!("\x1B[{}A\x1B[J", n);
}