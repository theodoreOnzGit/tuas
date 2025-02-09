
use std::f64::consts::PI;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::*;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::NusseltPrandtlReynoldsData;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::WakaoData;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;

use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::area::square_inch;
use uom::si::f64::*;
use uom::si::length::{inch, meter};
use uom::ConstZero;
use uom::si::area::square_meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
/// represents heater version 2 without insulation 
/// This is because during 2018-ish, the heater insulation 
/// got burnt off and a lot of frequency response tests were done 
/// with insulation removed
///
/// Heater version 2 bare has no insulation
/// but it has a twisted tape interior
///
///
/// note that it only contains the heated section, not the top nor 
/// bottom heads
///
/// note: the pressure drop correlations are not yet properly implemented 
/// so it behaves like a pipe in terms of pressure drop
/// For now, I did not do anything special with it
#[derive(Debug,Clone,PartialEq)]
pub struct NonInsulatedPorousMediaFluidComponent {

    inner_nodes: usize,


    /// heat transfer entity representing control volumes 
    /// of heat generating or 
    /// non-heat generating components within the pipe 
    /// or fluid component 
    ///
    /// for example,
    /// the twisted tape in the heated section of CIET's Heater
    pub interior_solid_array_for_porous_media: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the steel piping in the heated section of CIET's Heater
    pub pipe_shell: HeatTransferEntity,

    /// this HeatTransferEntity represents the pipe fluid
    /// which is coupled to the pipe shell via a Nusselt Number based
    /// thermal resistance (usually Gnielinski correlation)
    /// But it is up to you to specify
    ///
    /// heat transfer entity representing control volumes 
    /// for the therminol fluid in the heated section of CIET's Heater
    pub pipe_fluid_array: HeatTransferEntity,

    /// 
    /// pipe heat transfer coefficient to ambient
    /// eg.
    /// ambient temperature of air used to calculate heat loss
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient used to calculate heat loss 
    /// to ambient, such as air air
    pub heat_transfer_to_ambient: HeatTransfer,


    /// flow area
    flow_area: Area,

    /// loss correlations
    pub darcy_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// thermal conductance lengthscale to ambient 
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub solid_side_thermal_conductance_lengthscale_pipe_to_ambient: Length,

    /// thermal conductance lengthscale from pipe to fluid
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub solid_side_thermal_conductance_lengthscale_pipe_to_fluid: Length,

    /// thermal conductance lengthscale from fluid to 
    /// porous media internal
    /// 
    /// for calculating thermal resistance, we need a length 
    /// scale 
    ///
    /// thermal conductance = (kA)/L
    /// 
    /// assuming 1D cartesian coordinates, you need to specify 
    /// a lengthscale for an appropraite thermal resistance.
    ///
    /// This is not L, but rather A/L
    ///
    /// to get thermal conductance just A/L * k
    /// basically...
    pub solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length,



    /// nusselt correlation to pipe shell (to ambient)
    pub nusselt_correlation_to_pipe_shell: NusseltCorrelation,

    /// lengthscale for nusselt correlation to ambient 
    /// for pipes, the hydraulic diameter usually suffices 
    pub nusselt_correlation_lengthscale_to_ambient: Length,

    /// convection heat transfer area to ambient 
    /// used to calculate conductance to ambient hA
    /// conductance = h A 
    pub convection_heat_transfer_area_to_ambient: Area,

    /// nusselt correlation to porous media interior
    pub nusselt_correlation_to_porous_media_interior: NusseltCorrelation,

    /// lengthscale for nusselt correlation to porous_media_interior 
    /// for pipes, the hydraulic diameter usually suffices 
    pub nusselt_correlation_lengthscale_to_porous_media_interior: Length,

    /// convection heat transfer area to pipe 
    /// used to calculate conductance to pipe hA
    /// conductance = h A 
    pub convection_heat_transfer_area_to_pipe: Area,

    /// convection heat transfer area to interior 
    /// used to calculate conductance to interior hA
    /// conductance = h A 
    pub convection_heat_transfer_area_to_interior: Area,

}


impl NonInsulatedPorousMediaFluidComponent {

