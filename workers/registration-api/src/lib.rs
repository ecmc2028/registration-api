use shared::form::Participant;
use worker::*;

#[event(fetch)]
async fn fetch(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let form = match req.form_data().await {
        Ok(form) => form,
        Err(_) => {
            return Response::error("Failed to parse form data", 400);
        }
    };

    match Participant::try_from(&form) {
        Ok(participant) => {
            let db = env.d1("ecmc2028_registration")?;
            let query = participant.get_insert_query(&db)?;
            query.run().await?;
            Response::ok("Registered 😇")
        }
        Err(e) => Response::error(format!("{e:?}"), 400),
    }
}
