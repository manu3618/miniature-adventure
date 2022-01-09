use uuid::Uuid;

enum PlaceType {
    /// kind of places
    Inside,
    Outside,
}

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

impl Place {
    pub fn new() -> Place {
        Place {
            id: Uuid::new_v4(),
            description: String::from("This place is empty."),
            explored: 0,
            place_type: PlaceType::Inside,
            exits: Vec::new(),
        }
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
    fn self_reference_place() {
        let place = Place::new();
        let place_id = place.id;

        // place.exits = [place].to_vec(); // XXX
    }
}
