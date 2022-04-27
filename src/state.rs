use std::sync::Mutex;
// use super::models::Course;

pub struct AppState {
    pub health_check_response: String, // immutable
    pub visit_count: Mutex<u32>, // mutable
    // pub courses: Mutex<Vec<Course>>, // add a vector full of my courses to persist in AppState
}