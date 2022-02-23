use tonic::{Request, Status};

pub(crate) fn auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let meta = req.metadata();
    let auth_data = meta.get("Authorization");

    if auth_data.is_none() {
        return Err(Status::permission_denied("Invalid credentials"));
    }

    let auth_data = auth_data.unwrap();
    let auth_data = auth_data.to_str().ok();
    if auth_data.is_none() {
        return Err(Status::permission_denied("Invalid credentials"));
    }

    let auth_data = auth_data.unwrap();
    if !auth_data.starts_with("Bearer ") {
        return Err(Status::permission_denied("Invalid credentials"));
    }

    let token = auth_data.replace("Bearer ", "");

    // TODO: handle actual auth
    if token.len() > 100 {
        return Err(Status::permission_denied("Invalid credentials"));
    }

    req.metadata_mut()
        .insert("zid", "z5555555".parse().unwrap());

    Ok(req)
}
