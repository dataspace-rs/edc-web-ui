use crate::components::{CreateAsset, ListAssets};
use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[component]
pub fn AssetPage() -> Html {
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
    |asset_id: String, (refresh, edc_connector_context)| {
      let refresh = refresh.clone();
      let edc_connector_context = edc_connector_context.clone();
      let asset_id = asset_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client.assets().delete(&asset_id).await;
        }
        refresh.set(*refresh + 1);
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
          <Modal variant={ModalVariant::Medium} title="Create an Asset">
            <CreateAsset {on_create} />
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
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Assets" }</Title>
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
              <AssetPageInner
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
pub struct AssetPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn AssetPageInner(props: &AssetPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let asset_items = use_future_with(
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
        // .filter("https://w3id.org/edc/v0.0.1/ns/master-catalog-company-id", "=", "424F9F7A-BBC8-4BAD-B128-C3D0A693ABBA")
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .assets()
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(AssetItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let asset_items = (*asset_items).clone();

  Ok(html!(
    <ListAssets
      asset_items={asset_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      ondelete={props.ondelete.clone()}
    />
  ))
}
