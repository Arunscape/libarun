// will use f64 until f128 is in stable
pub fn celsius_to_farenheit<N>(celsius: N) -> f64
where
    N: Into<f64>,
{
    celsius.into() * 9.0 / 5.0 + 32.0
}
pub fn farenheit_to_celsius<N>(farenheit: N) -> f64
where
    N: Into<f64>,
{
    (farenheit.into() - 32.0) * 5.0 / 9.0
}
pub fn lbs_to_kg<N>(lbs: N) -> f64
where
    N: Into<f64>,
{
    // https://books.google.ca/books?id=4aWN-VRV1AoC&pg=PA13&redir_esc=y#v=onepage&q&f=false
    lbs.into() * 0.45359237
}
pub fn kg_to_lbs<N>(kg: N) -> f64
where
    N: Into<f64>,
{
    kg.into() / 0.45359237
}
pub fn miles_to_km<N>(miles: N) -> f64
where
    N: Into<f64>,
{
    miles.into() * 1.609344
}
pub fn km_to_miles<N>(km: N) -> f64
where
    N: Into<f64>,
{
    km.into() / 1.609344
}
pub fn inches_to_cm<N>(inches: N) -> f64
where
    N: Into<f64>,
{
    inches.into() * 2.54
}
pub fn cm_to_inches<N>(cm: N) -> f64
where
    N: Into<f64>,
{
    cm.into() / 2.54
}
pub fn yards_to_meters<N>(yards: N) -> f64
where
    N: Into<f64>,
{
    yards.into() * 0.9144
}
pub fn meters_to_yards<N>(meters: N) -> f64
where
    N: Into<f64>,
{
    meters.into() / 0.9144
}
pub fn feet_to_meters<N>(feet: N) -> f64
where
    N: Into<f64>,
{
    feet.into() * 0.3048
}
pub fn meters_to_feet<N>(meters: N) -> f64
where
    N: Into<f64>,
{
    meters.into() / 0.3048
}
pub fn gallons_to_liters<N>(gallons: N) -> f64
where
    N: Into<f64>,
{
    gallons.into() * 3.785411784
}
pub fn liters_to_gallons<N>(liters: N) -> f64
where
    N: Into<f64>,
{
    liters.into() / 3.785411784
}
pub fn ounces_to_grams<N>(ounces: N) -> f64
where
    N: Into<f64>,
{
    ounces.into() * 28.349523125
}
pub fn grams_to_ounces<N>(grams: N) -> f64
where
    N: Into<f64>,
{
    grams.into() / 28.349523125
}
pub fn pints_to_liters<N>(pints: N) -> f64
where
    N: Into<f64>,
{
    pints.into() * 0.473176473
}
pub fn liters_to_pints<N>(liters: N) -> f64
where
    N: Into<f64>,
{
    liters.into() / 0.473176473
}
pub fn quarts_to_liters<N>(quarts: N) -> f64
where
    N: Into<f64>,
{
    quarts.into() * 0.946352946
}
pub fn liters_to_quarts<N>(liters: N) -> f64
where
    N: Into<f64>,
{
    liters.into() / 0.946352946
}
/// US customary cup
pub fn cups_to_milliliters<N>(cups: N) -> f64
where
    N: Into<f64>,
{
    cups.into() * 236.5882365
}
/// US customary cup
pub fn milliliters_to_cups<N>(milliliters: N) -> f64
where
    N: Into<f64>,
{
    milliliters.into() / 236.5882365
}
pub fn tablespoons_to_milliliters<N>(tablespoons: N) -> f64
where
    N: Into<f64>,
{
    tablespoons.into() * 14.78676478125
}
pub fn milliliters_to_tablespoons<N>(milliliters: N) -> f64
where
    N: Into<f64>,
{
    milliliters.into() / 14.78676478125
}
pub fn teaspoons_to_milliliters<N>(teaspoons: N) -> f64
where
    N: Into<f64>,
{
    teaspoons.into() * 4.92892159375
}
pub fn milliliters_to_teaspoons<N>(milliliters: N) -> f64
where
    N: Into<f64>,
{
    milliliters.into() / 4.92892159375
}
pub fn fluid_ounces_to_milliliters<N>(fluid_ounces: N) -> f64
where
    N: Into<f64>,
{
    fluid_ounces.into() * 29.5735295625
}
pub fn milliliters_to_fluid_ounces<N>(milliliters: N) -> f64
where
    N: Into<f64>,
{
    milliliters.into() / 29.5735295625
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_celsius_to_farenheit() {
        assert_eq!(celsius_to_farenheit(0.0), 32.0);
        assert_eq!(celsius_to_farenheit(100.0), 212.0);
        assert_eq!(celsius_to_farenheit(-40.0), -40.0);
    }
    #[test]
    fn test_farenheit_to_celsius() {
        assert_eq!(farenheit_to_celsius(32.0), 0.0);
        assert_eq!(farenheit_to_celsius(212.0), 100.0);
        assert_eq!(farenheit_to_celsius(-40.0), -40.0);
    }
    #[test]
    fn test_lbs_to_kg() {
        assert_eq!(lbs_to_kg(1.0), 0.45359237);
    }
    #[test]
    fn test_kg_to_lbs() {
        assert_eq!(kg_to_lbs(1.0), 1.0 / 0.45359237);
    }
    #[test]
    fn test_miles_to_km() {
        assert_eq!(miles_to_km(1.0), 1.609344);
    }
    #[test]
    fn test_km_to_miles() {
        assert_eq!(km_to_miles(1.0), 1.0 / 1.609344);
    }
    #[test]
    fn test_inches_to_cm() {
        assert_eq!(inches_to_cm(1.0), 2.54);
    }
    #[test]
    fn test_cm_to_inches() {
        assert_eq!(cm_to_inches(1.0), 1.0 / 2.54);
    }
    #[test]
    fn test_yards_to_meters() {
        assert_eq!(yards_to_meters(1.0), 0.9144);
    }
    #[test]
    fn test_meters_to_yards() {
        assert_eq!(meters_to_yards(1.0), 1.0 / 0.9144);
    }
    #[test]
    fn test_feet_to_meters() {
        assert_eq!(feet_to_meters(1.0), 0.3048);
    }
    #[test]
    fn test_meters_to_feet() {
        assert_eq!(meters_to_feet(1.0), 1.0 / 0.3048);
    }
    #[test]
    fn test_gallons_to_liters() {
        assert_eq!(gallons_to_liters(1.0), 3.785411784);
    }
    #[test]
    fn test_liters_to_gallons() {
        assert_eq!(liters_to_gallons(1.0), 1.0 / 3.785411784);
    }
    #[test]
    fn test_ounces_to_grams() {
        assert_eq!(ounces_to_grams(1.0), 28.349523125);
    }
    #[test]
    fn test_grams_to_ounces() {
        assert_eq!(grams_to_ounces(1.0), 1.0 / 28.349523125);
    }
    #[test]
    fn test_pints_to_liters() {
        assert_eq!(pints_to_liters(1.0), 0.473176473);
    }
    #[test]
    fn test_liters_to_pints() {
        assert_eq!(liters_to_pints(1.0), 1.0 / 0.473176473);
    }
    #[test]
    fn test_quarts_to_liters() {
        assert_eq!(quarts_to_liters(1.0), 0.946352946);
    }
    #[test]
    fn test_liters_to_quarts() {
        assert_eq!(liters_to_quarts(1.0), 1.0 / 0.946352946);
    }
    #[test]
    fn test_cups_to_milliliters() {
        assert_eq!(cups_to_milliliters(1.0), 236.5882365);
    }
    #[test]
    fn test_milliliters_to_cups() {
        assert_eq!(milliliters_to_cups(1.0), 1.0 / 236.5882365);
    }
    #[test]
    fn test_tablespoons_to_milliliters() {
        assert_eq!(tablespoons_to_milliliters(1.0), 14.78676478125);
    }
    #[test]
    fn test_milliliters_to_tablespoons() {
        assert_eq!(milliliters_to_tablespoons(1.0), 1.0 / 14.78676478125);
    }
    #[test]
    fn test_teaspoons_to_milliliters() {
        assert_eq!(teaspoons_to_milliliters(1.0), 4.92892159375);
    }
    #[test]
    fn test_milliliters_to_teaspoons() {
        assert_eq!(milliliters_to_teaspoons(1.0), 1.0 / 4.92892159375);
    }
    #[test]
    fn test_fluid_ounces_to_milliliters() {
        assert_eq!(fluid_ounces_to_milliliters(1.0), 29.5735295625);
    }
    #[test]
    fn test_milliliters_to_fluid_ounces() {
        assert_eq!(milliliters_to_fluid_ounces(1.0), 1.0 / 29.5735295625);
    }
}
