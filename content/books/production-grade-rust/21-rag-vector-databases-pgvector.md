---
title: "RAG & Vector Databases (pgvector)"
description: "Language models suffer from hallucinations and have fixed context cut-offs. We fix this using Retrieval-Augmented Generation (RAG). By embedding our past newsletter archives into a vector database, we can dynamically search for relevant history and inject it into the AI's prompt."
order: 21
---

Language models suffer from hallucinations and have fixed context cut-offs. We fix this using **Retrieval-Augmented Generation (RAG)**. By embedding our past newsletter archives into a vector database, we can dynamically search for relevant history and inject it into the AI's prompt.

## Integrating pgvector with SQLx

Instead of standing up a separate vector database like Pinecone or Milvus, we leverage our existing Postgres database using the `pgvector` extension. This keeps our architecture radically simpler and avoids distributed transaction failures.

**`migrations/03_add_pgvector.sql`**

```sql
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE newsletter_embeddings (
    id UUID PRIMARY KEY,
    newsletter_issue_id UUID NOT NULL,
    content_chunk TEXT NOT NULL,
    embedding vector(768) NOT NULL
);

-- Create an HNSW index for lightning-fast approximate nearest neighbor search
CREATE INDEX ON newsletter_embeddings USING hnsw (embedding vector_cosine_ops);
```

## Cosine Similarity Searches

When a user asks a question via our API, we first embed their question using a fast embeddings model (like `text-embedding-3-small`). We then query Postgres to find the closest matching chunks.

**`src/rag.rs`**

```rust
use pgvector::Vector;
use sqlx::PgPool;

pub async fn find_relevant_context(
    pool: &PgPool, 
    query_embedding: Vec<f32>
) -> Result<Vec<String>, sqlx::Error> {
    let vec = Vector::from(query_embedding);
    
    let records = sqlx::query!(
        r#"
        SELECT content_chunk 
        FROM newsletter_embeddings 
        ORDER BY embedding <=> $1 
        LIMIT 5
        "#,
        vec as _
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|r| r.content_chunk).collect())
}
```

  
> [!IMPORTANT]  
> Naive chunking (e.g., splitting every 500 characters) destroys semantic meaning. We implement intelligent chunking using AST parsers or markdown awareness to keep paragraphs intact. For expert-level RAG, we combine vector search with traditional keyword search (Postgres Full Text Search) in a Hybrid Search (Reciprocal Rank Fusion), yielding vastly superior retrieval accuracy for highly technical queries.  

## Hyper-Expansion: Complete Ingestion Pipeline (PDF & HTML)

RAG is useless without data. We use `scraper` for HTML parsing, and the `text-splitter` crate to perform intelligent token-aware chunking.

**`src/ingestion/pipeline.rs`**

```rust
use text_splitter::{TextSplitter, ChunkConfig};
use tiktoken_rs::get_bpe_from_model;
use scraper::{Html, Selector};

pub async fn ingest_html_newsletter(
    pool: &PgPool,
    gemini_client: &GeminiClient,
    raw_html: &str,
    issue_id: Uuid,
) -> Result<(), anyhow::Error> {
    
    let document = Html::parse_document(raw_html);
    let selector = Selector::parse("p, h1, h2, h3, li").unwrap();
    
    let mut clean_text = String::new();
    for element in document.select(&selector) {
        clean_text.push_str(&element.text().collect::<Vec<_>>().join(" "));
        clean_text.push_str("\n");
    }

    let tokenizer = get_bpe_from_model("text-embedding-3-small").unwrap();
    let chunk_config = ChunkConfig::new(512).with_overlap(50);
    let splitter = TextSplitter::new(tokenizer).with_config(chunk_config);
    
    let chunks: Vec<&str> = splitter.chunks(&clean_text).collect();

    let embeddings = gemini_client.batch_embed(chunks.clone()).await?;

    let mut tx = pool.begin().await?;
    for (chunk_text, embedding) in chunks.iter().zip(embeddings) {
        let vec = pgvector::Vector::from(embedding);
        sqlx::query!(
            "INSERT INTO newsletter_embeddings (id, newsletter_issue_id, content_chunk, embedding)
             VALUES ($1, $2, $3, $4)",
            Uuid::new_v4(),
            issue_id,
            *chunk_text,
            vec as _
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    Ok(())
}
```