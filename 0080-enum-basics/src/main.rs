#![allow(dead_code, unused_variables)]

#[derive(PartialEq)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(PartialEq)]
enum HotelRoom {
    Vacant,
    Occupied(Person),
    UnderMaintenance(String)
}

impl HotelRoom {
    fn is_vacant(&self) -> bool {
        *self == HotelRoom::Vacant
    }
}

fn main() {
    let mut hr = HotelRoom::UnderMaintenance("Renovation".to_string());
    match &hr {
        HotelRoom::Vacant => println!("The room is vacant."),
        HotelRoom::Occupied(person) if person.first_name.is_empty() => println!("The room is occupied by an unnamed person."),
        HotelRoom::Occupied(person) => println!("The room is occupied by {}.", person.first_name),
        HotelRoom::UnderMaintenance(reason) => println!("The room is under maintenance due to {}.", reason),
    }
    
    match &hr {
        HotelRoom::Occupied(person) => println!("The room is occupied by {}.", person.first_name),
        _ => {}
    }
    
    if let HotelRoom::Occupied(person) = &hr {
        println!("The room is occupied by {}.", person.first_name);
    }
    
    if let HotelRoom::UnderMaintenance(reason) = &mut hr {
        reason.push('!');
        println!("The room is under maintenance due to {}.", reason);
    }
    
    if let HotelRoom::UnderMaintenance(reason) = hr {
        println!("The room is under maintenance due to {}.", reason);
    }
    
    let p = Person { first_name: "John".to_string(), last_name: "Doe".to_string() };
    {
        let name = p.first_name;
        println!("Person's name: {}", name);
    }
    println!("Person's name is {}", p.last_name);
}
