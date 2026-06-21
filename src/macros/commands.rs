use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn impl_entity_commands_inner(entity: &str) -> TokenStream {
    let entity_name = entity.to_lowercase();
    let entity_name_cap = entity;

    let get_all_fn = format_ident!("get_all_{}", entity_name);
    let get_fn = format_ident!("get_{}", entity_name);
    let create_fn = format_ident!("create_{}", entity_name);
    let update_fn = format_ident!("update_{}", entity_name);
    let delete_fn = format_ident!("delete_{}", entity_name);

    let entity_ident = format_ident!("{}", entity_name_cap);

    let doc_get_all = format!("Get all {} records", entity_name_cap);
    let doc_get = format!("Get a {} by ID", entity_name_cap);
    let doc_create = format!("Create a new {}", entity_name_cap);
    let doc_update = format!("Update an existing {}", entity_name_cap);
    let doc_delete = format!("Delete a {}", entity_name_cap);

    quote! {
        #[allow(non_snake_case)]
        #[tauri::command]
        #[doc = #doc_get_all]
        pub async fn #get_all_fn(
            state: tauri::State<'_, AppState>,
        ) -> crate::Result<crate::Response<Vec<#entity_ident>>> {
            let items = #entity_ident::get_all(&state.db).await
                .map_err(crate::AppError::from)?;
            Ok(crate::Response::success(items))
        }

        #[allow(non_snake_case)]
        #[tauri::command]
        #[doc = #doc_get]
        pub async fn #get_fn(
            state: tauri::State<'_, AppState>,
            id: i64,
        ) -> crate::Result<crate::Response<#entity_ident>> {
            match #entity_ident::get_by_id(&state.db, id).await {
                Ok(item) => Ok(crate::Response::success(item)),
                Err(crate::error::orm::OrmError::NotFound(_)) => {
                    Ok(crate::Response::<#entity_ident>::not_found(#entity_name_cap))
                }
                Err(e) => Err(crate::AppError::from(e)),
            }
        }

        #[allow(non_snake_case)]
        #[tauri::command]
        #[doc = #doc_create]
        pub async fn #create_fn(
            state: tauri::State<'_, AppState>,
            data: #entity_ident,
        ) -> crate::Result<crate::Response<#entity_ident>> {
            match #entity_ident::create(&state.db, data).await {
                Ok(item) => Ok(crate::Response::created(item)),
                Err(crate::error::orm::OrmError::ValidationError(msg)) => {
                    Ok(crate::Response::<#entity_ident>::validation_error(msg))
                }
                Err(crate::error::orm::OrmError::Duplicate(entity)) => {
                    Ok(crate::Response::<#entity_ident>::error(format!("{} already exists", entity)))
                }
                Err(e) => Err(crate::AppError::from(e)),
            }
        }

        #[allow(non_snake_case)]
        #[tauri::command]
        #[doc = #doc_update]
        pub async fn #update_fn(
            state: tauri::State<'_, AppState>,
            id: i64,
            data: #entity_ident,
        ) -> crate::Result<crate::Response<#entity_ident>> {
            match #entity_ident::update(&state.db, id, data).await {
                Ok(item) => Ok(crate::Response::updated(item)),
                Err(crate::error::orm::OrmError::NotFound(_)) => {
                    Ok(crate::Response::<#entity_ident>::not_found(#entity_name_cap))
                }
                Err(crate::error::orm::OrmError::ValidationError(msg)) => {
                    Ok(crate::Response::<#entity_ident>::validation_error(msg))
                }
                Err(e) => Err(crate::AppError::from(e)),
            }
        }

        #[allow(non_snake_case)]
        #[tauri::command]
        #[doc = #doc_delete]
        pub async fn #delete_fn(
            state: tauri::State<'_, AppState>,
            id: i64,
        ) -> crate::Result<crate::Response<#entity_ident>> {
            match #entity_ident::delete(&state.db, id).await {
                Ok(item) => Ok(crate::Response::deleted(item)),
                Err(crate::error::orm::OrmError::NotFound(_)) => {
                    Ok(crate::Response::<#entity_ident>::not_found(#entity_name_cap))
                }
                Err(e) => Err(crate::AppError::from(e)),
            }
        }
    }
}
