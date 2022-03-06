use mutagen::{Generatable, Mutatable, Updatable, UpdatableRecursively};
use serde::{Deserialize, Serialize};

use crate::{datatype::continuous::*, mutagen_args::*};

#[derive(
    Clone, Copy, Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug,
)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum SFloatNormaliser {
    Sawtooth,
    Triangle,
    Sin,
    SinRepeating,
    TanH,
    Clamp,
    Fractional,
    Random,
}

impl SFloatNormaliser {
    pub fn normalise(self, value: f32) -> SNFloat {
        use SFloatNormaliser::*;

        match self {
            Sawtooth => SNFloat::new_sawtooth(non_normal_to_default(value)),
            Triangle => SNFloat::new_triangle(non_normal_to_default(value)),
            Sin => SNFloat::new_sin(non_normal_to_default(value)),
            SinRepeating => SNFloat::new_sin_repeating(non_normal_to_default(value)),
            TanH => SNFloat::new_tanh(non_normal_to_default(value)),
            Clamp => SNFloat::new_clamped(non_normal_to_default(value)),
            Fractional => SNFloat::new_fractional(non_normal_to_default(value)),
            Random => SNFloat::new_random_clamped(non_normal_to_default(value)),
        }
    }
}

impl<'a> Updatable<'a> for SFloatNormaliser {
    type UpdateArg = ProtoUpdArg<'a>;

    fn update(&mut self, mut _arg: ProtoUpdArg<'a>) {}
}

#[derive(
    Clone, Copy, Generatable, UpdatableRecursively, Mutatable, Serialize, Deserialize, Debug,
)]
#[mutagen(gen_arg = type (), mut_arg = type ())]
pub enum UFloatNormaliser {
    //TODO: Add sigmoid function
    Sawtooth,
    Triangle,
    Sin,
    SinRepeating,
    Clamp,
    Random,
}

impl UFloatNormaliser {
    pub fn normalise(self, value: f32) -> UNFloat {
        use UFloatNormaliser::*;

        match self {
            Sawtooth => UNFloat::new_sawtooth(non_normal_to_default(value)),
            Triangle => UNFloat::new_triangle(non_normal_to_default(value)),
            Sin => UNFloat::new_sin(non_normal_to_default(value)),
            SinRepeating => UNFloat::new_sin_repeating(non_normal_to_default(value)),
            Clamp => UNFloat::new_clamped(non_normal_to_default(value)),
            Random => UNFloat::new_random_clamped(non_normal_to_default(value)),
        }
    }
}

impl<'a> Updatable<'a> for UFloatNormaliser {
    type UpdateArg = ProtoUpdArg<'a>;

    fn update(&mut self, mut _arg: ProtoUpdArg<'a>) {}
}

fn non_normal_to_default(value: f32) -> f32 {
    if value.is_normal() {
        value
    } else {
        f32::default()
    }
}
