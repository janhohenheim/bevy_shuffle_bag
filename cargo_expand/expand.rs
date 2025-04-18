#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use bevy::prelude::*;
#[reflect(Component, Resource)]
struct ShuffleBag {
    pub full_collection: Vec<i64>,
    pub current_draft: Vec<i64>,
    pub last_pick: Option<i64>,
}
impl bevy::ecs::component::Component for ShuffleBag
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
    fn register_required_components(
        requiree: bevy::ecs::component::ComponentId,
        components: &mut bevy::ecs::component::Components,
        storages: &mut bevy::ecs::storage::Storages,
        required_components: &mut bevy::ecs::component::RequiredComponents,
        inheritance_depth: u16,
    ) {}
    #[allow(unused_variables)]
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {}
}
impl bevy::ecs::system::Resource for ShuffleBag
where
    Self: Send + Sync + 'static,
{}
const _: () = {
    #[allow(unused_mut)]
    impl bevy::reflect::GetTypeRegistration for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        fn get_type_registration() -> bevy::reflect::TypeRegistration {
            let mut registration = bevy::reflect::TypeRegistration::of::<Self>();
            registration
                .insert::<
                    bevy::reflect::ReflectFromPtr,
                >(bevy::reflect::FromType::<Self>::from_type());
            registration
                .insert::<
                    bevy::reflect::ReflectFromReflect,
                >(bevy::reflect::FromType::<Self>::from_type());
            registration
                .insert::<
                    ReflectComponent,
                >(bevy::reflect::FromType::<Self>::from_type());
            registration
                .insert::<ReflectResource>(bevy::reflect::FromType::<Self>::from_type());
            registration
        }
        #[inline(never)]
        fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {
            <Vec<
                i64,
            > as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                registry,
            );
            <Vec<
                i64,
            > as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                registry,
            );
            <Option<
                i64,
            > as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                registry,
            );
        }
    }
    impl bevy::reflect::Typed for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        #[inline]
        fn type_info() -> &'static bevy::reflect::TypeInfo {
            static CELL: bevy::reflect::utility::NonGenericTypeInfoCell = bevy::reflect::utility::NonGenericTypeInfoCell::new();
            CELL.get_or_set(|| {
                bevy::reflect::TypeInfo::Struct(
                    bevy::reflect::StructInfo::new::<
                        Self,
                    >(
                            &[
                                bevy::reflect::NamedField::new::<
                                    Vec<i64>,
                                >("full_collection")
                                    .with_custom_attributes(
                                        bevy::reflect::attributes::CustomAttributes::default(),
                                    ),
                                bevy::reflect::NamedField::new::<Vec<i64>>("current_draft")
                                    .with_custom_attributes(
                                        bevy::reflect::attributes::CustomAttributes::default(),
                                    ),
                                bevy::reflect::NamedField::new::<Option<i64>>("last_pick")
                                    .with_custom_attributes(
                                        bevy::reflect::attributes::CustomAttributes::default(),
                                    ),
                            ],
                        )
                        .with_custom_attributes(
                            bevy::reflect::attributes::CustomAttributes::default(),
                        ),
                )
            })
        }
    }
    impl bevy::reflect::TypePath for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
    {
        fn type_path() -> &'static str {
            "test::ShuffleBag"
        }
        fn short_type_path() -> &'static str {
            "ShuffleBag"
        }
        fn type_ident() -> Option<&'static str> {
            ::core::option::Option::Some("ShuffleBag")
        }
        fn crate_name() -> Option<&'static str> {
            ::core::option::Option::Some("test".split(':').next().unwrap())
        }
        fn module_path() -> Option<&'static str> {
            ::core::option::Option::Some("test")
        }
    }
    impl bevy::reflect::Reflect for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        #[inline]
        fn into_any(
            self: ::std::boxed::Box<Self>,
        ) -> ::std::boxed::Box<dyn ::core::any::Any> {
            self
        }
        #[inline]
        fn as_any(&self) -> &dyn ::core::any::Any {
            self
        }
        #[inline]
        fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
            self
        }
        #[inline]
        fn into_reflect(
            self: ::std::boxed::Box<Self>,
        ) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
            self
        }
        #[inline]
        fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
            self
        }
        #[inline]
        fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
            self
        }
        #[inline]
        fn set(
            &mut self,
            value: ::std::boxed::Box<dyn bevy::reflect::Reflect>,
        ) -> ::core::result::Result<(), ::std::boxed::Box<dyn bevy::reflect::Reflect>> {
            *self = <dyn bevy::reflect::Reflect>::take(value)?;
            ::core::result::Result::Ok(())
        }
    }
    impl bevy::reflect::Struct for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        fn field(
            &self,
            name: &str,
        ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
            match name {
                "full_collection" => ::core::option::Option::Some(&self.full_collection),
                "current_draft" => ::core::option::Option::Some(&self.current_draft),
                "last_pick" => ::core::option::Option::Some(&self.last_pick),
                _ => ::core::option::Option::None,
            }
        }
        fn field_mut(
            &mut self,
            name: &str,
        ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
            match name {
                "full_collection" => {
                    ::core::option::Option::Some(&mut self.full_collection)
                }
                "current_draft" => ::core::option::Option::Some(&mut self.current_draft),
                "last_pick" => ::core::option::Option::Some(&mut self.last_pick),
                _ => ::core::option::Option::None,
            }
        }
        fn field_at(
            &self,
            index: usize,
        ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
            match index {
                0usize => ::core::option::Option::Some(&self.full_collection),
                1usize => ::core::option::Option::Some(&self.current_draft),
                2usize => ::core::option::Option::Some(&self.last_pick),
                _ => ::core::option::Option::None,
            }
        }
        fn field_at_mut(
            &mut self,
            index: usize,
        ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
            match index {
                0usize => ::core::option::Option::Some(&mut self.full_collection),
                1usize => ::core::option::Option::Some(&mut self.current_draft),
                2usize => ::core::option::Option::Some(&mut self.last_pick),
                _ => ::core::option::Option::None,
            }
        }
        fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
            match index {
                0usize => ::core::option::Option::Some("full_collection"),
                1usize => ::core::option::Option::Some("current_draft"),
                2usize => ::core::option::Option::Some("last_pick"),
                _ => ::core::option::Option::None,
            }
        }
        fn field_len(&self) -> usize {
            3usize
        }
        fn iter_fields(&self) -> bevy::reflect::FieldIter {
            bevy::reflect::FieldIter::new(self)
        }
        fn clone_dynamic(&self) -> bevy::reflect::DynamicStruct {
            let mut dynamic: bevy::reflect::DynamicStruct = ::core::default::Default::default();
            dynamic
                .set_represented_type(
                    bevy::reflect::PartialReflect::get_represented_type_info(self),
                );
            dynamic
                .insert_boxed(
                    "full_collection",
                    bevy::reflect::PartialReflect::clone_value(&self.full_collection),
                );
            dynamic
                .insert_boxed(
                    "current_draft",
                    bevy::reflect::PartialReflect::clone_value(&self.current_draft),
                );
            dynamic
                .insert_boxed(
                    "last_pick",
                    bevy::reflect::PartialReflect::clone_value(&self.last_pick),
                );
            dynamic
        }
    }
    impl bevy::reflect::PartialReflect for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        #[inline]
        fn get_represented_type_info(
            &self,
        ) -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
            ::core::option::Option::Some(<Self as bevy::reflect::Typed>::type_info())
        }
        #[inline]
        fn clone_value(&self) -> ::std::boxed::Box<dyn bevy::reflect::PartialReflect> {
            ::std::boxed::Box::new(bevy::reflect::Struct::clone_dynamic(self))
        }
        #[inline]
        fn try_apply(
            &mut self,
            value: &dyn bevy::reflect::PartialReflect,
        ) -> ::core::result::Result<(), bevy::reflect::ApplyError> {
            if let bevy::reflect::ReflectRef::Struct(struct_value) = bevy::reflect::PartialReflect::reflect_ref(
                value,
            ) {
                for (i, value) in ::core::iter::Iterator::enumerate(
                    bevy::reflect::Struct::iter_fields(struct_value),
                ) {
                    let name = bevy::reflect::Struct::name_at(struct_value, i).unwrap();
                    if let ::core::option::Option::Some(v) = bevy::reflect::Struct::field_mut(
                        self,
                        name,
                    ) {
                        bevy::reflect::PartialReflect::try_apply(v, value)?;
                    }
                }
            } else {
                return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                    from_kind: bevy::reflect::PartialReflect::reflect_kind(value),
                    to_kind: bevy::reflect::ReflectKind::Struct,
                });
            }
            ::core::result::Result::Ok(())
        }
        #[inline]
        fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
            bevy::reflect::ReflectKind::Struct
        }
        #[inline]
        fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
            bevy::reflect::ReflectRef::Struct(self)
        }
        #[inline]
        fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
            bevy::reflect::ReflectMut::Struct(self)
        }
        #[inline]
        fn reflect_owned(self: ::std::boxed::Box<Self>) -> bevy::reflect::ReflectOwned {
            bevy::reflect::ReflectOwned::Struct(self)
        }
        #[inline]
        fn try_into_reflect(
            self: ::std::boxed::Box<Self>,
        ) -> ::core::result::Result<
            ::std::boxed::Box<dyn bevy::reflect::Reflect>,
            ::std::boxed::Box<dyn bevy::reflect::PartialReflect>,
        > {
            ::core::result::Result::Ok(self)
        }
        #[inline]
        fn try_as_reflect(&self) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
            ::core::option::Option::Some(self)
        }
        #[inline]
        fn try_as_reflect_mut(
            &mut self,
        ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
            ::core::option::Option::Some(self)
        }
        #[inline]
        fn into_partial_reflect(
            self: ::std::boxed::Box<Self>,
        ) -> ::std::boxed::Box<dyn bevy::reflect::PartialReflect> {
            self
        }
        #[inline]
        fn as_partial_reflect(&self) -> &dyn bevy::reflect::PartialReflect {
            self
        }
        #[inline]
        fn as_partial_reflect_mut(&mut self) -> &mut dyn bevy::reflect::PartialReflect {
            self
        }
        fn reflect_partial_eq(
            &self,
            value: &dyn bevy::reflect::PartialReflect,
        ) -> ::core::option::Option<bool> {
            (bevy::reflect::struct_partial_eq)(self, value)
        }
    }
    impl bevy::reflect::FromReflect for ShuffleBag
    where
        ShuffleBag: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Vec<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
        Option<
            i64,
        >: bevy::reflect::FromReflect + bevy::reflect::TypePath
            + bevy::reflect::MaybeTyped
            + bevy::reflect::__macro_exports::RegisterForReflection,
    {
        fn from_reflect(
            reflect: &dyn bevy::reflect::PartialReflect,
        ) -> ::core::option::Option<Self> {
            if let bevy::reflect::ReflectRef::Struct(__ref_struct) = bevy::reflect::PartialReflect::reflect_ref(
                reflect,
            ) {
                let __this = Self {
                    full_collection: (|| <Vec<
                        i64,
                    > as bevy::reflect::FromReflect>::from_reflect(
                        bevy::reflect::Struct::field(__ref_struct, "full_collection")?,
                    ))()?,
                    current_draft: (|| <Vec<
                        i64,
                    > as bevy::reflect::FromReflect>::from_reflect(
                        bevy::reflect::Struct::field(__ref_struct, "current_draft")?,
                    ))()?,
                    last_pick: (|| <Option<
                        i64,
                    > as bevy::reflect::FromReflect>::from_reflect(
                        bevy::reflect::Struct::field(__ref_struct, "last_pick")?,
                    ))()?,
                };
                ::core::option::Option::Some(__this)
            } else {
                ::core::option::Option::None
            }
        }
    }
};
fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}
