use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_definition::ContractDefinition;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListContractDefinitions() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListContractDefinitionsInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListContractDefinitionsInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let contract_definition_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.contract_definitions().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let contract_definition_list = &(*contract_definition_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Access Policy ID" index={Columns::AccessPolicyId} />
      <TableColumn<Columns> label="Contract Policy ID" index={Columns::ContractPolicyId} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
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

  let rows = contract_definition_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|contract_definition| ContractDefinitionRenderer(contract_definition.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractDefinitionRenderer>>>
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
  Actions,
}

#[derive(Clone, Debug)]
struct ContractDefinitionRenderer(ContractDefinition);

impl ContractDefinitionRenderer {}

impl TableEntryRenderer<Columns> for ContractDefinitionRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::AccessPolicyId => html!(self.0.access_policy_id()),
      Columns::ContractPolicyId => html!(self.0.contract_policy_id()),
      Columns::Actions => {
        let contract_definition_id = self.0.id().to_string();

        html!(
          <DeleteContractDefinition {contract_definition_id} />
        )
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub contract_definition_id: String,
}

#[function_component]
pub fn DeleteContractDefinition(props: &Props) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let onclick = use_callback(
    (edc_connector_context, props.contract_definition_id.clone()),
    move |_, (edc_connector_context, contract_definition_id)| {
      let edc_connector_context = edc_connector_context.clone();
      let contract_definition_id = contract_definition_id.to_string();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .contract_definitions()
            .delete(&contract_definition_id)
            .await;
        }
      });
    },
  );

  html!(
    <Button
      variant={ButtonVariant::Danger}
      icon={Icon::Trash}
      {onclick}
      >
      {"Delete"}
    </Button>
  )
}
