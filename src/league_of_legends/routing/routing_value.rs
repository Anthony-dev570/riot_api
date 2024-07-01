pub trait RoutingValue: ToString {
    fn host(&self) -> String;
}