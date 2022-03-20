use std::collections::HashMap;

use uuid::Uuid;

#[derive(Clone)]
enum PlaceType {
    /// kind of places
    Inside,
    Outside,
}

#[derive(Clone)]
pub struct Place {
    /// Place representation
    ///
    /// During the adventure, the user travel throught different places.
    pub id: Uuid,
    pub description: String,
    explored: u64, // number of time this place has been explored/searched
    place_type: PlaceType,
    exits: Vec<Place>, // link to other places
}

lazy_static! {
    static ref ALL_PLACES: HashMap<Uuid, Place> = {
        //let mut m = HashMap::new();
        let m = HashMap::new();
        m
    };
}

impl Place {
    pub fn new() -> Place {
        let place_id = Uuid::new_v4();
        let new_place = Place {
            id: place_id,
            description: String::from("This place is empty."),
            explored: 0,
            place_type: PlaceType::Inside,
            exits: Vec::new(),
        };

        // ALL_PLACES.insert(place_id, &mut new_place);
        return new_place;
    }
    pub fn describe(&self) -> String {
        let mut descr: Vec<&str> = Vec::new();
        match &self.place_type {
            PlaceType::Inside => descr.push("You are in a room."),
            PlaceType::Outside => descr.push("You are outdoor."),
        }
        descr.push(&self.description);

        let mut full_description = String::new();
        for sentence in descr {
            full_description = full_description + " " + sentence;
        }
        return full_description.trim().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_description() {
        let place = Place::new();
        let place = Place {
            description: String::from(""),
            ..place
        };
        assert_eq!(place.describe(), "You are in a room.");
    }
    fn place_description_determinism() {
        let place = Place::new();
        assert_eq!(place.describe(), place.describe());
    }
}
