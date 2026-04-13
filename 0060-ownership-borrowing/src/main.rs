mod wall_clock;

use wall_clock::WallClock;

impl WallClock {
    fn reset(&mut self) {
        self.hours = 0;
        self.minutes = 0;
    }
}

fn main() {
    let mut clock = WallClock::new(12, 30);
    clock.add_minutes(60);
    println!(
        "Hours: {}, Minutes: {}",
        clock.get_hours(),
        clock.get_minutes()
    );

    clock.reset();
    dbg!(clock);
    //println!("Clock: {clock:?}");

    let mut clock = Box::new(WallClock::new(10, 45));
    clock.add_minutes(60);
    //WallClock::add_minutes(&mut clock, 30);
    println!(
        "Hours: {}, Minutes: {}",
        clock.get_hours(),
        clock.get_minutes()
    );

    let clock = construct_clock();
    println!(
        "Hours: {}, Minutes: {}",
        clock.get_hours(),
        clock.get_minutes()
    );
    let mut clock2 = clock; // Transfer of ownership
    let clock3 = clock2.clone(); // No Transfer of ownership, cloning instead
    println!(
        "Hours: {}, Minutes: {}",
        clock2.get_hours(),
        clock2.get_minutes()
    );

    print(&clock2);
    print(&clock2);
    let cb1 = &clock2;
    let cb2 = &clock2;
    println!("Hours: {}, Minutes: {}", cb1.get_hours(), cb1.get_minutes());
    println!("Hours: {}, Minutes: {}", cb2.get_hours(), cb2.get_minutes());

    {
        let mb1 = &mut clock2;
        mb1.add_minutes(10);
        mb1.add_minutes(10);
    }
    let mb2 = &mut clock2;
    mb2.add_minutes(10);
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
}

fn construct_clock() -> WallClock {
    let result = WallClock::new(9, 15);
    result
}

fn print(clock: &WallClock) {
    // Read-only borrow
    println!(
        "Hours: {}, Minutes: {}",
        clock.get_hours(),
        clock.get_minutes()
    );
}
