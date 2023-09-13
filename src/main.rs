mod support;

mod calc {
    use std::f64::consts::PI;

    pub fn mm_to_inches(mm: f64) -> f64 {
        mm / 25.4
    }
    
    /// Units are in Inches.
    pub const PITCH_5MM_BELT: f64 = 5.0 / 25.4;

    /// Units are in Inches.
    pub const PITCH_3MM_BELT: f64 = 3.0 / 25.4;

    pub fn get_dp(teeth: i32) -> f64 {
        PITCH_5MM_BELT / PI * <i32 as Into<f64>>::into(teeth)
    }

    pub fn center_to_center(dp1: f64, dp2: f64, length: f64) -> f64 {
        let generic = PI * 2.0 * (dp1 + dp2) - length * 4.0 * PITCH_5MM_BELT;

         (-generic + f64::sqrt(f64::powi(generic, 2) - 32.0 * f64::powi(dp1 + dp2, 2))) / 16.0
    }
}


use std::f64::consts::PI;

fn main() {
    let system = support::init(file!());

    let mut pulley_one_teeth: i32 = 0;
    let mut pulley_two_teeth: i32 = 0;
    let mut pulley_desired_space: f32 = 0.0;

    system.main_loop(move |_, ui| {
        ui.window("ContraCalc")
            .size([800.0, 450.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Input your desired pulley tooth counts, and desired spacing in inches.\nBelts are assumed to be 5mm.\nReturned value is in inches.");
                ui.input_int("Pulley #1 Tooth Count", &mut pulley_one_teeth)
                    .build();
                ui.input_int("Pulley #2 Tooth Count", &mut pulley_two_teeth)
                    .build();
                ui.input_float("Desired Pulley Spacing", &mut pulley_desired_space)
                    .build();
                let pulley_desired_f64 = <f32 as Into<f64>>::into(pulley_desired_space);

                let dp1 = calc::get_dp(pulley_one_teeth);
                let dp2 = calc::get_dp(pulley_two_teeth);
                let pulley_desired_f64 = pulley_desired_f64 * 2.0 + PI / 2.0 * (dp1 + dp2) + f64::powi(dp1 + dp2, 2) / 4.0 / pulley_desired_f64;
                ui.text(&format!("Closest spacing less than or equal to desired, with exactly toothed belt: {}",
                    calc::center_to_center(
                        dp1,
                        dp2,
                        f64::floor(pulley_desired_f64 / calc::PITCH_5MM_BELT)
                    )
                ));
            });
    })
}
