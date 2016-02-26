use nickel:: { StaticFilesHandler };
use nickel::router:: { Router, HttpRouter };
use middleware:: { HealthCheckHandler };
use std::env:: { current_dir };

pub fn router() -> Router {
    let root = current_dir().unwrap();
    let mut router = Router::new();
    router.get("*", StaticFilesHandler::new(root.as_path()));
    router.get("/healthcheck", HealthCheckHandler);
    router
}
