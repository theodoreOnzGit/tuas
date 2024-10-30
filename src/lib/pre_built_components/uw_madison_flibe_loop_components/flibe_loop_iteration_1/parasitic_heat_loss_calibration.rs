/// In the reference,
/// Britsch, K., Anderson, M., Brooks, P., & 
/// Sridharan, K. (2019). Natural circulation 
/// FLiBe loop overview. International Journal of 
/// Heat and Mass Transfer, 134, 970-983.
///
/// Heater power is given in the tables A6. from 952 watts in test 1 
/// to 1652 watts in test 10 
///
/// Now, there are four heaters, so I'm not sure if the heater power in A6 
/// refers to heat added by individual heaters or heat added by the four 
/// heaters
///
/// Hence, I needed to back calculate what is the heat addition to the flibe 
/// fluid and compare that against the heat added by the four heaters
///
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test1(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (952.0, 2288.64525709192,2.75);

    let regression_heat_retention_fraction: f64 = 0.601009783900188;

    let salt_temp_change_degc: f64 = 59.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}

/// same as test 1 but for test 5
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test5(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (1471.0, 5213.45631446023, 6.72);

    let regression_heat_retention_fraction: f64 = 0.886039482403166;

    let salt_temp_change_degc: f64 = 55.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}


/// same as test 1 but for test 10
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test10(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (1644.0, 5695.16500801763, 4.75);

    let regression_heat_retention_fraction: f64 = 0.866053072995382;

    let salt_temp_change_degc: f64 = 85.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}


// calibrates parasitic heat losses for the heater given a fixed flowrate 
//
// the data is as follows
//
// test no.,TC 11(degC),TC 12(degC),TC 14(degC),TC 21(degC),TC 24(degC),TC 32(degC),TC 35(degC)
// 1,(514.1,520.8,535.9,542.3,490.4,487.9,483.4)
// 2,(576.3,580.4,592,600.9,553.1,550.4,546.7)
// 3,(638.5,645.9,669.1,667.6,620.9,619,617.4)
// 4,(538.6,543.8,569.9,571.4,502.1,496.8,497.4)
// 5,(692.2,699.8,722,720.5,672.7,675.7,665.7)
// 6,(590.7,597.6,623,626.8,562.3,560.8,560.2)
// 7,(548.8,553.8,572.7,583.4,510.2,509.5,508.2)
// 8,(603.9,611.5,638.7,641.4,572.6,567.2,571.3)
// 9,(572.5,578.9,600.2,612,536.2,529.3,535.6)
// 10,(549.9,556,587,589.8,499.2,500,504.6)

