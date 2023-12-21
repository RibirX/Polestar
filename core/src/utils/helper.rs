use crate::{model::Bot, service::service_provider::ServiceModel};

pub fn has_official_server(bots: &[Bot]) -> bool {
  bots.iter().any(|b| b.sp() == &ServiceModel::Polestar)
}
