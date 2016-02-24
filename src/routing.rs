use nickel::router:: { Router, HttpRouter };

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/foo", middleware! { |_|
        "foo"
    });
    router.get("/bar", middleware! { |_|
        "bar"
    });
    router
}
