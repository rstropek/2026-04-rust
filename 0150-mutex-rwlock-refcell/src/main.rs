use std::{cell::RefCell, rc::Rc, sync::{Mutex, RwLock}};

struct MyPreciousRing {
    engraving: String,
}

impl MyPreciousRing {
    fn forge() -> Self {
        Self {
            engraving: "One Ring to rule them all, One Ring to find them, One Ring to bring them all and in the darkness bind them".to_string(),
        }
    }
}

impl Drop for MyPreciousRing {
    fn drop(&mut self) {
        println!(
            r"
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣶⣶⣶⣄⠀⢠⣄⡀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⡿⠛⢻⣿⣿⣿⠀⢀⣿⣿⣦⡀⠀⠀
        ⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⠋⠉⠁⠀⣸⣿⣿⡏⠀⢸⣿⣿⣿⣷⡄⠀
        ⠀⠀⠀⠀⢀⣾⣿⣿⠋⠁⠉⠀⣰⣶⣾⣿⡿⠟⠀⢠⣿⣿⣿⣿⣿⣿⡄
        ⠀⠀⠀⣴⣿⣿⠟⠛⠀⠀⣿⣿⣿⡿⠛⠉⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⡇
        ⠀⢀⣾⣿⣿⠿⠀⠀⣶⣾⣿⡿⠋⠀⠀⠀⠀⣰⣿⣿⡟⠉⢻⣿⣿⣿⠇
        ⠀⣾⣿⡏⠀⢀⣀⣴⣿⡿⠋⠀⠀⠀⠀⣠⣾⣿⣿⠋⠁⠀⢀⣿⣿⡟⠀
        ⢸⣿⣿⣧⣀⣼⣿⣿⡟⠁⠀⠀⠀⣠⣾⣿⣿⠛⠛⠀⠀⣾⣿⣿⡟⠀⠀
        ⠸⣿⣿⣿⣿⣿⡿⠏⠀⠀⢀⣠⣾⣿⡿⠿⠿⠀⢠⣤⣾⣿⣿⠟⠀⠀⠀
        ⠀⠈⠉⠉⠁⠀⢀⣀⣤⣾⣿⣿⠿⠿⠃⠀⣀⣠⣾⣿⣿⡿⠃⠀⠀⠀⠀
        ⠀⠳⣶⣶⣶⣿⣿⣿⣿⣿⣿⣏⠀⢀⣀⣠⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀
        ⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣾⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠙⠻⢿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"
        );
    }
}

static BUFFER: RwLock<Vec<u8>> = RwLock::new(Vec::new());

fn main() {
    {
        let mut saurons_ring = MyPreciousRing::forge();
        println!(
            "Sauron's ring says: {}, address is {:p}",
            saurons_ring.engraving, &saurons_ring
        );
        let saurons_mutable_borrow = &mut saurons_ring;
        saurons_mutable_borrow.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk, agh burzum-ishi krimpatul".to_string();

        // A lot of things happen...

        let gollums_ring = saurons_ring; // Move = Transfer of ownership
        println!(
            "Gollum's ring says: {}, address is {:p}",
            gollums_ring.engraving, &gollums_ring
        );

        let bilbos_ring = gollums_ring; // Move again
        println!("Bilbo's ring says: {}", bilbos_ring.engraving);
    }

    {
        // Now we bring peace to middle earth
        // Ownership/borrowing -> ref counting
        let mut ring = MyPreciousRing::forge();
        ring.engraving = "Ash nazg...".to_string();
        
        let saurons_ring = Rc::new(Mutex::new(ring)); // Takes ownership of the ring
        println!("Address of rc {:p}, address of ring {:p}, count {}", &saurons_ring, Rc::as_ptr(&saurons_ring), Rc::strong_count(&saurons_ring));
        
        {
            let mut borrowed_ring = saurons_ring.lock().unwrap(); // Borrow the ring mutably
            borrowed_ring.engraving = "Ash nazg durbatulûk...".to_string();
            // borrowed_ring goes out of scope here -> drop MutexGuard -> releases the lock
        }
        
        let gollums_ring = Rc::clone(&saurons_ring);
        println!("Address of rc {:p}, address of ring {:p}, count {}", &gollums_ring, Rc::as_ptr(&gollums_ring), Rc::strong_count(&gollums_ring));
    }

    {
        // Now we bring peace to middle earth
        // Ownership/borrowing -> ref counting
        let mut ring = MyPreciousRing::forge();
        ring.engraving = "Ash nazg...".to_string();
        
        // Note: In a multi-threaded context, we would need Arc instead of Rc
        
        let saurons_ring = Rc::new(RwLock::new(ring)); // Takes ownership of the ring
        println!("Address of rc {:p}, address of ring {:p}, count {}", &saurons_ring, Rc::as_ptr(&saurons_ring), Rc::strong_count(&saurons_ring));
        
        let mut borrowed_ring = saurons_ring.write().unwrap(); // Borrow the ring mutably
        borrowed_ring.engraving = "Ash nazg durbatulûk...".to_string();
        drop(borrowed_ring); // drop RwLockWriteGuard -> releases the lock
        
        let gollums_ring = Rc::clone(&saurons_ring);
        println!("Address of rc {:p}, address of ring {:p}, count {}", &gollums_ring, Rc::as_ptr(&gollums_ring), Rc::strong_count(&gollums_ring));
    }

    {
        // Now we bring peace to middle earth
        // Ownership/borrowing -> ref counting
        let mut ring = MyPreciousRing::forge();
        ring.engraving = "Ash nazg...".to_string();
        
        // "Inner mutability" pattern: RefCell allows for mutable borrows checked at runtime
        // RefCell = borrow checking at runtime, not thread safe, only for single-threaded contexts
        
        let saurons_ring = Rc::new(RefCell::new(ring)); // Takes ownership of the ring
        println!("Address of rc {:p}, address of ring {:p}, count {}", &saurons_ring, Rc::as_ptr(&saurons_ring), Rc::strong_count(&saurons_ring));
        
        let mut borrowed_ring = saurons_ring.borrow_mut(); // Borrow the ring mutably
        borrowed_ring.engraving = "Ash nazg durbatulûk...".to_string();
        drop(borrowed_ring); // drop RefMut -> releases the lock
        
        let gollums_ring = Rc::clone(&saurons_ring);
        println!("Address of rc {:p}, address of ring {:p}, count {}", &gollums_ring, Rc::as_ptr(&gollums_ring), Rc::strong_count(&gollums_ring));
    }

    BUFFER.write().unwrap().push(42);
}
