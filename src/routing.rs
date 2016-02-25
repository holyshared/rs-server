use nickel::router:: { Router, HttpRouter };
use middleware:: { HealthCheckHandler };

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/foo", middleware! { |_|
        "foo"
    });
    router.get("/bar", middleware! { |_|
        "bar"
    });
    router.get("/healthcheck", HealthCheckHandler);
    router
}
