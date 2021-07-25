use crate::{RequestApp, RequestHyper, ResultResponseHyper};
use route_recognizer::Params;

pub trait Handler {
    fn new(app: RequestApp) -> Self;

    fn call(&self, req: RequestHyper, params: &Params) -> ResultResponseHyper;
}
