use crate::components::{CreatePolicy, ListPolicies};
use crate::contexts::use_edc_connector_context;
use crate::models::PolicyDefinitionItem;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[component]
pub fn PolicyPage() -> Html {
  let refresh = use_state(|| 0usize);
  let backdropper = use_backdrop();
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let onoffset = use_callback(
    (refresh.clone(), offset.setter()),
    |offset, (refresh, offset_setter)| {
      offset_setter.set(offset);
      refresh.set(**refresh + 1);
    },
  );

  let onlimit = use_callback(
    (refresh.clone(), limit.setter()),
    |limit, (refresh, limit_setter)| {
      limit_setter.set(limit);
      refresh.set(**refresh + 1);
    },
  );

  let edc_connector_context = use_edc_connector_context();

  let ondelete = use_callback(
    (refresh.clone(), edc_connector_context),
    |policy_id: String, (refresh, edc_connector_context)| {
      let refresh = refresh.clone();
      let edc_connector_context = edc_connector_context.clone();
      let policy_id = policy_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client.policies().delete(&policy_id).await;
          log::warn!("Deleted policy {} - {}", policy_id, *refresh + 1);
          refresh.set(*refresh + 1);
        }
      });
    },
  );

  let on_create = use_callback(
    (backdropper.clone(), refresh.clone()),
    |_, (backdropper, refresh)| {
      if let Some(backdropper) = backdropper {
        backdropper.close();
      }

      refresh.set(**refresh + 1);
    },
  );

  let onclick = use_callback((backdropper, on_create), |_, (backdropper, on_create)| {
    if let Some(backdropper) = backdropper {
      backdropper.open(html!(
        <Bullseye>
          <Modal variant={ModalVariant::Medium} title="Create an Policy">
            <CreatePolicy {on_create} />
          </Modal>
        </Bullseye>
      ))
    }
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Policies" }</Title>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>{ "Add" }</Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Card>
          <CardBody>
            <Suspense>
              <PolicyPageInner
                offset={*offset}
                limit={*limit}
                {onoffset}
                {onlimit}
                {ondelete}
                force_refresh={*refresh}
              />
            </Suspense>
          </CardBody>
        </Card>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn PolicyPageInner(props: &PolicyPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let policy_definition_items = use_future_with(
    (
      edc_connector_context,
      props.limit,
      props.offset,
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, _) = (*parameters).clone();

      let query = Query::builder()
        .limit(limit as u32)
        .offset(offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .policies()
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(PolicyDefinitionItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let policy_definition_items = (*policy_definition_items).clone();

  Ok(html!(
    <ListPolicies
      policy_definition_items={policy_definition_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      ondelete={props.ondelete.clone()}
    />
  ))
}
