use nickel:: { Request, Response, Middleware, MiddlewareResult };
use nickel::status::StatusCode;

pub struct HealthCheckHandler;

impl<D> Middleware<D> for HealthCheckHandler {

    fn invoke<'a>(&self, _: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        res.send(StatusCode::NoContent)
    }

}
