use bevy::{asset::VisitAssetDependencies, prelude::*, reflect::Typed};
use rand::{Rng, seq::SliceRandom as _};
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};
fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}

#[derive(Component, Clone)]
#[non_exhaustive]
pub struct ShuffleBag<T: Clone> {
    pub full_collection: Vec<T>,
    pub current_draft: Vec<T>,
    pub last_pick: Option<T>,
}

impl<T: Clone> ShuffleBag<T> {
    pub fn try_new(full_collection: impl Into<Vec<T>>, rng: &mut impl Rng) -> Option<Self> {
        let full_collection = full_collection.into();
        if full_collection.is_empty() {
            return None;
        }

        let mut bag = Self {
            full_collection,
            current_draft: vec![],
            last_pick: None,
        };

        bag.shuffle_new_draft(rng);
        Some(bag)
    }

    pub fn shuffle_new_draft(&mut self, rng: &mut impl Rng) {
        self.current_draft = self.full_collection.clone();
        self.current_draft.shuffle(rng);
    }

    pub fn pick(&mut self, rng: &mut impl Rng) -> T {
        let pick = self.current_draft.pop().unwrap();
        if self.current_draft.is_empty() {
            self.shuffle_new_draft(rng);
        }

        self.last_pick = Some(pick.clone());
        pick
    }
}

impl<T: Clone + Debug> Debug for ShuffleBag<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShuffleBag")
            .field("full_collection", &self.full_collection)
            .field("current_draft", &self.current_draft)
            .field("last_pick", &self.last_pick)
            .finish()
    }
}

impl<T: PartialEq + Clone> PartialEq for ShuffleBag<T> {
    fn eq(&self, other: &Self) -> bool {
        self.full_collection == other.full_collection
            && self.current_draft == other.current_draft
            && self.last_pick == other.last_pick
    }
}

impl<T: Eq + Clone> Eq for ShuffleBag<T> {}

impl<T: Hash + Clone> Hash for ShuffleBag<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_collection.hash(state);
        self.current_draft.hash(state);
        self.last_pick.hash(state);
    }
}

impl<T: Clone + Asset> Asset for ShuffleBag<T> {}
impl<T: Clone + TypePath> TypePath for ShuffleBag<T> {
    fn type_path() -> &'static str {
        todo!()
    }

    fn short_type_path() -> &'static str {
        todo!()
    }
}

impl<T: Clone + VisitAssetDependencies> VisitAssetDependencies for ShuffleBag<T> {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        for item in &self.full_collection {
            item.visit_dependencies(visit);
        }
    }
}

impl<T: Clone + PartialReflect + TypePath> PartialReflect for ShuffleBag<T> {
    fn get_represented_type_info(&self) -> Option<&'static bevy::reflect::TypeInfo> {
        todo!()
    }

    fn into_partial_reflect(self: Box<Self>) -> Box<dyn PartialReflect> {
        todo!()
    }

    fn as_partial_reflect(&self) -> &dyn PartialReflect {
        todo!()
    }

    fn as_partial_reflect_mut(&mut self) -> &mut dyn PartialReflect {
        todo!()
    }

    fn try_into_reflect(self: Box<Self>) -> Result<Box<dyn Reflect>, Box<dyn PartialReflect>> {
        todo!()
    }

    fn try_as_reflect(&self) -> Option<&dyn Reflect> {
        todo!()
    }

    fn try_as_reflect_mut(&mut self) -> Option<&mut dyn Reflect> {
        todo!()
    }

    fn try_apply(&mut self, value: &dyn PartialReflect) -> Result<(), bevy::reflect::ApplyError> {
        todo!()
    }

    fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
        todo!()
    }

    fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
        todo!()
    }

    fn reflect_owned(self: Box<Self>) -> bevy::reflect::ReflectOwned {
        todo!()
    }

    fn clone_value(&self) -> Box<dyn PartialReflect> {
        todo!()
    }
}

impl<T: Clone + Reflect + TypePath + Typed> Reflect for ShuffleBag<T> {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        todo!()
    }

    fn as_reflect(&self) -> &dyn Reflect {
        todo!()
    }

    fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        todo!()
    }

    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
        todo!()
    }
}

impl<T: Clone + Typed> Typed for ShuffleBag<T> {
    fn type_info() -> &'static bevy::reflect::TypeInfo {
        todo!()
    }
}

impl<T: Clone + PartialReflect + TypePath> Struct for ShuffleBag<T> {
    fn field(&self, name: &str) -> Option<&dyn PartialReflect> {
        todo!()
    }

    fn field_mut(&mut self, name: &str) -> Option<&mut dyn PartialReflect> {
        todo!()
    }

    fn field_at(&self, index: usize) -> Option<&dyn PartialReflect> {
        todo!()
    }

    fn field_at_mut(&mut self, index: usize) -> Option<&mut dyn PartialReflect> {
        todo!()
    }

    fn name_at(&self, index: usize) -> Option<&str> {
        todo!()
    }

    fn field_len(&self) -> usize {
        todo!()
    }

    fn iter_fields(&self) -> bevy::reflect::FieldIter {
        todo!()
    }

    fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    assert_implements_type!(
        Eq,
        Hash,
        Clone,
        Debug,
        PartialEq,
        PartialReflect,
        Reflect,
        Struct,
        TypePath,
        Typed
    );

    macro_rules! assert_implements_type {
        ($($name:ident),*) => {
            $(
                paste! {
                    #[test]
                    #[allow(non_snake_case)]
                    fn [<is_ $name>]() {
                    fn accept_type<T: $name>(_a: T) {}
                        let mut rng = rand::rng();
                        let bag = ShuffleBag::try_new(vec![1, 2, 3], &mut rng).unwrap();
                        accept_type(bag);
                    }
                }
            )*
        };
    }
    use assert_implements_type;
}
