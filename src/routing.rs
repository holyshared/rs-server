use nickel::router:: { Router, HttpRouter };
use middleware:: { HealthCheckHandler };

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/healthcheck", HealthCheckHandler);
    router
}
