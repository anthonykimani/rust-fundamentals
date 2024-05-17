mod counties;
mod plants;
use counties::counties_kenya;
use plants::{poisonous_plants, fruits_plants};

fn main() {
    counties_kenya();
    poisonous_plants::poisonous_plants();
    fruits_plants::fruit_plants();
}
