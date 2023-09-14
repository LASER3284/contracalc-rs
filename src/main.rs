mod support;

mod calc {
    use std::f64::consts::PI;

    /// Dynamically converts from millimeters to inches.
    pub fn mm_to_inches(mm: f64) -> f64 {
        mm / 25.4
    }

    /// Units are in Inches.
    pub const PITCH_5MM_BELT: f64 = 5.0 / 25.4;

    /// Units are in Inches.
    pub const PITCH_3MM_BELT: f64 = 3.0 / 25.4;

    /// Returns the diameter of an n-toothed pulley for a 5mm belt
    pub fn get_diam_5mm(teeth: i32) -> f64 {
        PITCH_5MM_BELT / PI * <i32 as Into<f64>>::into(teeth)
    }

    /// This module contains utilities for contrabelts (aka, twisted belts).
    pub mod contra {
        use super::PI;
        /// Finds the Center-to-Center of two pulleys based on the diameters,
        /// number of teeth of the belt, and pitch of the belt.
        pub fn center_to_center(diam1: f64, diam2: f64, teeth: f64, pitch: f64) -> f64 {
            let generic = PI * 2.0 * (diam1 + diam2) - teeth * 4.0 * pitch;

            (-generic + f64::sqrt(f64::powi(generic, 2) - 32.0 * f64::powi(diam1 + diam2, 2))) / 16.0
        }

        /// Finds the actual tooth count and Center-to-Center that are closest
        /// to the desired length, with the appropriate diameter of each pulley,
        /// as well as desired tooth count, and pitch. Rounds the tooth count to
        /// the nearest multiple of 5.
        pub fn belt_length_actual(diam1: f64, diam2: f64, teeth: f64, pitch: f64) -> (f64, f64) {
            let teeth = teeth.floor();
            let teeth = if teeth % 5.0 >= 2.5 { teeth + (5.0 - (teeth % 5.0)) } else { teeth - (teeth % 5.0) };

            (teeth, center_to_center(diam1, diam2, teeth, pitch))
        }

        /// Finds the Center-to-Center of a set of pulleys given their
        /// diameters, tooth count of the belt, and pitch of the belt. Does not
        /// round the tooth count to the nearest multiple of 5.
        pub fn belt_length_no_round(diam1: f64, diam2: f64, teeth: f64, pitch: f64) -> (f64, f64) {
            let teeth = teeth.floor();
            (teeth, center_to_center(diam1, diam2, teeth, pitch))
        }

        /// Runs the calculations and outputs them to ImGui.
        ///
        /// Note: `ui` is of type `&&mut imgui::Ui`, which is a ref to a mut
        /// ref. This only works, and I would not normally reccomend.
        pub fn output_prompt(ui: &&mut imgui::Ui, diam1: f64, diam2: f64, desired: f64) {
            let (teeth, ctc) = belt_length_actual(
                diam1,
                diam2,
                f64::floor(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
                );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Belts with teeth of multiple of 5");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}in",
                teeth,
                ctc
            ));

            let (teeth, c2c) = belt_length_no_round(
                diam1,
                diam2,
                f64::floor(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
            );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Exactly toothed belt lower");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}",
                teeth,
                c2c
            ));

