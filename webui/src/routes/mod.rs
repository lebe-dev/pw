pub mod home;
pub mod secret;

#[derive(PartialEq)]
pub enum PageState {
    Loading,
    Ready,
    NotFound,
    Error
}