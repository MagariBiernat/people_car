use actix_web::{get, web, HttpResponse, Responder, http::{ StatusCode }};
use serde::{Deserialize};
use rand::Rng;
extern crate vin;

#[derive(Debug, Deserialize)]
pub struct CarInfo {
	distance: f64,
	yearOfProduction: i16,
	fuelUsagePer100KM: f64
}
	
#[get("/calculateDieselUsageForDistance")]
pub async fn calculate_diesel_usage_for_distance(info: web::Query<CarInfo>) -> impl Responder {

	let calculate_fuel_consumption = format!("{:.2}", ((&info.distance * 0.01) * &info.fuelUsagePer100KM));

	println!("{}", calculate_fuel_consumption);

    HttpResponse::Ok()
		.body(calculate_fuel_consumption)
}


#[derive(Debug, Deserialize)]
pub struct UnitInfo {
	VIN: String
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn probability_of_unit_ijector_fail(info: web::Query<UnitInfo>) -> impl Responder {

	let is_invalid_vin = vin::check_validity(&info.VIN).is_err();

	if is_invalid_vin {
		println!("Vin is invalid!");
		return HttpResponse::BadRequest()
			.status(StatusCode::BAD_REQUEST)
			.body("Wrong VIN")
	}

	let probability_of_unit_fail: f32 = (rand::thread_rng().gen_range(0..101) as f32) * 0.01;
	let float_probability_result = format!("
		Probability fail for PeopleCar PasWagon C6 identified by VIN number: {} is equal to {}", info.VIN, probability_of_unit_fail);

	print!("\n {} \n", float_probability_result);

	let result = format!("{:.2}", probability_of_unit_fail);

	HttpResponse::Ok()
		.body(result)
}