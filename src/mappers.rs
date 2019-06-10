pub trait FromModel<Model> {
    fn from_model(m: Model) -> Self;
}

pub trait ToModel<'a, Model> {
    fn to_model(&'a self) -> Model;
}