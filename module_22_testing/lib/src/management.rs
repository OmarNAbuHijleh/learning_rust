use crate::attractions::{MovieTheater, TicketSeller};


#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    pub venue: T,
    manager: Option<String>
}

impl<T: TicketSeller> VenueManagement<T> {
    // This design isn't great, it's dependent on the venue being a Movie theater. We're dependent on a single type rather than a flexible type. How can we make VenueManagement work in a more flexible fashion? --> We'll update it to accept a generic that is bound by the ticketseller trait
    pub fn new(venue: T) -> Self {
        Self {
            // venue: MovieTheater::new(),
            venue: venue, // This is an example of dependency injection. While before, we hard coded the Movie theater object, now we pass it in as an argument
            manager: None
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use pretty_assertions::assert_eq;

    // for dependency injection purposes
    struct DummyVenue {}

    impl TicketSeller for DummyVenue {
        fn sell_ticket(&mut self) {

        }
    }

    #[test]
    fn venue_management_can_hire_manager() {
        let movie_theater = MovieTheater::new(); // Creating our depedency we'll be injecting. This makes VenueManagement Flexible to other types. the problem is that our test still couples movie theaters to the test case. Instead, we can just create a dummy venue because our venuemanagment applies to ticketseller types
        let dummy_venue = DummyVenue{};
        let mut venue_mgmt = VenueManagement::new(movie_theater); // VenueManagement::new(movie_theater);
        venue_mgmt.hire_manager("Mario");
        assert_eq!(venue_mgmt.manager.unwrap(), String::from("Mario"));
    }
}
