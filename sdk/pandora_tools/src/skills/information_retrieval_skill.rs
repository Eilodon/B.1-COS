use dashmap::DashMap;
use hnsw_rs::prelude::*;
use lancedb::Connection;
use serde_json::Value as CognitiveInput;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Default, Clone)]
pub struct Confidence {
    pub score: f32,
    pub epistemic_uncertainty: f32,
    pub aleatoric_uncertainty: f32,
}

#[derive(Debug, Default, Clone)]
pub struct CognitiveOutput {
    pub content: String,
    pub confidence: Confidence,
    pub reasoning_trace: Vec<String>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Error)]
pub enum RetrievalError {
    #[error("Lỗi kết nối LanceDB: {0}")]
    LanceDBConnection(String),
    #[error("Lỗi thực thi truy vấn LanceDB: {0}")]
    LanceDBQuery(String),
    #[error("Không tìm thấy document với ID: {0}")]
    DocumentNotFound(String),
    #[error("Input không hợp lệ cho việc tìm kiếm: {0}")]
    InvalidInput(String),
    #[error("Lỗi embedding: {0}")]
    EmbeddingError(String),
}

/// Lưu trữ các sự thật có cấu trúc: (Subject, Vec<(Predicate, Object)>)
pub type KnowledgeGraph = DashMap<String, Vec<(String, String)>>;

/// Đại diện cho một tài liệu trong bộ nhớ
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Engine tìm kiếm lũy tiến, kết hợp cache, vector DB và knowledge graph.
#[allow(dead_code)]
pub struct ProgressiveSemanticEngine {
    /// Cache HNSW cực nhanh trong bộ nhớ cho các truy vấn/kết quả phổ biến.
    hnsw_cache: Option<Hnsw<'static, f32, DistCosine>>,

    /// Map từ index của HNSW sang Document ID thực tế.
    cache_id_map: HashMap<usize, String>,

    /// Lõi lưu trữ chính, hỗ trợ hybrid search.
    lance_db_conn: Connection,

    /// Tên bảng trong LanceDB.
    lance_table_name: String,

    /// Kho tri thức có cấu trúc, an toàn luồng.
    knowledge_graph: KnowledgeGraph,

    /// Kích thước của vector embedding.
    embedding_dim: usize,

    /// Bộ nhớ trong (mock) để phục vụ test nhanh
    inmem_docs: Vec<Document>,
}

impl ProgressiveSemanticEngine {
    /// Khởi tạo một engine tìm kiếm mới.
    pub async fn new(
        lance_db_path: &str,
        table_name: &str,
        embedding_dim: usize,
    ) -> Result<Self, RetrievalError> {
        let hnsw_cache = None;

        let conn = lancedb::connect(lance_db_path)
            .execute()
            .await
            .map_err(|e| RetrievalError::LanceDBConnection(e.to_string()))?;

        // TODO: Logic để tạo bảng LanceDB nếu chưa tồn tại.

        Ok(Self {
            hnsw_cache,
            cache_id_map: HashMap::new(),
            lance_db_conn: conn,
            lance_table_name: table_name.to_string(),
            knowledge_graph: DashMap::new(),
            embedding_dim,
            inmem_docs: Vec::new(),
        })
    }

    /// Thêm một tài liệu mới vào hệ thống.
    pub async fn add_document(&mut self, doc: Document) -> Result<(), RetrievalError> {
        // TODO: ghi vào LanceDB; hiện tại thêm vào in-memory để test
        self.inmem_docs.push(doc);
        Ok(())
    }

    /// Tìm concept theo text (stub, sẽ được hiện thực sau)
    pub async fn search_by_text(
        &self,
        text: &str,
        top_k: usize,
    ) -> Result<Vec<Document>, RetrievalError> {
        let txt = text.to_lowercase();
        let mut matches: Vec<Document> = self
            .inmem_docs
            .iter()
            .filter(|d| d.content.to_lowercase().contains(&txt) || d.id.to_lowercase() == txt)
            .cloned()
            .collect();
        matches.truncate(top_k);
        Ok(matches)
    }

    /// Tìm concept theo vector (stub, sẽ được hiện thực sau)
    pub async fn search_by_vector(
        &self,
        vector: &[f32],
        top_k: usize,
    ) -> Result<Vec<Document>, RetrievalError> {
        // Cosine similarity với in-memory docs
        fn cosine(a: &[f32], b: &[f32]) -> f32 {
            let mut dot = 0.0f32;
            let mut na = 0.0f32;
            let mut nb = 0.0f32;
            for (x, y) in a.iter().zip(b.iter()) {
                dot += x * y;
                na += x * x;
                nb += y * y;
            }
            let denom = na.sqrt() * nb.sqrt();
            if denom > 0.0 {
                dot / denom
            } else {
                0.0
            }
        }
        let mut scored: Vec<(f32, &Document)> = self
            .inmem_docs
            .iter()
            .filter(|d| d.embedding.len() == vector.len())
            .map(|d| (cosine(vector, &d.embedding), d))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scored
            .into_iter()
            .take(top_k)
            .map(|(_, d)| d.clone())
            .collect())
    }

    /// Thực hiện tìm kiếm lũy tiến.
    pub async fn search(&self, _input: &CognitiveInput) -> Result<CognitiveOutput, RetrievalError> {
        // TODO: Hiện thực hóa logic tìm kiếm lũy tiến:
        // 1. Lấy embedding cho query.
        // 2. Tra cứu cache HNSW.
        // 3. Nếu cache miss, truy vấn LanceDB và Knowledge Graph.
        // 4. Tổng hợp, xếp hạng lại kết quả và tính toán điểm tự tin.
        // 5. Cập nhật cache.
        // 6. Trả về CognitiveOutput.

        // Đoạn code giả để biên dịch thành công
        Ok(CognitiveOutput {
            content: "Kết quả tìm kiếm giả".to_string(),
            confidence: Confidence {
                score: 1.0,
                epistemic_uncertainty: 0.0,
                aleatoric_uncertainty: 0.0,
            },
            reasoning_trace: vec!["Đã trả về kết quả giả.".to_string()],
            ..Default::default()
        })
    }
}
