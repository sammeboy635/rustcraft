
pub const INSET_VALUE: f32 = 0.03; // Adjust this value as needed

#[macro_export]
macro_rules! atlas_coords {
    ($coord_x:expr, $coord_y:expr) => {
        [
            [
                ($coord_x as f32 + INSET_VALUE) / ATLAS_SIZE[0] as f32,
                ($coord_y as f32 + INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                (($coord_x + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[0] as f32,
                ($coord_y as f32 + INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                (($coord_x + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[0] as f32,
                (($coord_y + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                ($coord_x as f32 + INSET_VALUE) / ATLAS_SIZE[0] as f32,
                (($coord_y + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
        ]
    };
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_atlas() {
        let atlas_size = (16, 16);
		let inset = (0.01, 0.01);
		let texture_ids = [164, 165, 166, 167, 168, 169]; // Specify texture IDs for each face

    }
}
// Example usage
