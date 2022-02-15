use std::time::Duration;

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    /// Simulate a fern growing for one day.
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }

    pub fn is_furled(&self) -> bool {
        true
    }

    pub fn is_fully_unfurled(&self) -> bool {
        true
    }
}

/// Run a fern simulation for some number of days.
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

pub struct Terrarium;

impl Terrarium {
    pub fn load<'a>(file: &'a str) -> World {
        println!("{}", file);
        World {
            ferns: Vec::new(),
            updated: false,
        }
    }
}

pub struct World {
    ferns: Vec<Fern>,
    updated: bool,
}

impl World {
    pub fn apply_sunlight(&mut self, _d: Duration) {
        self.updated = true;
    }

    pub fn fern(&self, i: usize) -> &Fern {
        match self.ferns.get(i) {
            Some(f) => f,
            None => &Fern {
                size: 1.0,
                growth_rate: 1.0,
            },
        }
    }
}
