use crate::utility::Point;

#[derive(Clone, Default)]
pub struct Route {
    pub points: Vec<Point>,
}

impl Route {
    pub fn new() -> Self {
        Route { points: Vec::new() }
    }

    pub fn add_point(&self, point: Point) -> Self {
        let mut new_route = self.clone();
        new_route.points.push(point);
        new_route
    }

    pub fn first(&self) -> Option<&Point> {
        self.points.first()
    }

    pub fn get_after_last(&self) -> Self {
        let mut new_route = self.clone();
        new_route.points.remove(0);
        new_route
    }

    pub fn last(&self) -> Option<&Point> {
        self.points.last()
    }
    
    pub fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }
}