    /// traditional callibrated heater constructor 
    /// with 20 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    ///
    pub fn new_dewet_model_heater_v2(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
        let steel_shell_id = Length::new::<meter>(0.0381);
        let steel_shell_od = Length::new::<meter>(0.04);


        // inner therminol array 
        //
        // the darcy loss correlation is f = 17.9 *Re^{-0.34}
        // accurate to within 4% (Lukas et al)
        // Improved Heat Transfer and Volume Scaling through 
        // Novel Heater Design
        // 

        let a = Ratio::ZERO;
        let b = Ratio::new::<ratio>(17.9);
        let c: f64  = -0.34;
        let mut therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            a,
            b,
            c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );

        // the therminol array nusselt correlation should be that of the 
        // heater 

        let heater_prandtl_reynolds_data: NusseltPrandtlReynoldsData 
        = NusseltPrandtlReynoldsData::default();
        therminol_array.nusselt_correlation = 
            NusseltCorrelation::CIETHeaterVersion2(
                heater_prandtl_reynolds_data
                );

        let darcy_loss_correlation = 
            therminol_array.fluid_component_loss_properties.clone();
        // the therminol arrays here use gnielinski correlation by 
        // default

        let wakao_correlation = NusseltCorrelation::Wakao(
            WakaoData::default()
        );

        

        // now, nusselt correlation to ambient and to porous media 
        // are the same, I did not do anything special because 
        // transient validation was not important (yet) 
        // when I originally wrote this code 
        let nusselt_correlation_to_ambient = therminol_array.nusselt_correlation;
        let nusselt_correlation_to_porous_media_interior = wakao_correlation;
        let nusselt_correlation_lengthscale_to_ambient = hydraulic_diameter;
        let nusselt_correlation_lengthscale_to_porous_media_interior = hydraulic_diameter;

        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            steel_shell_id,
            steel_shell_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        // for thermal conductance lengthscale for cylinder, we 
        // the easiest way is to get the actual conductance 
        // which is in terms of (kA/L) then divide by the conductivity
        let steel_shell_mid_diameter: Length = (steel_shell_od + steel_shell_id)/2.0;
        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();
        
