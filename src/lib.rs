#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

#[cfg(feature = "bevy_assets")]
use bevy::asset::VisitAssetDependencies;
use bevy::prelude::*;
use rand::{Rng, seq::SliceRandom as _};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

/// A bag full of items that can endlessly be picked in a random order.
///
/// The bag will be emptied in *drafts*, where each draft contains all the items in the bag, but in a random order.
/// This means that if you have e.g. a bag with 3 soundtracks, all of them will play once in a random order, and then the bag will be refilled with the same soundtracks in a random order.
/// If the bag contains no duplicates, items are always picked such that the same item is never picked twice in a row.
///
/// No more playing the same sound effect or dialogue twice in a row!
#[derive(Component, Resource, Debug, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Component, Resource)]
#[non_exhaustive]
pub struct ShuffleBag<T> {
    /// The full collection of items that could be in the bag at the same time.
    pub full_collection: Vec<T>,
    /// The indices of the current draft of items that can be picked from the bag.
    pub current_draft: Vec<usize>,
    /// The last item that was picked from the bag. If the bag was just created, this will be `None`.
    pub last_pick: Option<usize>,
}

impl<T> ShuffleBag<T> {
    /// Create a new shuffle bag from an iterator. Returns `None` if the iterator is empty.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use std::collections::HashSet;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let mut possible_loot = HashSet::new();
    /// possible_loot.insert("gold");
    /// possible_loot.insert("armor");
    /// possible_loot.insert("sword");
    /// let treasure_chest = ShuffleBag::try_from_iter(possible_loot, &mut rng).unwrap();
    /// ```
    pub fn try_from_iter(iter: impl IntoIterator<Item = T>, rng: &mut impl Rng) -> Option<Self> {
        let full_collection: Vec<_> = iter.into_iter().collect();
        Self::try_new(full_collection, rng)
    }

    /// Create a new shuffle bag from something that can be converted into a vector. Returns `None` if the vector is empty.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let treasure_chest = ShuffleBag::try_new(["gold", "armor", "sword"], &mut rng).unwrap();
    /// ```
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

    /// Shuffle a new draft of the bag. This is automatically called when the bag is empty or when the bag is first created.
    ///
    /// "Draft" here means the collection of items that can be picked from the bag.
    /// After this call, the draft will contain all the items in the bag, but in a random order.
    /// The draft is constructed such that the next item to be picked by [`ShuffleBag::pick`] is not the same as the last item that was picked.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let mut treasure_chest = ShuffleBag::try_new(["gold", "armor", "sword"], &mut rng).unwrap();
    /// let loot = treasure_chest.pick(&mut rng);
    ///
    /// // The player just used a joker to get a new item! Let's reshuffle the bag. Doing so will
    /// // still ensure that the same item is never picked twice in a row.
    /// treasure_chest.shuffle_new_draft(&mut rng);
    /// let new_loot = treasure_chest.pick(&mut rng);
    /// ```
    pub fn shuffle_new_draft(&mut self, rng: &mut impl Rng) {
        self.current_draft = (0..self.full_collection.len()).collect();
        self.current_draft.shuffle(rng);

        if self.current_draft.len() <= 1 {
            return;
        }

        let Some(last_pick) = &self.last_pick else {
            return;
        };

        if self.current_draft.last() != Some(last_pick) {
            return;
        }

        // Looks like we picked the same item twice in a row, so let's shuffle it into the middle of the draft.
        let max_index = self.current_draft.len() - 2;
        let index = rng.random_range(0..=max_index);
        // Swap the this item with the last item in the draft.
        let new_next_pick = self.current_draft.swap_remove(index);
        self.current_draft.push(new_next_pick);
    }

    /// Shuffle a new draft of the bag and forget the last item that was picked.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let mut treasure_chest = ShuffleBag::try_new(["gold", "armor", "sword"], &mut rng).unwrap();
    /// let loot = treasure_chest.pick(&mut rng);
    ///
    /// // The player just reloaded from a save, so let's reset the bag.
    /// treasure_chest.reset(&mut rng);
    /// let new_loot = treasure_chest.pick(&mut rng);
    /// ```
    pub fn reset(&mut self, rng: &mut impl Rng) {
        self.current_draft = vec![];
        self.last_pick = None;
        self.shuffle_new_draft(rng);
    }

    /// Pick an item from the bag. If the bag is empty, this will automatically shuffle a new draft.
    /// If there are no duplicates in the bag, this will never pick the same item twice in a row.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let mut treasure_chest = ShuffleBag::try_new(["gold", "armor", "sword"], &mut rng).unwrap();
    /// let loot = treasure_chest.pick(&mut rng);
    /// ```
    pub fn pick(&mut self, rng: &mut impl Rng) -> &T {
        let pick = self.current_draft.pop().unwrap();
        self.last_pick = Some(pick);

        if self.current_draft.is_empty() {
            self.shuffle_new_draft(rng);
        }

        &self.full_collection[pick]
    }