            let (teeth, c2c) = belt_length_no_round(
                diam1,
                diam2,
                f64::ceil(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
            );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Exactly toothed belt upper");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}",
                teeth,
                c2c
            ));
        }
    }

    /// This module contains utilities for normal/untwisted belts.
    pub mod normal {
        use super::PI;

        /// Calculates the Center-to-Center of two pulleys based on the radii,
        /// number of teeth of the belt, and belt pitch.
        pub fn center_to_center(r1: f64, r2: f64, teeth: f64, pitch: f64) -> f64 {
            f64::sqrt(
                f64::powi((teeth * pitch - PI * (r1 + r2)) / 2.0, 2) - f64::powi(r2 - r1, 2)
            )
        }

        /// Finds the actual tooth count and Center-to-Center that are closest
        /// to the desired length, based on radii, tooth count, and pitch.
        /// Rounds the tooth count to the nearest multiple of 5.
        pub fn belt_length_actual(r1: f64, r2: f64, teeth: f64, pitch: f64) -> (f64, f64) {
            let teeth = teeth.floor();
            let teeth = if teeth % 5.0 >= 2.5 { teeth + (5.0 - (teeth % 5.0)) } else { teeth - (teeth % 5.0) };

            (teeth, center_to_center(r1, r2, teeth, pitch))
        }

        /// Finds the Center-to-Center of two pulleys given their radii, tooth
        /// count of the belt, and pitch of the belt. Does not round tooth count
        /// to nearest multiple of 5.
        pub fn belt_length_no_round(r1: f64, r2: f64, teeth: f64, pitch: f64) -> (f64, f64) {
            let teeth = teeth.floor();
            (teeth, center_to_center(r1, r2, teeth, pitch))
        }

        /// Runs the calculations and outputs them to ImGui.
        ///
        /// Note: `ui` is of type `&&mut imgui::Ui`, which is a ref to a mut
        /// ref. This only works, and I would not normally reccomend.
        pub fn output_prompt(ui: &&mut imgui::Ui, r1: f64, r2: f64, desired: f64) {
            let (teeth, c2c) = belt_length_actual(
                r1,
                r2,
                f64::floor(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
            );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Belts with teeth of multiple of 5");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}\n",
                teeth,
                c2c
            ));

            let (teeth, c2c) = super::normal::belt_length_no_round(
                r1,
                r2,
                f64::floor(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
            );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Exactly toothed belt lower");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}",
                teeth,
                c2c
            ));

            let (teeth, c2c) = super::normal::belt_length_no_round(
                r1,
                r2,
                f64::ceil(desired / super::PITCH_5MM_BELT),
                super::PITCH_5MM_BELT
            );

            ui.text_colored([1.0, 1.0, 0.0, 1.0], "Exactly toothed belt upper");
            ui.text(format!("Number of teeth: {}\nCenter-to-Center: {}",
                teeth,
                c2c
            ));
        }
    }
}


use std::f64::consts::PI;

/// The choices for types of calculations to perform. Determines the UI.
#[derive(Copy, Clone, PartialEq)]
enum Choices {
    NormalBelt,
    ContraBelt,
}

const PROMPT_BELT: &'static str = r#"
Input your desired pulley tooth counts, and desired spacing in inches.
Belts are assumed to be 5mm.
Units for outputs are provided.
"#;

fn main() {
    let system = support::init(file!());

    let mut pulley_one_teeth: i32 = 0;
    let mut pulley_two_teeth: i32 = 0;
    let mut pulley_desired_space: f32 = 0.0;
    let mut choice = Choices::NormalBelt;

    system.main_loop(move |_, ui| {
        ui.window("ContraCalc")
            .size([800.0, 450.0], imgui::Condition::FirstUseEver)
            .build(|| {

                ui.radio_button("Normal Belt", &mut choice, Choices::NormalBelt);
                ui.radio_button("Contra-Belt", &mut choice, Choices::ContraBelt);

                match choice {
                    Choices::ContraBelt => {
                        ui.text(PROMPT_BELT);
                        ui.input_int("Pulley #1 Tooth Count", &mut pulley_one_teeth)
                            .build();
                        ui.input_int("Pulley #2 Tooth Count", &mut pulley_two_teeth)
                            .build();
                        ui.input_float("Desired Pulley Spacing", &mut pulley_desired_space)
                            .build();
                        let pulley_desired_f64 = <f32 as Into<f64>>::into(pulley_desired_space);

                        let diam1 = calc::get_diam_5mm(pulley_one_teeth);
                        let diam2 = calc::get_diam_5mm(pulley_two_teeth);

                        let pulley_desired_f64 =
                            pulley_desired_f64 * 2.0 + PI / 2.0 * (diam1 + diam2)
                            + f64::powi(diam1 + diam2, 2) / 4.0 / pulley_desired_f64;

                        calc::contra::output_prompt(&ui, diam1, diam2, pulley_desired_f64);

                    },
                    Choices::NormalBelt => {
                        ui.text(PROMPT_BELT);
                        ui.input_int("Pulley #1 tooth count", &mut pulley_one_teeth)
                            .build();
                        ui.input_int("Pulley #2 tooth count", &mut pulley_two_teeth)
                            .build();
                        ui.input_float("Desired Pulley Spacing", &mut pulley_desired_space)
                            .build();
                        let pulley_desired_f64 = <f32 as Into<f64>>::into(pulley_desired_space);

                        let r1 = calc::get_diam_5mm(pulley_one_teeth) / 2.0;
                        let r2 = calc::get_diam_5mm(pulley_two_teeth) / 2.0;

                        let pulley_desired_f64 = PI * (r1 + r2) + 2.0 * f64::sqrt(pulley_desired_f64.powi(2) + (r2 - r1).powi(2));

                        calc::normal::output_prompt(&ui, r1, r2, pulley_desired_f64);
                    }
                }
            });
    })
}
