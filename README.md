# people_car

Simple api build with Rust and Actix Web Framework

________________________
It has two endpoints

/calculateDisselUsageForDistance
method: GET
receives 3 parameters in url

distance, yearOfProduction, fuelUsagePer100KM

all parameters must be provided and must be numbers

returns a number with calculated fuel consumption

________________________
/probabilityOfUnitInjectorFail
method: GET
receives 1 parameter in ur

VIN

VIN must be provided to successfully calculate probably of unit injector fail
also, VIN must be correct 

returns a number with probability of unit failure.



________________________

To run code,
file .env must be created in root folder with 2 variables
like so :

SERVER.HOST=127.0.0.1
SERVER.PORT=8080

also, cargo must be installed
then simply run command
cargo run
