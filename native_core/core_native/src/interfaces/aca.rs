use async_trait::async_trait;
use crate::ontology::{DataEidos, CognitiveFlow};

pub trait AcaLayer { fn layer_name(&self) -> &'static str; }

#[async_trait]
pub trait PerceptionLayer: AcaLayer {
    async fn process_stimulus(&self, flow: &mut CognitiveFlow);
}

#[async_trait]
pub trait KnowledgeLayer: AcaLayer {
    async fn represent(&self, flow: &mut CognitiveFlow);
    async fn retrieve(&self, flow: &mut CognitiveFlow);
}

// Các tầng khác hiện tại để trống
pub trait CognitionLayer: AcaLayer {}
pub trait ValueLayer: AcaLayer {}
pub trait ReflectionLayer: AcaLayer {}
pub trait ManifestationLayer: AcaLayer {}
