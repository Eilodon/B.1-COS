#[cfg(test)]
mod tests {
    use crate::skandha_implementations::skandha_factory::*;
    use crate::skandha_implementations::advanced_skandhas::*;
    use crate::interfaces::skandhas::*;
    use crate::ontology::EpistemologicalFlow;

    #[tokio::test]
    async fn test_advanced_rupa_skandha() {
        let skandha = AdvancedRupaSkandha::new(true, true);
        let event = b"test event with metadata".to_vec();
        
        let flow = skandha.process_event(event).await;
        
        assert!(flow.rupa.is_some());
        assert_eq!(flow.rupa.as_ref().unwrap().as_ref(), b"test event with metadata");
    }

    #[tokio::test]
    async fn test_advanced_vedana_skandha() {
        let skandha = AdvancedVedanaSkandha::new(0.5, true);
        let mut flow = EpistemologicalFlow {
            rupa: Some(b"success operation completed".to_vec().into()),
            ..Default::default()
        };
        
        skandha.feel(&mut flow).await;
        
        assert!(flow.vedana.is_some());
        match flow.vedana.as_ref().unwrap() {
            crate::ontology::Vedana::Pleasant { karma_weight } => {
                assert!(*karma_weight > 0.0);
            }
            _ => panic!("Expected Pleasant feeling"),
        }
    }

