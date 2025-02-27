use super::Response;
use warp::http::StatusCode;

mod offsets {
    pub const AUTHENTICATION: u32 = 0;
    pub const AUTHORIZATION: u32 = 1;
    pub const VALIDATION: u32 = 2;
    pub const NOT_IMPLEMENTED: u32 = 3;
    pub const NOT_FOUND: u32 = 4;
    pub const INTERNAL: u32 = 5;
    pub const TIMEOUT: u32 = 6;
}

pub fn authentication(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::UNAUTHORIZED,
        "Invalid access token.",
        code_prefix as u32 * 10000 + offsets::AUTHENTICATION * 100,
    )
}

pub fn authorization(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::FORBIDDEN,
        "Permission denied.",
        code_prefix as u32 * 10000 + offsets::AUTHORIZATION * 100,
    )
}

// todo validation errors after error details are implemented

pub mod validation {
    use super::{offsets, Response};
    use warp::http::StatusCode;

    pub fn missing_parameter(code_prefix: u16) -> Response {
        Response::singleton(
            StatusCode::BAD_REQUEST,
            "Missing required parameter.",
            code_prefix as u32 * 10000 + offsets::VALIDATION * 100,
        )
    }

    pub fn invalid_parameter(code_prefix: u16) -> Response {
        Response::singleton(
            StatusCode::BAD_REQUEST,
            "Invalid parameter value.",
            code_prefix as u32 * 10000 + offsets::VALIDATION * 100 + 1,
        )
    }

    pub fn missing_header(code_prefix: u16) -> Response {
        Response::singleton(
            StatusCode::BAD_REQUEST,
            "Missing required header.",
            code_prefix as u32 * 10000 + offsets::VALIDATION * 100 + 2,
        )
    }

    pub fn invalid_header(code_prefix: u16) -> Response {
        Response::singleton(
            StatusCode::BAD_REQUEST,
            "Invalid header value.",
            code_prefix as u32 * 10000 + offsets::VALIDATION * 100 + 3,
        )
    }

    pub fn body_deserialization(code_prefix: u16) -> Response {
        Response::singleton(
            StatusCode::BAD_REQUEST,
            "Body deserialization error.",
            code_prefix as u32 * 10000 + offsets::VALIDATION * 100 + 4,
        )
    }
}

pub fn not_implemented(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::NOT_IMPLEMENTED,
        "Not implemented.",
        code_prefix as u32 * 10000 + offsets::NOT_IMPLEMENTED * 100,
    )
}

pub fn not_found(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::NOT_FOUND,
        "Not found.",
        code_prefix as u32 * 10000 + offsets::NOT_FOUND * 100,
    )
}

pub fn internal(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::INTERNAL_SERVER_ERROR,
        internal::MESSAGE,
        code_prefix as u32 * 10000 + offsets::INTERNAL * 100,
    )
}

// todo subcodes after error details
pub mod internal {
    //     use super::Response;
    //     use warp::http::StatusCode;

    pub const MESSAGE: &str = "Internal server error";
    //     pub const CODE_OFFSET: u32 = 500;

    //     // todo subcode in details
    //     fn database(code_prefix: u16) -> Response {
    //         Response::singleton(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             MESSAGE,
    //             code_prefix as u32 * 10000 + CODE_OFFSET,
    //         )
    //     }

    //     fn upstream(code_prefix: u16) -> Response {
    //         Response::singleton(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             MESSAGE,
    //             code_prefix as u32 * 10000 + CODE_OFFSET,
    //         )
    //     }
}

pub fn timeout(code_prefix: u16) -> Response {
    Response::singleton(
        StatusCode::GATEWAY_TIMEOUT,
        "Timed out.",
        code_prefix as u32 * 10000 + offsets::TIMEOUT * 100,
    )
}
