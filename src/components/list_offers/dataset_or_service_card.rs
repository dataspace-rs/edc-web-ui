use edc_connector_client::types::catalog::DatasetOrService;
use patternfly_yew::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub dataset_or_service: DatasetOrService,
  #[prop_or_default]
  pub on_offer_click: Option<Callback<DatasetOrService>>,
}

#[styled_component]
pub fn DatasetOrServiceCard(props: &Props) -> Html {
  let thumbnails_class = css!(
    border-top-right-radius: var(--pf-v5-c-card--m-rounded--BorderRadius);
    border-top-left-radius: var(--pf-v5-c-card--m-rounded--BorderRadius);
    padding-block-end: var(--pf-v5-c-card--child--PaddingBottom);
    height: 180px;
    object-fit: cover;
  );

  let provider_logo_class = css!(
    width: 50px;
    height: 50px;
    object-fit: contain;
  );

  let version_class = css!(
    position: absolute;
    top: 5px;
    right: 5px;
  );

  let keyword_class = css!(
    .pf-v5-c-chip-group__list {
      overflow: hidden;
      height: 26px;
    }
  );

  let offer_button_class = css!(
    text-align: right;
  );

  let dataset_or_service = props.dataset_or_service.clone();

  let offer_button = if let Some(offer) = props.on_offer_click.clone() {
    html!(
      <Button
        variant={ButtonVariant::Primary}
        icon={Icon::AngleRight}
        onclick={move |_| {offer.emit(dataset_or_service.clone())}}
        >
        {"Select"}
      </Button>
    )
  } else {
    html!()
  };

  let common_properties = match &props.dataset_or_service {
    DatasetOrService::Dataset(dataset) => dataset.common_properties().clone(),
    DatasetOrService::Service(service) => service.common_properties().clone(),
  };

  let title = common_properties
    .title
    .map(|title| title.to_string())
    .unwrap_or_default();

  let version = common_properties
    .version
    .clone()
    .map(|version| version.to_string())
    .unwrap_or_default();

  let comment = common_properties.comment.clone().unwrap_or_default();

  let thumbnail = common_properties
    .thumbnail
    .clone()
    .map(|thumbnail| thumbnail.resource().to_string())
    .map(|thumbnail| html! {<img src={thumbnail} class={thumbnails_class.clone()} />});

  let provider_logo = common_properties
    .creator
    .clone()
    .and_then(|creator| creator.thumbnail().clone())
    .map(|thumbnail| thumbnail.resource().to_string())
    .map(|thumbnail| html! {<img src={thumbnail} class={provider_logo_class.clone()} />});

  let keywords = common_properties
    .keywords
    .iter()
    .map(|keyword| html_nested! {<Chip text={keyword.clone()} />});

  html!(
    <Card rounded=true full_height=true>
      {thumbnail}
      <div class={version_class.clone()}>
        <Badge read=true>
          {"v"}{version}
        </Badge>
      </div>
      <CardTitle>
        <div>
          {provider_logo}
        </div>
        {title}
      </CardTitle>
      <CardBody>
        <Stack gutter=true>
          <StackItem fill=true>
            {comment}
          </StackItem>
          <StackItem>
            <div class={offer_button_class.clone()}>
              {offer_button.clone()}
            </div>
          </StackItem>
        </Stack>
      </CardBody>
      <CardFooter>
        <slot class={keyword_class.clone()}>
          <ChipGroup>
            {for keywords}
          </ChipGroup>
        </slot>
      </CardFooter>
    </Card>
  )
}
