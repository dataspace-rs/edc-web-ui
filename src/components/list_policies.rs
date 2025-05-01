use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::policy::{PolicyDefinition, PolicyKind};
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[function_component]
pub fn ListPolicies() -> Html {
  let fallback = html!("Loading ...");

  html!(
    <Suspense {fallback}>
      <ListPoliciesInner />
    </Suspense>
  )
}

#[function_component]
pub fn ListPoliciesInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let policy_list = use_future_with(
    (edc_connector_context, *limit, *offset),
    |parameters| async move {
      let (edc_connector_context, limit, offset) = &*parameters;

      let query = Query::builder()
        .limit(*limit as u32)
        .offset(*offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client.policies().query(query).await
      } else {
        Ok(vec![])
      }
    },
  )?;

  let policy_list = &(*policy_list);

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Kind" index={Columns::Kind} />
      <TableColumn<Columns> label="Assignee" index={Columns::Assignee} />
      <TableColumn<Columns> label="Assigner" index={Columns::Assigner} />
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

  let rows = policy_list
    .as_ref()
    .unwrap()
    .iter()
    .map(|policy| PolicyRenderer(policy.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<PolicyRenderer>>>
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
  Kind,
  Assignee,
  Assigner,
}

#[derive(Clone, Debug)]
struct PolicyRenderer(PolicyDefinition);

impl TableEntryRenderer<Columns> for PolicyRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! {self.0.id().to_string()},
      Columns::Kind => {
        let kind = match self.0.policy().kind() {
          PolicyKind::Agreement => "Agreement",
          PolicyKind::Offer => "Offer",
          PolicyKind::Set => "Set",
        };

        html!({ kind })
      }
      Columns::Assignee => html!(
        self
          .0
          .policy()
          .assignee()
          .cloned()
          .unwrap_or_default()
          .to_string()
      ),
      Columns::Assigner => html!(
        self
          .0
          .policy()
          .assigner()
          .cloned()
          .unwrap_or_default()
          .to_string()
      ),
    }
    .into()
  }
}
