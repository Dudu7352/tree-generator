use crate::params::Params;

pub struct TreeConfig {
    pub max_branch_angle: f32
}

impl From<&Params> for TreeConfig{
    fn from(value: &Params) -> Self {
        Self {
            max_branch_angle: value.max_branch_angle
        }
    }
}