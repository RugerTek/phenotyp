use rand::seq::SliceRandom;

#[derive(Debug)]
#[derive(Clone)]
pub struct Waypoint {
    x: f32,
    y: f32
}
#[derive(Debug)]
pub struct Chromosome {
    id: String,
    label: String,
    start: Waypoint,
    finish: Waypoint,
    waypoints: Vec<Waypoint>
}

impl Chromosome {
    fn calculate_fitness(&self) -> i32 {
        
    }
}

#[derive(Debug)]
struct EnvironmentEngine {
    population: Vec<Chromosome>
}

fn init_population(population_size: &i32) -> EnvironmentEngine {
    let mut waypoints = vec![
        Waypoint { x: 3.0, y: 1.5 },
        Waypoint { x: 8.0, y: 12.2 },
        Waypoint { x: 7.3, y: 1.3 },
        Waypoint { x: 13.0, y: 4.9 },
        Waypoint { x: 4.7, y: 16.7 },
        Waypoint { x: 4.5, y: 18.5 },
        Waypoint { x: 13.6, y: 8.6 },
        Waypoint { x: 2.3, y: 13.5 },
        Waypoint { x: 10.4, y: 8.3 },
        Waypoint { x: 6.8, y: 1.7 },
        Waypoint { x: 16.2, y: 4.3 },
        Waypoint { x: 2.5, y: 9.8 },
    ];
    let mut population = vec![];
    for i in 0..*population_size {
        let slice = waypoints.as_mut_slice();
        let mut rng = rand::thread_rng();

        slice.shuffle(&mut rng);
        let chromosome = Chromosome {
            id: i.to_string(),
            label: String::from("test"),
            start: Waypoint { x: 1.0, y: 3.0 },
            finish: Waypoint { x: 1.0, y: 3.0 },
            waypoints: slice.to_vec()
        };    
        population.push(chromosome);
    }
    
    
    let engine = EnvironmentEngine {
        population
    };
    
    engine
}

fn main() {
    let _epochs = 1000;
    let population_size = 100;
    let env = init_population(&population_size);
    for chromosome in env.population {
        println!("{:?}", chromosome);    
    }
}
