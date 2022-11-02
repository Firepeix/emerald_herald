use rocket::launch;

#[launch]
fn rocket() -> _ {
    color_eyre::install().expect("Não foi possivel instalar color eyre!");
    emerald_herald::install().expect("Não foi possivel instalar configurações!");
    rocket::build().mount("/", emerald_herald::routes())
}