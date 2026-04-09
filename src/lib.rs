pub mod resource;
pub mod registry;
pub mod site;
pub mod types;
pub mod handlers;

pub use resource::AdminResource;
pub use registry::AdminRegistry;
pub use site::AdminSite;

use tera::Tera;
use std::sync::Arc;

/// Helper to create a Tera instance with embedded templates.
pub fn init_templates() -> Tera {
    let mut tera = Tera::default();
    
    tera.add_raw_template("base.html", include_str!("templates/base.html")).unwrap();
    tera.add_raw_template("login.html", include_str!("templates/login.html")).unwrap();
    tera.add_raw_template("dashboard.html", include_str!("templates/dashboard.html")).unwrap();
    tera.add_raw_template("list.html", include_str!("templates/list.html")).unwrap();
    tera.add_raw_template("form.html", include_str!("templates/form.html")).unwrap();
    
    tera
}

pub type AdminTemplates = Arc<Tera>;
