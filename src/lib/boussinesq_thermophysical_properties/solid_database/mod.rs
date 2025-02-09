/// stainless steel 304L 
pub mod ss_304_l;

/// copper 
pub mod copper;

/// fiberglass 
pub mod fiberglass;

/// custom material for solid 
pub mod custom_solid_material;


/// pyrogel hps 
///
/// This is an aerogel with silica fibres.
///
/// Most information comes from:
///
/// Kovács, Z., Csík, A., & Lakatos, Á. (2023). 
/// Thermal stability investigations of different 
/// aerogel insulation materials at elevated temperature.
/// Thermal Science and Engineering Progress, 42, 101906.
pub mod pyrogel_hps;


// standby code for heating elements and radiative heaters

///// generic heating element for 
///// heater, based roughly on tungsten
//#[cfg(test)]
//pub mod generic_heating_element;
//
///// FeCrAl, used as a heating element or for alloys in LWR
//pub mod fecral;
