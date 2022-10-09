use crate::contexts::charts::point::Point;

/// Represents a data domain -> view coordinate mapping for graph elements
#[derive(Clone)]
pub struct ViewBox {
    pub data_min: Point,
    pub data_max: Point,
    pub view_width: f32,
    pub view_height: f32,
}

impl ViewBox {
    /// Map a point in the data domain, to a point in the view coordinate space
    pub(crate) fn data_point_to_view_box_point(&self, data_point: &Point) -> Point {
        let scale_x = self.view_width / (self.data_max.x - self.data_min.x);
        let scale_y = self.view_height / (self.data_max.y - self.data_min.y);

        Point {
            x: (data_point.x - self.data_min.x) * scale_x,
            y: self.view_height - (data_point.y - self.data_min.y) * scale_y,
        }
    }
}
