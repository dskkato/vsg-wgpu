#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMessage {
    #[prost(oneof="set_message::Command", tags="1, 2")]
    pub command: ::core::option::Option<set_message::Command>,
}
/// Nested message and enum types in `SetMessage`.
pub mod set_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag="1")]
        SetShape(super::Shape),
        #[prost(message, tag="2")]
        SetBgColor(super::BgColor),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shape {
    #[prost(oneof="shape::Shape", tags="1, 2, 3")]
    pub shape: ::core::option::Option<shape::Shape>,
}
/// Nested message and enum types in `Shape`.
pub mod shape {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Shape {
        #[prost(message, tag="1")]
        Square(super::Square),
        #[prost(message, tag="2")]
        Circle(super::Circle),
        #[prost(message, tag="3")]
        Cross(super::Cross),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BgColor {
    #[prost(float, repeated, tag="1")]
    pub color: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinates {
    #[prost(float, tag="1")]
    pub x: f32,
    #[prost(float, tag="2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Square {
    #[prost(float, tag="1")]
    pub size: f32,
    #[prost(message, optional, tag="2")]
    pub ctr: ::core::option::Option<Coordinates>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Circle {
    #[prost(float, tag="1")]
    pub radius: f32,
    #[prost(message, optional, tag="2")]
    pub ctr: ::core::option::Option<Coordinates>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cross {
    #[prost(float, tag="1")]
    pub size: f32,
    #[prost(float, tag="2")]
    pub line_width: f32,
    #[prost(message, optional, tag="3")]
    pub ctr: ::core::option::Option<Coordinates>,
}
