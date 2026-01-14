use micromath::F32Ext;

struct FStopTimer {
    base_stops: f32,
    knob_offset: f32, // possible 0.5, 0.33, 0.25, 0.12
}

impl FStopTimer {
    // calculates total seconds for a given stop value
    fn stops_to_ms(stops: f32) -> u32 {
        // 0.0 stops = 1.0 second = 1000ms
        // formula: 1000 * 2^stops
        let seconds = 2.0_f32.powf(stops);
        (seconds * 1000.0) as u32
    }

    // calculates burn time
    fn calculate_burn_duration(&self) -> f32 {
        let t_base = Self::stops_to_ms(self.base_stops);
        let t_diff = Self::stops_to_ms(self.base_stops + self.knob_offset);
        t_diff - t_base // The "extra slice" of light
    }

    // countdown in stops
    fn get_current_stops_display(&self, remaining_ms: u32, total_ms: u32) -> f32 {
        let elapsed_secs = (total_ms - remaining_ms) as f32 / 1000.0;
        // Logic to show logarithmic progress
        elapsed_secs.log2()
    }
}
