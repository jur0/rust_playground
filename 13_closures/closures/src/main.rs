use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    cost: u32,
}

#[derive(Clone, Copy, Debug)]
enum StatKind {
    Population,
    Cost,
}

impl City {
    fn get_statistic(&self, stat: StatKind) -> u32 {
        match stat {
            StatKind::Population => self.population,
            StatKind::Cost => self.cost,
        }
    }
}

fn sort_cities(cities: &mut Vec<City>, stat: StatKind) {
    // Closure captures stat variable which is copyable, so it's copied into
    // the closure.
    cities.sort_by_key(|city| city.get_statistic(stat));
}

// Variable cities takes ownership and is mutable (can be assigned to). The
// move keyword in closures is used for moving (instead of borrowing reference)
// the variables into the closures.
fn sort_cities_thread(mut cities: Vec<City>, stat: StatKind) -> Vec<City> {
    let key_fn = move |city: &City| -> u32 { city.get_statistic(stat) };

    // the new thread created by thread::spawn can’t be expected to finish its
    // work before cities and stat are destroyed at the end of the function.
    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    }).join().unwrap()
}

fn main() {
    let mut v = vec![
        City { name: String::from("Prague"), population: 1_200_000, cost: 1500 },
        City { name: String::from("London"), population: 10_000_000, cost: 2500 },
        City { name: String::from("Las Palmas"), population: 700_000, cost: 1300 },
        City { name: String::from("Piestany"), population: 30_000, cost: 800 }
    ];
    println!("v = {:?}", v);

    sort_cities(&mut v, StatKind::Population);
    println!("v (population) = {:?}", v);

    v = sort_cities_thread(v, StatKind::Cost);
    println!("v (cost) = {:?}", v);
}
