use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use tera::{Context, Tera};
use crate::registry::SharedRegistry;
use crate::resource::{AdminTitle, AdminPrefix};

pub async fn index(
    session: Session,
    registry: web::Data<SharedRegistry>,
    tmpl: web::Data<Tera>,
    title: web::Data<AdminTitle>,
    prefix: web::Data<AdminPrefix>,
) -> impl Responder {
    if session.get::<String>("admin_user").unwrap().is_none() {
        return HttpResponse::Found()
            .insert_header(("Location", "/admin/login"))
            .finish();
    }

    let mut ctx = Context::new();
    ctx.insert("title", &title.0);
    ctx.insert("page_title", "Dashboard");
    ctx.insert("user", &session.get::<String>("admin_user").unwrap().unwrap());
    ctx.insert("path_dashboard", &format!("/{}/", prefix.0));
    ctx.insert("path_logout", &format!("/{}/logout", prefix.0));
    
    let resources: Vec<_> = registry.all().iter().map(|r| r.info(&prefix.0)).collect();
    ctx.insert("resources", &resources);

    match tmpl.render("dashboard.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
