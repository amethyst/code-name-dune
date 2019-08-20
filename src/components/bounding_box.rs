use amethyst::{
    core::{math::Vector2, Transform},
    ecs::{Component, DenseVecStorage},
};

// use crate::components::Side;

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct BoundingBox {
    pub half_size: Vector2<f32>,
    pub center: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub on_ground: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            half_size: Vector2::new(0., 0.),
            center: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
        }
    }
}

impl BoundingBox {
    pub fn new(width: f32, height: f32) -> Self {
        BoundingBox {
            half_size: Vector2::new(width / 2., height  / 2.),
            center: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Vector2::new(x, y);
    }

    pub fn update_transform_position(&self, transform: &mut Transform) {
        transform.set_translation_x(self.position.x);
        transform.set_translation_y(self.position.y);
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.half_size.y
    }

    pub fn set_top(&mut self, top: f32) {
        self.position.y = top - self.half_size.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - self.half_size.y
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.position.y = bottom + self.half_size.y
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.half_size.x
    }

    pub fn set_left(&mut self, left: f32) {
        self.position.x = left + self.half_size.x
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.half_size.x
    }

    pub fn set_right(&mut self, right: f32) {
        self.position.x = right - self.half_size.x
    }

    pub fn overlapping_x(&self, other: &Self) -> bool {
        self.left() < other.right() && other.left() < self.right()
    }

    pub fn overlapping_y(&self, other: &Self) -> bool {
        self.bottom() < other.top() && other.bottom() < self.top()
    }

    pub fn get_next_right(
        &self,
        two_dim_object: &BoundingBox,
        old_x: f32,
        mut possible_new_x: f32,
        velocity_b_x: f32,
    ) -> (f32, bool) {
        let mut has_changed = false;
        if self.overlapping_y(two_dim_object)
            && old_x <= two_dim_object.left()
            && possible_new_x >= two_dim_object.left() + velocity_b_x
        {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_x = two_dim_object.left()
                + velocity_b_x * (two_dim_object.left() - old_x)
                    / ((possible_new_x - old_x) - velocity_b_x);
            has_changed = true;
        }
        (possible_new_x, has_changed)
    }

    pub fn get_next_left(
        &self,
        two_dim_object: &BoundingBox,
        old_x: f32,
        mut possible_new_x: f32,
        velocity_b_x: f32,
    ) -> (f32, bool) {
        let mut has_changed = false;
        if self.overlapping_y(two_dim_object)
            && old_x >= two_dim_object.right()
            && possible_new_x <= two_dim_object.right() + velocity_b_x
        {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_x = two_dim_object.right()
                + velocity_b_x * (old_x - two_dim_object.right())
                    / ((old_x - possible_new_x) + velocity_b_x);
            has_changed = true;
        }
        (possible_new_x, has_changed)
    }

    pub fn get_next_top(
        &self,
        two_dim_object: &BoundingBox,
        old_y: f32,
        mut possible_new_y: f32,
    ) -> f32 {
        if self.overlapping_x(two_dim_object)
            && old_y <= two_dim_object.bottom()
            && possible_new_y >= two_dim_object.bottom()
        {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_y = two_dim_object.bottom();
        }
        possible_new_y
    }

    pub fn get_next_bottom(
        &self,
        two_dim_object: &BoundingBox,
        old_y: f32,
        mut possible_new_y: f32,
    ) -> f32 {
        if self.overlapping_x(two_dim_object)
            && old_y >= two_dim_object.top()
            && possible_new_y <= two_dim_object.top()
        {
            // Can't early return here, because we need to consider collision with
            // more than one other object. Don't need to set velocity back to zero here,
            // but could depending on how we want the marine animation to act
            possible_new_y = two_dim_object.top();
        }
        possible_new_y
    }

    pub fn is_overlapping_with(&self, other: &BoundingBox) -> bool {
        if (self.position.x - other.position.x).abs() >= (self.half_size.x + other.half_size.x).abs() {
            false
        } else if (self.position.y - other.position.y).abs() >= (self.half_size.y + other.half_size.y).abs() {
            false
        } else {
            true
        }
    }
}