        let steel_shell_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_shell_od,
                heated_length,
                steel_thermal_conductivity).unwrap();

        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_id,
                steel_shell_mid_diameter,
                heated_length,
                steel_thermal_conductivity).unwrap();


        let solid_side_thermal_conductance_lengthscale_pipe_to_ambient: Length = 
            steel_shell_conductance_to_ambient/steel_thermal_conductivity;

        let solid_side_thermal_conductance_lengthscale_pipe_to_fluid: Length = 
            steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        // for this iteration of the heater, I'm kind of lazy 
        // my conductance lengthscale to the porous media interior
        // is kind of guesswork
        //
        // I could put a large number here to to neglect the 
        // resistance of the porous media fluid
        // In fact, when calculating thermal resistance for the 
        // twisted tape, I ignored the resistance
        // of the twisted tape, so just put a large number here
        let solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);
            

        // now twisted_tape 
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        // convection heat transfer area to interior (twisted tape)
        // this is approximate btw
        //
        // I just copied this straight from the preprocessing 
        // bit to be consistent

        // find suitable heat transfer area
        let heated_length = Length::new::<meter>(1.6383);
        let heated_length_plus_heads = Length::new::<inch>(78.0);

        let heat_transfer_area_heated_length_plus_heads: Area = 
            Area::new::<square_inch>(719.0);

        let heat_transfer_area_heated_length_only: Area
            = heated_length/ heated_length_plus_heads * 
            heat_transfer_area_heated_length_plus_heads;

        let convection_heat_transfer_area_to_interior = 
            heat_transfer_area_heated_length_only;
        // area = PI * inner diameter * L
        let convection_heat_transfer_area_to_pipe: Area 
            = PI * steel_shell_id * heated_length;



        // area = PI * outer diameter * L 
        let convection_heat_transfer_area_to_ambient: Area 
            = PI * steel_shell_od * heated_length;

        return Self { inner_nodes: user_specified_inner_nodes,
            interior_solid_array_for_porous_media: twisted_tape.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            solid_side_thermal_conductance_lengthscale_pipe_to_ambient,
            solid_side_thermal_conductance_lengthscale_pipe_to_fluid,
            solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            nusselt_correlation_to_pipe_shell: nusselt_correlation_to_ambient,
            nusselt_correlation_to_porous_media_interior,
            nusselt_correlation_lengthscale_to_ambient,
            nusselt_correlation_lengthscale_to_porous_media_interior,
            convection_heat_transfer_area_to_ambient,
            convection_heat_transfer_area_to_pipe,
            convection_heat_transfer_area_to_interior,

        };
    }
    /// traditional uncallibrated heater constructor 
    /// with 6 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    /// 6 W/(m^2 K) is the heat transfer coefficeint assuming natural 
    /// convection only 
    ///
    /// it was increased to 20 W/(m^2 K) because of the support structures 
    /// and other such losses
    pub fn _new_six_watts_per_m2_kelvin_model_heater_v2_model(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let dummy_pipe_form_loss = Ratio::new::<ratio>(0.1);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);
        let steel_shell_id = Length::new::<meter>(0.0381);
        let steel_shell_od = Length::new::<meter>(0.04);


        let mut therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            dummy_pipe_form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // the therminol arrays here use gnielinski correlation by 
        // default, but
        // the therminol array nusselt correlation should be that of the 
        // heater 

        let heater_prandtl_reynolds_data: NusseltPrandtlReynoldsData 
        = NusseltPrandtlReynoldsData::default();
        therminol_array.nusselt_correlation = 
            NusseltCorrelation::CIETHeaterVersion2(
                heater_prandtl_reynolds_data
                );


        let wakao_correlation = NusseltCorrelation::Wakao(
            WakaoData::default()
        );

        

        // now, nusselt correlation to ambient and to porous media 
        // are the same, I did not do anything special because 
        // transient validation was not important (yet) 
        // when I originally wrote this code 
        let nusselt_correlation_to_ambient = therminol_array.nusselt_correlation;
        let nusselt_correlation_to_porous_media_interior = wakao_correlation;
        let nusselt_correlation_lengthscale_to_ambient = hydraulic_diameter;
        let nusselt_correlation_lengthscale_to_porous_media_interior = hydraulic_diameter;

        let darcy_loss_correlation = 
            therminol_array.fluid_component_loss_properties.clone();
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            steel_shell_id,
            steel_shell_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        // for thermal conductance lengthscale for cylinder, we 
        // the easiest way is to get the actual conductance 
        // which is in terms of (kA/L) then divide by the conductivity
        let steel_shell_mid_diameter: Length = (steel_shell_od + steel_shell_id)/2.0;
        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();
        
        let steel_shell_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_shell_od,
                heated_length,
                steel_thermal_conductivity).unwrap();

        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_id,
                steel_shell_mid_diameter,
                heated_length,
                steel_thermal_conductivity).unwrap();


        let solid_side_thermal_conductance_lengthscale_pipe_to_ambient: Length = 
            steel_shell_conductance_to_ambient/steel_thermal_conductivity;

        let solid_side_thermal_conductance_lengthscale_pipe_to_fluid: Length = 
            steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        // for this iteration of the heater, I'm kind of lazy 
        // my conductance lengthscale to the porous media interior
        // is kind of guesswork
        //
        // I could put a large number here to to neglect the 
        // resistance of the porous media fluid
        // In fact, when calculating thermal resistance for the 
        // twisted tape, I ignored the resistance
        // of the twisted tape, so just put a large number here
        let solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);
            

        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 

        );
        // convection heat transfer area to interior (twisted tape)
        // this is approximate btw
        //
        // I just copied this straight from the preprocessing 
        // bit to be consistent

        let convection_heat_transfer_area_to_interior: Area 
            = Area::new::<square_inch>(719.0);

        // area = PI * inner diameter * L
        let convection_heat_transfer_area_to_pipe: Area 
            = PI * steel_shell_id * heated_length;

        // area = PI * outer diameter * L 
        let convection_heat_transfer_area_to_ambient: Area 
            = PI * steel_shell_od * heated_length;

        return Self { inner_nodes: user_specified_inner_nodes,
            interior_solid_array_for_porous_media: twisted_tape.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            solid_side_thermal_conductance_lengthscale_pipe_to_ambient,
            solid_side_thermal_conductance_lengthscale_pipe_to_fluid,
            solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            nusselt_correlation_to_pipe_shell: nusselt_correlation_to_ambient,
            nusselt_correlation_to_porous_media_interior,
            nusselt_correlation_lengthscale_to_ambient,
            nusselt_correlation_lengthscale_to_porous_media_interior,
            convection_heat_transfer_area_to_ambient,
            convection_heat_transfer_area_to_pipe,
            convection_heat_transfer_area_to_interior,

        };
    }

    /// user uncallibrated heater constructor 
    /// with 6 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    /// 6 W/(m^2 K) is the heat transfer coefficeint assuming natural 
    /// convection only 
    ///
    /// it was increased to 20 W/(m^2 K) because of the support structures 
    /// and other such losses
    pub fn ciet_heater_v2_generic_model(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize,
        h_to_air: HeatTransfer) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let dummy_pipe_form_loss = Ratio::new::<ratio>(0.1);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let steel_shell_id = Length::new::<meter>(0.0381);
        let steel_shell_od = Length::new::<meter>(0.04);


        // inner therminol array
        let mut therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            dummy_pipe_form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );


        // the therminol arrays here use gnielinski correlation by 
        // default, but 
        // the therminol array nusselt correlation should be that of the 
        // heater 

        let heater_prandtl_reynolds_data: NusseltPrandtlReynoldsData 
        = NusseltPrandtlReynoldsData::default();
        therminol_array.nusselt_correlation = 
            NusseltCorrelation::CIETHeaterVersion2(
                heater_prandtl_reynolds_data
                );

        let wakao_correlation = NusseltCorrelation::Wakao(
            WakaoData::default()
        );

        

        // now, nusselt correlation to ambient and to porous media 
        // are the same, I did not do anything special because 
        // transient validation was not important (yet) 
        // when I originally wrote this code 
        let nusselt_correlation_to_ambient = therminol_array.nusselt_correlation;
        let nusselt_correlation_to_porous_media_interior = wakao_correlation;
        let nusselt_correlation_lengthscale_to_ambient = hydraulic_diameter;
        let nusselt_correlation_lengthscale_to_porous_media_interior = hydraulic_diameter;

        let darcy_loss_correlation = 
            therminol_array.fluid_component_loss_properties.clone();
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            steel_shell_id,
            steel_shell_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // for thermal conductance lengthscale for cylinder, we 
        // the easiest way is to get the actual conductance 
        // which is in terms of (kA/L) then divide by the conductivity
        let steel_shell_mid_diameter: Length = (steel_shell_od + steel_shell_id)/2.0;
        let steel_thermal_conductivity: ThermalConductivity = 
            try_get_kappa_thermal_conductivity(
                SolidMaterial::SteelSS304L.into(), 
                initial_temperature, 
                atmospheric_pressure).unwrap();
        
        let steel_shell_conductance_to_ambient: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_mid_diameter,
                steel_shell_od,
                heated_length,
                steel_thermal_conductivity).unwrap();

        let steel_shell_conductance_to_fluid: ThermalConductance = 
            try_get_thermal_conductance_annular_cylinder(
                steel_shell_id,
                steel_shell_mid_diameter,
                heated_length,
                steel_thermal_conductivity).unwrap();


        let solid_side_thermal_conductance_lengthscale_pipe_to_ambient: Length = 
            steel_shell_conductance_to_ambient/steel_thermal_conductivity;

        let solid_side_thermal_conductance_lengthscale_pipe_to_fluid: Length = 
            steel_shell_conductance_to_fluid/steel_thermal_conductivity;

        // for this iteration of the heater, I'm kind of lazy 
        // my conductance lengthscale to the porous media interior
        // is kind of guesswork
        //
        // I could put a large number here to to neglect the 
        // resistance of the porous media fluid
        // In fact, when calculating thermal resistance for the 
        // twisted tape, I ignored the resistance
        // of the twisted tape, so just put a large number here
        let solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal: Length = 
            Length::new::<meter>(1e9 as f64);
            
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // convection heat transfer area to interior (twisted tape)
        // this is approximate btw
        //
        // I just copied this straight from the preprocessing 
        // bit to be consistent

        let convection_heat_transfer_area_to_interior: Area 
            = Area::new::<square_inch>(719.0);

        // area = PI * inner diameter * L
        let convection_heat_transfer_area_to_pipe: Area 
            = PI * steel_shell_id * heated_length;

        // area = PI * outer diameter * L 
        let convection_heat_transfer_area_to_ambient: Area 
            = PI * steel_shell_od * heated_length;

        return Self { inner_nodes: user_specified_inner_nodes,
            interior_solid_array_for_porous_media: twisted_tape.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_ambient: h_to_air,
            flow_area,
            darcy_loss_correlation,
            solid_side_thermal_conductance_lengthscale_pipe_to_ambient,
            solid_side_thermal_conductance_lengthscale_pipe_to_fluid,
            solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal,
            nusselt_correlation_to_pipe_shell: nusselt_correlation_to_ambient,
            nusselt_correlation_to_porous_media_interior,
            nusselt_correlation_lengthscale_to_ambient,
            nusselt_correlation_lengthscale_to_porous_media_interior,
            convection_heat_transfer_area_to_ambient,
            convection_heat_transfer_area_to_pipe,
            convection_heat_transfer_area_to_interior,
        };
    }

}




