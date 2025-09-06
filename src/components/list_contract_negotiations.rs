use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_negotiation::{
  ContractNegotiation, ContractNegotiationKind, ContractNegotiationState,
};
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListContractNegotiations() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListContractNegotiationsInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListContractNegotiationsInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let contract_negotiation_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.contract_negotiations().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let contract_negotiation_list = &(*contract_negotiation_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Access Policy ID" index={Columns::AccessPolicyId} />
      <TableColumn<Columns> label="Contract Policy ID" index={Columns::ContractPolicyId} />
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Contract Agreement ID" index={Columns::ContractAgreementId} />
      <TableColumn<Columns> label="Counter Party ID" index={Columns::CounterPartyId} />
      <TableColumn<Columns> label="Counter Party Address" index={Columns::CounterPartyAddress} />
      <TableColumn<Columns> label="Protocol" index={Columns::Protocol} />
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

  let rows = contract_negotiation_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|contract_negotiation| ContractNegotiationRenderer(contract_negotiation.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractNegotiationRenderer>>>
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
  AccessPolicyId,
  ContractPolicyId,
  State,
  ContractAgreementId,
  CounterPartyId,
  CounterPartyAddress,
  Protocol,
  Kind,
}

#[derive(Clone, Debug)]
struct ContractNegotiationRenderer(ContractNegotiation);

impl TableEntryRenderer<Columns> for ContractNegotiationRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    let contract_agreement_id = self
      .0
      .contract_agreement_id()
      .map(|contract_agreement_id| contract_agreement_id.to_string())
      .unwrap_or_default();

    let kind = match self.0.kind() {
      ContractNegotiationKind::Consumer => "Consumer",
      ContractNegotiationKind::Provider => "Provider",
    };

    let state = match self.0.state() {
      ContractNegotiationState::Initial => "Initial".to_string(),
      ContractNegotiationState::Requesting => "Requesting".to_string(),
      ContractNegotiationState::Requested => "Requested".to_string(),
      ContractNegotiationState::Offering => "Offering".to_string(),
      ContractNegotiationState::Offered => "Offered".to_string(),
      ContractNegotiationState::Accepting => "Accepting".to_string(),
      ContractNegotiationState::Accepted => "Accepted".to_string(),
      ContractNegotiationState::Agreeing => "Agreeing".to_string(),
      ContractNegotiationState::Agreed => "Agreed".to_string(),
      ContractNegotiationState::Verifying => "Verifying".to_string(),
      ContractNegotiationState::Verified => "Verified".to_string(),
      ContractNegotiationState::Finalizing => "Finalizing".to_string(),
      ContractNegotiationState::Finalized => "Finalized".to_string(),
      ContractNegotiationState::Terminating => "Terminating".to_string(),
      ContractNegotiationState::Terminated => "Terminated".to_string(),
      ContractNegotiationState::Other(other) => other.to_string(),
    };

    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::AccessPolicyId => html! {},
      Columns::ContractPolicyId => html! {},
      Columns::State => html! { state },
      Columns::ContractAgreementId => html! { contract_agreement_id },
      Columns::CounterPartyId => html! { self.0.counter_party_id().clone().unwrap_or_default() },
      Columns::CounterPartyAddress => html! { self.0.counter_party_address() },
      Columns::Protocol => html! { self.0.protocol() },
      Columns::Kind => html! { kind },
    }
    .into()
  }
}
