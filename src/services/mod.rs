//! Shared services module.
//!
//! This module provides shared service implementations that can be used
//! across multiple Tauri applications.

pub mod base_crud_service;

pub use base_crud_service::BaseCrudService;
