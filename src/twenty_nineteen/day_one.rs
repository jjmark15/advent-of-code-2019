pub fn get_fuel_requirement(mass: i64) -> i64 {
    let floor_divide_by_3: i64 = mass / 3;
    floor_divide_by_3 - 2
}

pub fn get_fuel_requirement_bulk(masses: &Vec<i64>) -> i64 {
    masses.into_iter().map(|mass| get_fuel_requirement(*mass)).sum()
}

#[cfg(test)]
mod tests {
    use crate::twenty_nineteen::day_one::{get_fuel_requirement, get_fuel_requirement_bulk};

    #[test]
    fn get_fuel_requirement_test() {
        assert_eq!(get_fuel_requirement(12), 2);
        assert_eq!(get_fuel_requirement(14), 2);
        assert_eq!(get_fuel_requirement(1969), 654);
        assert_eq!(get_fuel_requirement(100756), 33583);
    }

    #[test]
    fn get_fuel_requirement_bulk_test() {
        assert_eq!(get_fuel_requirement_bulk(vec![12, 14, 1969].as_ref()), 658);
    }
}