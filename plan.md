For a developer website intended to last for years rather than months, I would avoid treating it as a simple "portfolio." Instead, treat it as a personal publishing platform—a Rust application that happens to expose a portfolio, technical blog, digital library, and knowledge base from a single system.

For dev.irshad.org.in, the architecture should optimize for:

Pure Rust
High performance
Long-term maintainability
Minimal dependencies
Type safety
Server-side rendering
SEO
Future extensibility
Self-hosting or cloud deployment
High-Level Architecture
                         Internet
                             │
                     HTTPS (TLS)
                             │
                    Reverse Proxy
               (Caddy / Nginx / Traefik)
                             │
                ┌──────────────────────┐
                │      Axum Server     │
                └──────────────────────┘
                             │
         ┌───────────────────┼────────────────────┐
         │                   │                    │
         ▼                   ▼                    ▼
   Leptos SSR          REST / JSON API      Static Assets
         │                   │                    │
         └──────────────┬────┴──────────────┐
                        │
                Application Layer
                        │
        ┌───────────────┼────────────────┐
        ▼               ▼                ▼
    Portfolio       Article Engine    Book Engine
        │               │                │
        └───────────────┼────────────────┘
                        ▼
                 Content Service
                        │
        ┌───────────────┼─────────────────────┐
        ▼               ▼                     ▼
      Neon DB       File Storage        Search Index
                        │
                 Markdown / Assets
Technology Stack
Layer Technology
Language Rust 2024
Frontend Leptos SSR
Backend Axum
Runtime Tokio
Routing Axum Router
Serialization Postcard
Configuration RON
Database Neon PostgreSQL
ORM SQLx
Authentication JWT + Secure Cookies
Templates Leptos Components
Markdown pulldown-cmark
Search Tantivy
Logging tracing
Errors thiserror
Async Tokio
Images image crate
Password Argon2
Compression tower-http
Caching moka
Folder Structure
dev-site/

├── app/
│
├── frontend/
│
├── backend/
│
├── shared/
│
├── configuration/
│
├── database/
│
├── migrations/
│
├── assets/
│
├── books/
│
├── articles/
│
├── public/
│
├── search/
│
├── themes/
│
├── tests/
│
├── scripts/
│
└── docs/
Workspace
workspace

├── app
│
├── ui
│
├── api
│
├── domain
│
├── database
│
├── config
│
├── search
│
├── auth
│
├── markdown
│
├── books
│
├── portfolio
│
├── shared
│
└── cli

Each crate has a single responsibility.

Pages
/

About

Projects

Portfolio

Experience

Articles

Books

Notes

Talks

Timeline

Now

Uses

Bookmarks

Contact

Search

RSS

Atom

Sitemap

404
Portfolio Module

Contains

Projects

Open Source

Experience

Education

Skills

Achievements

Resume

GitHub Stats

Roadmap

Architecture Showcase

Projects can have

Title

Description

Gallery

Architecture

Tech Stack

Repository

Live Demo

Blog

Screenshots

Metrics

Lessons Learned
Articles Module

Each article stored as

RON metadata

Markdown body

Assets

Example

articles/

rust/

    ownership/

        meta.ron

        article.md

        image.webp

        diagram.svg

Metadata

(
    title:"Rust Ownership",

    slug:"rust-ownership",

    description:"...",

    published:true,

    tags:[
        "rust",
        "ownership",
    ],

    created:"2026-07-20",

    updated:"2026-07-22",
)
Books Module

Each book

Book

Volumes

Chapters

Sections

Images

Examples

Downloads

Books stored

books/

rust/

    book.ron

    chapter1.md

    chapter2.md

    images/
Configuration

Everything configurable using RON.

Example

config/

app.ron

database.ron

theme.ron

search.ron

cache.ron

security.ron
Content Pipeline
Markdown

↓

Frontmatter

↓

RON parser

↓

Markdown parser

↓

