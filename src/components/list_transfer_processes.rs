use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::query::Query;
use edc_connector_client::types::transfer_process::{
  TransferProcess, TransferProcessKind, TransferProcessState,
};
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListTransferProcesses() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListTransferProcessesInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListTransferProcessesInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let transfer_process_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.transfer_processes().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let transfer_process_list = &(*transfer_process_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Asset ID" index={Columns::AssetId} />
      <TableColumn<Columns> label="Contract ID" index={Columns::ContractId} />
      <TableColumn<Columns> label="Transfer Type" index={Columns::TransferType} />
      <TableColumn<Columns> label="Kind" index={Columns::Kind} />
    </TableHeader<Columns>>
  };

  let limit_callback = use_callback(limit.clone(), |number, limit| limit.set(number));

  let total_entries: Option<usize> = None;

  let nav_callback = use_callback(
    (offset.clone(), *limit, total_entries),
    |page: Navigation, (offset, limit, total_entries)| {
      let o = match page {
        Navigation::First => 0,
        Navigation::Last => (total_entries.unwrap_or_default().saturating_sub(1) / limit) * limit,
        Navigation::Previous => **offset - limit,
        Navigation::Next => **offset + limit,
        Navigation::Page(n) => n * limit,
      };
      offset.set(o);
    },
  );

  let rows = transfer_process_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|transfer_process| ListTransferProcessRenderer(transfer_process.clone()))
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  let table = html!(
    <>
      <Toolbar>
        <ToolbarContent>
          <ToolbarItem r#type={ToolbarItemType::Pagination}>
            <Pagination
              offset={*offset}
              entries_per_page_choices={vec![5, 10, 25, 50, 100]}
              selected_choice={*limit}
              onlimit={&limit_callback}
              onnavigation={&nav_callback}
            />
          </ToolbarItem>
        </ToolbarContent>
      </Toolbar>
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ListTransferProcessRenderer>>>
        mode={TableMode::Compact}
        {header}
        {entries}
        />
    </>
  );

  Ok(table)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Id,
  State,
  AssetId,
  ContractId,
  TransferType,
  Kind,
}

#[derive(Clone, Debug)]
struct ListTransferProcessRenderer(TransferProcess);

impl ListTransferProcessRenderer {}

impl TableEntryRenderer<Columns> for ListTransferProcessRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    let state = match self.0.state() {
      TransferProcessState::Initial => "Initial".to_string(),
      TransferProcessState::Provisioning => "Provisioning".to_string(),
      TransferProcessState::ProvisioningRequested => "ProvisioningRequested".to_string(),
      TransferProcessState::Provisioned => "Provisioned".to_string(),
      TransferProcessState::Requesting => "Requesting".to_string(),
      TransferProcessState::Requested => "Requested".to_string(),
      TransferProcessState::Starting => "Starting".to_string(),
      TransferProcessState::Started => "Started".to_string(),
      TransferProcessState::Suspending => "Suspending".to_string(),
      TransferProcessState::Suspended => "Suspended".to_string(),
      TransferProcessState::Resuming => "Resuming".to_string(),
      TransferProcessState::Resumed => "Resumed".to_string(),
      TransferProcessState::Completing => "Completing".to_string(),
      TransferProcessState::Completed => "Completed".to_string(),
      TransferProcessState::Terminating => "Terminating".to_string(),
      TransferProcessState::Terminated => "Terminated".to_string(),
      TransferProcessState::Deprovisioning => "Deprovisioning".to_string(),
      TransferProcessState::DeprovisioningRequested => "DeprovisioningRequested".to_string(),
      TransferProcessState::Deprovisioned => "Deprovisioned".to_string(),
      TransferProcessState::Other(other) => other.to_string(),
    };

    let kind = match self.0.kind() {
      TransferProcessKind::Consumer => "Consumer",
      TransferProcessKind::Provider => "Provider",
    };

    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::State => html!(state),
      Columns::AssetId => html!(self.0.asset_id().to_string()),
      Columns::ContractId => html! {self.0.contract_id().to_string()},
      Columns::TransferType => html!(self.0.transfer_type().to_string()),
      Columns::Kind => html!(kind),
    }
    .into()
  }
}
