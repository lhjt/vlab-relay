macro_rules! unauthenticated {
    ($msg:expr) => {
        Err(Status::new(tonic::Code::Unauthenticated, $msg))
    };
}

macro_rules! generic_failed {
    ($msg:expr, $($arg:expr),*) => {{
        error!($msg, $($arg),*);
        Ok(Response::new(GenericResponse {
            success: true,
            error:   format!($msg, $($arg),*),
        }))
    }};
}

macro_rules! generic_success {
    () => {
        Ok(Response::new(GenericResponse {
            success: true,
            error:   "".to_string(),
        }))
    };
}

macro_rules! validate_admin {
    ($request:expr) => {
        if !is_admin($request.metadata()).unwrap_or(false) {
            return unauthenticated!("You must be authenticated as an admin to use this service.");
        }
    };
}
