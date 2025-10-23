use actix_web::{Error, HttpResponse, web};
use argon2::Config;
use serde_json::json;

use crate::iter6::{
    AppState, EzyTutorError, TutorRegisterForm, TutorResponse, TutorSigninForm, User,
    get_user_record_pool, post_new_user,
};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record_pool(&app_state.db, username.to_string()).await;
    let user_not_found: bool = user.is_err();

    // If user is not found in database, proceed to verification of passwords
    if user_not_found {
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);

            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            let mut response = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap();

            if response.status().is_success() {
                let res = response.body().await?;
                let tutor_response: TutorResponse =
                    serde_json::from_str(&std::str::from_utf8(&res)?)?;
                s = format!(
                    "Congratulations. You have been successfully registered with EzyTutor and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",
                    tutor_response.tutor_id
                );

                //Hash the password
                let salt = b"somerandomsalt";
                let config = Config::default();
                let hash = argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config)
                    .unwrap();
                let user = User {
                    username,
                    tutor_id: Some(tutor_response.tutor_id),
                    user_password: hash,
                };
                let _tutor_created = post_new_user(&app_state.db, user).await?;
            } else {
                ctx.insert("error", "Failed to register tutor. Please try again.");
                ctx.insert("current_username", &params.username);
                ctx.insert("current_password", "");
                ctx.insert("current_confirmation", "");
                ctx.insert("current_name", &params.name);
                ctx.insert("current_imageurl", &params.imageurl);
                ctx.insert("current_profile", &params.profile);

                s = tmpl
                    .render("register.html", &ctx)
                    .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
            }
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);

        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_signin_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    let s = tmpl
        .render("signin.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_signin(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorSigninForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record_pool(&app_state.db, username.to_string()).await;

    // check if user exists
    if let Ok(user) = user {
        // verify password
        let does_password_match = argon2::verify_encoded(
            &user.user_password.trim(),
            params.password.clone().as_bytes(),
        )
        .unwrap();

        // if password does not match
        if !does_password_match {
            ctx.insert("error", "Invalid login");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");

            // render signin form again
            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            ctx.insert("name", &params.username);
            ctx.insert("title", &"Signin confirmation!".to_owned());
            ctx.insert(
                "message",
                &"You have successfully logged in to EzyTutor!".to_owned(),
            );

            s = tmpl
                .render("user.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        }
    } else {
        ctx.insert("error", "User id not found");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");

        s = tmpl
            .render("signin.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
