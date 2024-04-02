pub(crate) trait RoundToTwoDecimalPlaces {
    fn round_to_two_decimal_places(self) -> f32;
}

impl RoundToTwoDecimalPlaces for f32 {
     fn round_to_two_decimal_places(self) -> f32 {
         (self * 100000.0).round() / 100000.0
    }
}