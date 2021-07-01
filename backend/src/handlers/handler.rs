use route_recognizer::Params;
use crate::{RequestApp, RequestHyper, ResultResponseHyper};

pub trait Handler {
    fn new (app: RequestApp) -> Self;

    fn call (&self, req: RequestHyper, params: &Params) -> ResultResponseHyper;
}
