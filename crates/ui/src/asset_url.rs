use elements_core::asset_cache;
use elements_ecs::World;
use elements_element::{Element, ElementComponent, ElementComponentExt, Hooks};
use elements_std::{
    asset_url::{select_asset, AssetUrl, AssetUrlCollection, GetAssetType, TypedAssetUrl}, Cb
};

use crate::{align_vertical, space_between_items, Align, Button, ButtonStyle, Editor, EditorOpts, FlowRow, Text, STREET};

impl<T: GetAssetType + 'static> Editor for TypedAssetUrl<T> {
    type Output = Self;

    fn editor(self, on_change: Option<Cb<dyn Fn(Self) + Sync + Send>>, _opts: EditorOpts) -> Element {
        AssetUrlEditor { value: self, on_change }.el()
    }

    fn value(&self) -> Self::Output {
        self.clone()
    }
}
#[derive(Debug, Clone)]
pub struct AssetUrlEditor<T: GetAssetType> {
    pub value: TypedAssetUrl<T>,
    pub on_change: Option<Cb<dyn Fn(TypedAssetUrl<T>) + Sync + Send>>,
}
impl<T: GetAssetType + 'static> ElementComponent for AssetUrlEditor<T> {
    fn render(self: Box<Self>, _world: &mut World, _hooks: &mut Hooks) -> Element {
        let Self { value, on_change } = *self;
        if let Some(on_change) = on_change {
            FlowRow::el([
                Text::el(value.0.to_string()),
                Button::new("\u{f74e} Browse", move |world| {
                    let on_change = on_change.clone();
                    select_asset(world.resource(asset_cache()), T::asset_type(), move |asset_url| {
                        if let Some(url) = asset_url.random() {
                            on_change(TypedAssetUrl::parse(url).unwrap());
                        }
                    });
                })
                .style(ButtonStyle::Flat)
                .el(),
            ])
            .set(align_vertical(), Align::Center)
            .set(space_between_items(), STREET)
        } else {
            Text::el(value.0.to_string())
        }
    }
}

impl<T: GetAssetType + 'static> Editor for AssetUrlCollection<T> {
    type Output = Self;

    fn editor(self, on_change: Option<Cb<dyn Fn(Self) + Sync + Send>>, _opts: EditorOpts) -> Element {
        AssetUrlCollectionEditor { value: self, on_change }.el()
    }

    fn value(&self) -> Self::Output {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AssetUrlCollectionEditor<T: GetAssetType> {
    pub value: AssetUrlCollection<T>,
    pub on_change: Option<Cb<dyn Fn(AssetUrlCollection<T>) + Sync + Send>>,
}
impl<T: GetAssetType + 'static> ElementComponent for AssetUrlCollectionEditor<T> {
    fn render(self: Box<Self>, _world: &mut World, _hooks: &mut Hooks) -> Element {
        let Self { value, on_change } = *self;
        if let Some(on_change) = on_change {
            FlowRow::el([
                Text::el(format!("{:?}", value.0)),
                Button::new("\u{f74e} Browse", move |world| {
                    let on_change = on_change.clone();
                    select_asset(world.resource(asset_cache()), T::asset_type(), move |asset_url| {
                        on_change(AssetUrlCollection::new(asset_url.all().into_iter().map(|x| AssetUrl::parse(x).unwrap()).collect()));
                    });
                })
                .style(ButtonStyle::Flat)
                .el(),
            ])
            .set(align_vertical(), Align::Center)
            .set(space_between_items(), STREET)
        } else {
            Text::el(format!("{:?}", value.0))
        }
    }
}
