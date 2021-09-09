pub enum Material {
    Lambertian {},
    Metal {},
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {}
    }
}
