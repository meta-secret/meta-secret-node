use rocket::{routes, Route};

pub mod commons;
pub mod membership;
pub mod password;
pub mod register;
pub mod vault;

pub fn meta_secret_routes() -> Vec<Route> {
    routes![
        commons::hi,
        register::register,
        vault::get_vault,
        membership::accept,
        membership::decline,
        password::claim_for_password_recovery,
        password::distribute,
        password::find_shares,
        password::passwords,
        password::find_password_recovery_claims,
    ]
}
