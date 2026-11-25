use worker::*;

#[event(fetch)]
async fn fetch(
 mut req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    
    let form = match req.form_data().await{
        Ok(form) => form,
        Err(_) => {
            return Response::error("Failed to parse form data", 400);
        }
    };
    
    if let Some(name) = form.get_field("name") {
        Response::ok(format!("Hello, {}!", name))
    } else {
        Response::error("Missing 'name' field in form data", 400)
    }
}