/// contains method implementations for obtaining conductances 
/// between the different arrays, and also laterally coupling 
/// the arrays to one another using a radial thermal resistance
pub mod preprocessing;


/// contains method implementations for FluidComponentTrait
/// This means all the stuff about getting mass flowrate from pressure 
/// and vice versa
pub mod fluid_entity;


/// contains methods to help advance timesteps (ie update the 
/// state of the control volumes after each timestep)
pub mod calculation;

/// for postprocessing, one can obtain temperature profiles 
/// of the component using the postprocessing modules
pub mod postprocessing;

/// for converting into fluid components 
pub mod type_conversion;


/// tests for all ciet's heaters in 
///
/// Ong, T. K. C. (2024). Digital Twins as Testbeds for 
/// Iterative Simulated Neutronics Feedback Controller Development 
/// (Doctoral dissertation, UC Berkeley).
///
/// The tuas_boussinesq_solver library was constructed with CIET 
/// in mind
///
/// This is the compact integral effects test from the UC Berkeley 
/// Thermal Hydraulics Lab
/// A Library which contains useful traits and methods for thermal 
/// hydraulics calculations.
///
///
/// This crate has heavy reliance on units of measure (uom) released under 
/// Apache 2.0 license. So you'll need to get used to unit safe calculations
/// with uom as well.
///
///
/// This library was initially developed for 
/// use in my PhD thesis under supervision 
/// of Professor Per F. Peterson. It a thermal hydraulics
/// library in Rust that is released under the GNU General Public License
/// v 3.0. This is partly due to the fact that some of the libraries 
/// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
/// Public License v3.0.
///
/// As such, the entire library is released under GNU GPL v3.0. It is a strong 
/// copyleft license which means you cannot use it in proprietary software.
///
///
/// License
///    This is a thermal hydraulics library written 
///    in rust meant to help with the
///    fluid mechanics and heat transfer aspects of the calculations
///    for the Compact Integral Effects Tests (CIET) and hopefully 
///    Gen IV Reactors such as the Fluoride Salt cooled High Temperature 
///    Reactor (FHR)
///     
///    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
///    Research and Safety Initiative, Per F. Peterson, University of 
///    California, Berkeley Thermal Hydraulics Laboratory
///
///    tuas_boussinesq_solver is free software; you can 
///    redistribute it and/or modify it
///    under the terms of the GNU General Public License as published by the
///    Free Software Foundation; either version 2 of the License, or (at your
///    option) any later version.
///
///    tuas_boussinesq_solver is distributed in the hope 
///    that it will be useful, but WITHOUT
///    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
///    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
///    for more details.
///
///    This thermal hydraulics library 
///    contains some code copied from GeN-Foam, and OpenFOAM derivative.
///    This offering is not approved or endorsed by the OpenFOAM Foundation nor
///    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
///    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
///    Nor is it endorsed by the authors and owners of GeN-Foam.
///
///    You should have received a copy of the GNU General Public License
///    along with this program.  If not, see <http://www.gnu.org/licenses/>.
///
/// © All rights reserved. Theodore Kay Chen Ong,
/// Singapore Nuclear Research and Safety Initiative,
/// Per F. Peterson,
/// University of California, Berkeley Thermal Hydraulics Laboratory
///
/// Main author of the code: Theodore Kay Chen Ong, supervised by
/// Professor Per F. Peterson
///
/// Btw, I no affiliation with the Rust Foundation. 
pub mod tests;