Syntax highlighting

↓

Leptos component tree

↓

SSR HTML

↓

Hydration
Database

Only dynamic data goes into Neon.

Tables

users

sessions

articles

books

tags

categories

comments

likes

views

newsletter

contact_messages

analytics

drafts
Static Content

Never stored in database.

Instead

Markdown

Images

PDF

EPUB

SVG

WebP

RON

Git becomes CMS.

Search Engine

Use Tantivy.

Index

Article title

Book title

Body

Tags

Descriptions

Projects

Skills

Incremental indexing.

Asset Pipeline

Original image

↓

Generate

AVIF

WebP

Thumbnail

Mobile

Desktop

Retina
Caching
Memory

↓

Moka

↓

HTTP Cache

↓

CDN

↓

Browser
Authentication
Admin Login

↓

Argon2 password

↓

JWT

↓

Secure Cookie

↓

RBAC

Roles

Admin

Editor

Reader
Admin Dashboard

Create

Edit

Delete

Preview

Publish

Schedule

Books

Articles

Projects

Analytics

Search Index

Settings

Uploads

APIs
GET /articles

GET /books

GET /projects

GET /search

GET /rss

GET /feed

GET /sitemap

POST /login

POST /contact
SEO

Generate automatically

OpenGraph

Twitter Card

JSON-LD

RSS

Atom

robots.txt

sitemap.xml

canonical
Performance

SSR

Streaming SSR

Hydration Islands

Lazy Loading

Image Optimization

Compression

HTTP/2

HTTP/3

ETag

Cache-Control

Serialization Strategy

Use the appropriate format for each purpose rather than forcing one format everywhere:

Use case Format Reason
Configuration RON Human-readable and version-controlled
Content metadata RON Easy to edit manually
Internal Rust state Postcard Compact binary serialization
Network APIs RON/JSON Browser interoperability
Session/cache blobs Postcard Small and fast
Database Native PostgreSQL types Efficient querying and indexing
About "zero-copy" with Postcard

postcard itself is not a zero-copy serialization format. It is a compact binary serializer built on serde. You can reduce allocations by deserializing borrowed data (e.g. &str or &[u8]) where supported, but true zero-copy depends on the data layout and the serialized format.

For true zero-copy Rust data exchange, consider formats such as:

rkyv (archived, zero-copy deserialization within Rust)
zerocopy (safe zero-copy byte reinterpretation for suitable types)
bytemuck (POD-style casting where applicable)

A practical design is:

RON for editable configuration and content metadata.
Postcard for compact internal messages, cache entries, and IPC.
rkyv/zerocopy only in performance-critical paths where profiling shows a measurable benefit.
Future Features
Multi-language

Versioned articles

Book reader

Reading progress

Bookmarks

Code playground

Architecture diagrams

Mermaid support

PlantUML

PDF generation

EPUB generation

Newsletter

Comments

Series

Tag explorer

Knowledge graph

AI semantic search

RSS aggregation

Dark/Light themes

PWA

Offline reading

WebAuthn

OpenID Connect

Federation (ActivityPub)

Public API

Analytics dashboard
Recommended Layered Architecture
Presentation
    ├── Leptos Components
    ├── SSR Rendering
    └── API Handlers

Application
    ├── Portfolio Service
    ├── Article Service
    ├── Book Service
    ├── Search Service
    ├── Authentication Service
    └── Asset Service

Domain
    ├── Entities
    ├── Value Objects
    ├── Aggregates
    ├── Repository Traits
    └── Business Rules

Infrastructure
    ├── SQLx + Neon PostgreSQL
    ├── Tantivy
    ├── Filesystem Storage
    ├── RON Configuration
    ├── Postcard Serialization
    └── Observability (tracing, metrics)

This separation keeps business logic independent of Leptos, Axum, SQLx, and Neon, making the application easier to test, evolve, and potentially migrate to different frameworks or storage backends in the future.
