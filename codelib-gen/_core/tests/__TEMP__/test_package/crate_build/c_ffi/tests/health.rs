// Namespace health

#[test]
pub fn health_check() {
	let healthcheck = HealthCheck {};

	healthcheck.alive();
	healthcheck.capabilities();

}