    #[tokio::test]
    async fn test_advanced_sanna_skandha() {
        let skandha = AdvancedSannaSkandha::new(0.3, true);
        let mut flow = EpistemologicalFlow {
            rupa: Some(b"complex pattern analysis".to_vec().into()),
            ..Default::default()
        };
        
        skandha.perceive(&mut flow).await;
        
        assert!(flow.sanna.is_some());
        assert!(flow.related_eidos.is_some());
        assert!(!flow.related_eidos.as_ref().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_advanced_sankhara_skandha() {
        let skandha = AdvancedSankharaSkandha::new(0.4, true);
        let mut flow = EpistemologicalFlow {
            rupa: Some(b"critical error detected".to_vec().into()),
            vedana: Some(crate::ontology::Vedana::Unpleasant { karma_weight: -2.0 }),
            sanna: Some(crate::ontology::DataEidos {
                active_indices: [1u32, 2u32, 3u32].iter().cloned().collect(),
                dimensionality: 2048,
            }),
            ..Default::default()
        };
        
        skandha.form_intent(&mut flow).await;
        
        assert!(flow.sankhara.is_some());
        let intent = flow.sankhara.as_ref().unwrap();
        assert!(intent.contains("CORRECTIVE") || intent.contains("HIGH_PRIORITY"));
    }

    #[tokio::test]
    async fn test_advanced_vinnana_skandha() {
        let skandha = AdvancedVinnanaSkandha::new(0.5, true);
        let flow = EpistemologicalFlow {
            rupa: Some(b"important event".to_vec().into()),
            vedana: Some(crate::ontology::Vedana::Pleasant { karma_weight: 1.5 }),
            sanna: Some(crate::ontology::DataEidos {
                active_indices: [1u32, 2u32, 3u32, 4u32, 5u32].iter().cloned().collect(),
                dimensionality: 2048,
            }),
            related_eidos: Some(vec![
                crate::ontology::DataEidos {
                    active_indices: [6u32, 7u32].iter().cloned().collect(),
                    dimensionality: 2048,
                }
            ].into()),
            sankhara: Some("TEST_INTENT".to_string().into()),
        };
        
        let result = skandha.synthesize(&flow).await;
        
        assert!(result.is_some());
        let result_bytes = result.unwrap();
        let event = String::from_utf8_lossy(&result_bytes);
        assert!(event.contains("AdvancedConsciousness"));
        assert!(event.contains("SynthesisScore"));
    }

    #[tokio::test]
    async fn test_skandha_factory_basic() {
        let (rupa, vedana, sanna, sankhara, vinnana) = SkandhaFactory::create_basic_skandhas();
        
        assert_eq!(rupa.name(), "Basic Rupa (Form)");
        assert_eq!(vedana.name(), "Basic Vedana (Feeling)");
        assert_eq!(sanna.name(), "Basic Sanna (Perception)");
        assert_eq!(sankhara.name(), "Basic Sankhara (Formations)");
        assert_eq!(vinnana.name(), "Basic Vinnana (Consciousness)");
    }

    #[tokio::test]
    async fn test_skandha_factory_advanced() {
        let (rupa, vedana, sanna, sankhara, vinnana) = SkandhaFactory::create_advanced_skandhas();
        
        assert_eq!(rupa.name(), "Advanced Rupa (Form)");
        assert_eq!(vedana.name(), "Advanced Vedana (Feeling)");
        assert_eq!(sanna.name(), "Advanced Sanna (Perception)");
        assert_eq!(sankhara.name(), "Advanced Sankhara (Formations)");
        assert_eq!(vinnana.name(), "Advanced Vinnana (Consciousness)");
    }

    #[tokio::test]
    async fn test_skandha_factory_presets() {
        for preset in SkandhaPreset::all() {
            let (rupa, vedana, sanna, sankhara, vinnana) = SkandhaFactory::create_preset_processor(preset);
            
            // Test rằng tất cả skandhas đều có tên hợp lệ
            assert!(!rupa.name().is_empty());
            assert!(!vedana.name().is_empty());
            assert!(!sanna.name().is_empty());
            assert!(!sankhara.name().is_empty());
            assert!(!vinnana.name().is_empty());
            
            // Test description
            assert!(!preset.description().is_empty());
        }
    }

    #[tokio::test]
    async fn test_skandha_factory_custom() {
        let (rupa, vedana, sanna, sankhara, vinnana) = SkandhaFactory::create_custom_advanced_skandhas(
            (true, true),   // rupa: metadata + timestamp
            (0.3, true),    // vedana: low threshold + context
            (0.2, true),    // sanna: low threshold + semantic
            (0.1, true),    // sankhara: very low threshold + priority
            (0.4, true),    // vinnana: medium threshold + metacognition
        );
        
        assert_eq!(rupa.name(), "Advanced Rupa (Form)");
        assert_eq!(vedana.name(), "Advanced Vedana (Feeling)");
        assert_eq!(sanna.name(), "Advanced Sanna (Perception)");
        assert_eq!(sankhara.name(), "Advanced Sankhara (Formations)");
        assert_eq!(vinnana.name(), "Advanced Vinnana (Consciousness)");
    }

    #[tokio::test]
    async fn test_skandha_preset_descriptions() {
        assert!(SkandhaPreset::Basic.description().contains("Basic"));
        assert!(SkandhaPreset::Advanced.description().contains("Advanced"));
        assert!(SkandhaPreset::HighPerformance.description().contains("Performance"));
        assert!(SkandhaPreset::Debug.description().contains("Debug"));
        assert!(SkandhaPreset::Minimal.description().contains("Minimal"));
    }

    #[tokio::test]
    async fn test_advanced_skandha_integration() {
        // Test toàn bộ pipeline với Advanced Skandhas
        let (rupa, vedana, sanna, sankhara, vinnana) = SkandhaFactory::create_advanced_skandhas();
        
        let event = b"critical system error: database connection failed".to_vec();
        
        // 1. Rupa: Process event
        let mut flow = rupa.process_event(event).await;
        
        // 2. Vedana: Feel
        vedana.feel(&mut flow).await;
        assert!(flow.vedana.is_some());
        
        // 3. Sanna: Perceive
        sanna.perceive(&mut flow).await;
        assert!(flow.sanna.is_some());
        assert!(flow.related_eidos.is_some());
        
        // 4. Sankhara: Form intent
        sankhara.form_intent(&mut flow).await;
        assert!(flow.sankhara.is_some());
        
        // 5. Vinnana: Synthesize
        let result = vinnana.synthesize(&flow).await;
        assert!(result.is_some());
        
        let result_bytes = result.unwrap();
        let synthesized = String::from_utf8_lossy(&result_bytes);
        assert!(synthesized.contains("AdvancedConsciousness"));
    }
}
