use axum::{
    extract::{Path, Query},
    response::Response,
};
use jwst_core::{History, HistoryOptions};
use utoipa::IntoParams;

use super::*;

/// Block History Options
#[derive(Deserialize, IntoParams)]
pub struct BlockHistoryQuery {
    /// client id, is give 0 or empty then return all clients histories
    client: Option<u64>,
    /// skip count, available when client is set
    skip: Option<usize>,
    /// limit count, available when client is set
    limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockHistory {
    pub workspace_id: String,
    pub block_id: String,
    pub parent: Vec<String>,
    pub content: String,
}

impl From<(&str, &History)> for BlockHistory {
    fn from((workspace_id, history): (&str, &History)) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            block_id: history.id.clone(),
            parent: history.parent.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            content: history.content.clone(),
        }
    }
}

/// Get the history generated by a specific `Client ID` of the `Workspace`
///
/// If client id set to 0, return all history of the `Workspace`.
#[utoipa::path(
    get,
    tag = "Workspace",
    context_path = "/api/block",
    path = "/{workspace}/history",
    params(
        ("workspace", description = "workspace id"),
        BlockHistoryQuery,
    ),
    responses(
        (status = 200, description = "Get workspace history", body = [History]),
        (status = 400, description = "Client id invalid"),
        (status = 500, description = "Failed to get workspace history")
    )
)]
pub async fn history_workspace(
    Extension(context): Extension<Arc<Context>>,
    Path(ws_id): Path<String>,
    query: Query<BlockHistoryQuery>,
) -> Response {
    if let Ok(workspace) = context.get_workspace(&ws_id).await {
        Json(
            workspace
                .history(HistoryOptions {
                    client: query.client,
                    skip: query.skip,
                    limit: query.limit,
                })
                .into_iter()
                .map(|h| (ws_id.as_str(), &h).into())
                .collect::<Vec<BlockHistory>>(),
        )
        .into_response()
    } else {
        (StatusCode::NOT_FOUND, format!("Workspace({ws_id:?}) not found")).into_response()
    }
}
