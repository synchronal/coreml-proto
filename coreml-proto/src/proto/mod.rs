// Re-export the generated protobuf modules

include!("core_ml.specification.rs");

pub mod core_ml_models {
  include!("core_ml.specification.core_ml_models.rs");
}

pub mod mil_spec {
  include!("core_ml.specification.mil_spec.rs");
}
