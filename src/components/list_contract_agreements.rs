use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_agreement::ContractAgreement;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListContractAgreements() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListContractAgreementsInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListContractAgreementsInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let contract_agreement_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.contract_agreements().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let contract_agreement_list = &(*contract_agreement_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Contract Signing Date" index={Columns::ContractSigningDate} />
      <TableColumn<Columns> label="Consumer ID" index={Columns::ConsumerId} />
      <TableColumn<Columns> label="Provider ID" index={Columns::ProviderId} />
      <TableColumn<Columns> label="Asset ID" index={Columns::AssetId} />
      <TableColumn<Columns> label="Policy ID" index={Columns::PolicyID} />
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

  let rows = contract_agreement_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|contract_agreement| ContractAgreementRenderer(contract_agreement.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractAgreementRenderer>>>
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
  ContractSigningDate,
  ConsumerId,
  ProviderId,
  AssetId,
  PolicyID,
}

#[derive(Clone, Debug)]
struct ContractAgreementRenderer(ContractAgreement);

impl ContractAgreementRenderer {}

impl TableEntryRenderer<Columns> for ContractAgreementRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    let policy_id = self
      .0
      .policy()
      .id()
      .map(|policy_id| policy_id.to_string())
      .unwrap();

    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::ContractSigningDate => html!(self.0.contract_signing_date().to_string()),
      Columns::ConsumerId => html!(self.0.consumer_id().to_string()),
      Columns::ProviderId => html! {self.0.provider_id().to_string()},
      Columns::AssetId => html!(self.0.asset_id().to_string()),
      Columns::PolicyID => html!(policy_id),
    }
    .into()
  }
}
