#![allow(dead_code)]

use rand::prelude::*;

struct ConsultingWork {
    hours: f32,
    rate: f32,
}

fn create_billable(hours: f32, rate: f32) -> impl Billable {
    ConsultingWork { hours, rate }
}

fn create_billable_2(product: Option<String>, val1: f32, val2: f32) -> Box<dyn Billable> {
    if product.is_some() {
        Box::new(Material {
            product: product.unwrap(),
            price: val1,
            quantity: val2,
        })
    } else {
        Box::new(ConsultingWork { hours: val1, rate: val2 })
    }
}

fn print_billable(billable: &impl Billable) {
    println!("Total billable amount: {}", billable.get_total());
}

trait Billable {
    fn get_total(&self) -> f32;
    
    fn get_total_in_000(&self) -> f32 {
        self.get_total() / 1000.0
    }
}

impl Billable for ConsultingWork {
    fn get_total(&self) -> f32 {
        self.hours * self.rate
    }
}

struct Material {
    product: String,
    price: f32,
    quantity: f32,
}

impl Billable for Material {
    fn get_total(&self) -> f32 {
        self.price * self.quantity
    }
}

impl Billable for f32 {
    fn get_total(&self) -> f32 {
        *self
    }
}

impl Billable for Option<f32> {
    fn get_total(&self) -> f32 {
        self.unwrap_or(0.0)
    }
}

fn main() {
    let cw = create_billable(5.0, 100.0);
    print_billable(&cw);
    
    let mat = Material {
        product: "Steel".to_string(),
        price: 50.0,
        quantity: 10.0,
    };
    print_billable(&mat);
    
    let dyn_billables = vec![
        create_billable_2(Some("Wood".to_string()), 20.0, 15.0),
        create_billable_2(None, 3.0, 200.0),
        Box::new(150.0),
        Box::new(Some(75.0)),
        Box::new(None),
    ];
    for billable in dyn_billables {
        println!("Total billable amount (dynamic): {}", billable.get_total_in_000());
    }
    
}
