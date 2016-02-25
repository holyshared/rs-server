use nickel:: { Request, Response, Middleware, MiddlewareResult };

pub struct HealthCheckHandler;

impl<D> Middleware<D> for HealthCheckHandler {

    fn invoke<'a>(&self, _: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        res.send("")
    }

}
