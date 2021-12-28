enum PlaceType {
    //kind of place
    room,
    outdoor,
}

struct Place {
    description: String,
    explored: u64,         // number of time this place has been explored/searched
    objects: Vec<Objects>, // objectes that can be found here
    place_type: PlaceType,
}

fn build_empty_place() -> Place {
    Place {
        description: String::from("This place is empty"),
        explored: 0,
    }
}
