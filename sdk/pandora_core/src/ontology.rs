use fnv::FnvHashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataEidos {
    pub active_indices: FnvHashSet<u32>,
    pub dimensionality: u32,
}

#[derive(Debug, Clone, Default)]
pub struct CognitiveFlow {
    pub raw_stimulus: Option<Vec<u8>>,
    pub features: Option<HashMap<String, f32>>,
    pub eidos: Option<DataEidos>,
    pub related_eidos: Option<Vec<DataEidos>>,
}
