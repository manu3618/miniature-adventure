enum PlaceType {
    //kind of place
    Room,
    Outdoor,
}

struct Objects {
    description: String,
    transportable: bool,
}

pub struct Place {
    pub description: String,
    explored: u64,         // number of time this place has been explored/searched
    objects: Vec<Objects>, // objectes that can be found here
    place_type: PlaceType,
}

pub fn build_empty_place() -> Place {
    Place {
        description: String::from("This place is empty"),
        explored: 0,
        objects: Vec::new(),
        place_type: PlaceType::Room,
    }
}