    /// Peek at the next item that would be picked from the bag.
    ///
    /// # Example
    /// ```
    /// # use bevy::prelude::*;
    /// # use bevy_shuffle_bag::ShuffleBag;
    /// let mut rng = rand::rng();
    /// let mut shop = ShuffleBag::try_new(["potion", "shield", "bow"], &mut rng).unwrap();
    ///
    /// // The player is browsing the shop. The shopkeeper randomly picks an item that is available for purchase.
    /// let available_purchase = shop.peek().clone();
    ///
    /// // The player likes what they see, so they buy it!
    /// let purchased_item = shop.pick(&mut rng).clone();
    ///
    /// assert_eq!(purchased_item, available_purchase);
    /// ```
    pub fn peek(&self) -> &T {
        &self.full_collection[*self.current_draft.last().unwrap()]
    }
}

impl<T: Clone> Clone for ShuffleBag<T> {
    fn clone(&self) -> Self {
        Self {
            full_collection: self.full_collection.clone(),
            current_draft: self.current_draft.clone(),
            last_pick: self.last_pick,
        }
    }
}

impl<T: PartialEq> PartialEq for ShuffleBag<T> {
    fn eq(&self, other: &Self) -> bool {
        self.full_collection == other.full_collection
            && self.current_draft == other.current_draft
            && self.last_pick == other.last_pick
    }
}

impl<T: Eq> Eq for ShuffleBag<T> {}

impl<T: Hash + Clone> Hash for ShuffleBag<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_collection.hash(state);
        self.current_draft.hash(state);
        self.last_pick.hash(state);
    }
}

impl<T: VisitAssetDependencies> VisitAssetDependencies for ShuffleBag<T> {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy::asset::UntypedAssetId)) {
        for item in &self.full_collection {
            item.visit_dependencies(visit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::reflect::Typed;
    use paste::paste;
    #[cfg(feature = "serialize")]
    use serde::{Deserialize, Serialize};

    #[test]
    fn fails_to_create_empty_shuffle_bag() {
        let mut rng = rand::rng();
        let bag = ShuffleBag::<usize>::try_new(vec![], &mut rng);
        assert!(bag.is_none());
    }

    #[test]
    fn picks_same_item_from_singular_bag() {
        let mut rng = rand::rng();
        let mut bag = ShuffleBag::<usize>::try_new(vec![1], &mut rng).unwrap();
        for _ in 0..100 {
            assert_eq!(*bag.pick(&mut rng), 1);
        }
    }

    #[test]
    fn picks_all_items_from_bag() {
        let mut rng = rand::rng();
        let mut bag = ShuffleBag::<usize>::try_new(vec![1, 2, 3], &mut rng).unwrap();
        let mut picked = Vec::new();
        for _ in 0..99 {
            let item = bag.pick(&mut rng);
            picked.push(*item);
        }
        assert_eq!(picked.len(), 99, "expected 99 items, got {}", picked.len());
        let ones = picked.iter().filter(|&&item| item == 1).count();
        let twos = picked.iter().filter(|&&item| item == 2).count();
        let threes = picked.iter().filter(|&&item| item == 3).count();
        assert!(ones == 33, "ones: {} (expected 33)", ones);
        assert!(twos == 33, "twos: {} (expected 33)", twos);
        assert!(threes == 33, "threes: {} (expected 33)", threes);
    }

    #[test]
    fn never_picks_the_same_item_twice() {
        let mut rng = rand::rng();
        let mut bag = ShuffleBag::<usize>::try_new(vec![1, 2, 3], &mut rng).unwrap();
        let mut last_pick = None;
        for _ in 0..1000 {
            let pick = *bag.pick(&mut rng);
            assert_ne!(Some(pick), last_pick);
            last_pick = Some(pick);
        }
    }

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
        Typed,
        Component,
        Resource
    );

    #[cfg(feature = "serialize")]
    assert_implements_type!(Serialize);

    #[cfg(feature = "serialize")]
    #[test]
    fn is_deserialize() {
        fn accept_type<T: for<'a> Deserialize<'a>>(_: T) {}
        let mut rng = rand::rng();
        let bag = ShuffleBag::try_new(vec![1, 2, 3], &mut rng).unwrap();
        accept_type(bag);
    }

    #[derive(Asset, TypePath)]
    struct _TestAsset {
        #[dependency]
        shuffle_bag: ShuffleBag<Handle<()>>,
    }

    macro_rules! assert_implements_type {
        ($($name:ty),*) => {
            $(
                paste! {
                    #[test]
                    #[allow(non_snake_case)]
                    fn [<is_ $name>]() {
                    fn accept_type<T: $name>(_: T) {}
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
