use serde::Deserialize;

pub trait Service {
  fn model(&self) -> ServiceModel;
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceModel {
  OpenAI,
  Polestar,
}

pub struct OpenAIService {}

impl Service for OpenAIService {
  fn model(&self) -> ServiceModel { ServiceModel::OpenAI }
}

pub struct PolestarService {}

impl Service for PolestarService {
  fn model(&self) -> ServiceModel { ServiceModel::Polestar }
}

pub fn get_service(model: ServiceModel) -> Box<dyn Service> {
  match model {
    ServiceModel::OpenAI => Box::new(OpenAIService {}),
    ServiceModel::Polestar => Box::new(PolestarService {}),
  }
}
