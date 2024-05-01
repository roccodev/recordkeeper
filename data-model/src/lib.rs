//! Data model exported by the procedural macros for `SaveBin`.

use std::{any::TypeId, marker::PhantomData};

/// Types that have a well-formed data model for the save file format.
pub trait Model: 'static {
    fn model() -> TypeModel;
}

pub type ModelName = &'static str;

/// The model for a struct type, i.e. how it and its fields are represented
/// in the save file.
#[derive(Debug)]
pub struct StructModel {
    /// The struct's name.
    ///
    /// This does not include generic lifetimes and type parameters.
    pub name: ModelName,
    /// A unique identifier for the struct's type, which also considers
    /// generic parameters.
    pub type_id: TypeId,
    pub total_len: usize,
    pub fields: Vec<FieldModel>,
}

pub struct StructModelBuilder(StructModel);

/// The model for a struct field.
#[derive(Debug)]
pub struct FieldModel {
    pub name: ModelName,
    pub type_model: TypeModel,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
pub enum TypeModel {
    /// Empty types
    Empty,
    /// Struct types
    Struct(Box<StructModel>),
    /// Array types
    Array(Box<(TypeModel, usize)>),
    /// Rust primitive types
    Primitive(ModelName),
}

impl StructModel {
    /// Creates a model for a struct with no fields (or all hidden fields).
    ///
    /// It is "opaque" because from the user's perspective, it is just a block of bytes with a
    /// name and size.
    pub fn new_opaque(name: ModelName, type_id: TypeId, size: usize) -> Self {
        Self {
            name,
            type_id,
            total_len: size,
            fields: Vec::new(),
        }
    }

    /// Creates a struct model builder.
    ///
    /// The builder allows fields to be added to the model.
    pub fn new_builder(name: ModelName, type_id: TypeId) -> StructModelBuilder {
        StructModelBuilder(StructModel {
            name,
            type_id,
            total_len: 0,
            fields: Vec::new(),
        })
    }
}

impl StructModelBuilder {
    pub fn add_field(
        &mut self,
        name: ModelName,
        type_model: TypeModel,
        offset: usize,
        size: usize,
    ) -> &mut Self {
        if offset < self.0.total_len {
            self.0.total_len = offset;
        }
        self.0.fields.push(FieldModel {
            name,
            offset,
            type_model,
            size,
        });
        self.0.total_len += size;
        self
    }

    pub fn set_total_len(&mut self, size: usize) -> &mut Self {
        assert!(size >= self.0.total_len);
        self.0.total_len = size;
        self
    }

    pub fn build(self) -> StructModel {
        self.0
    }
}

macro_rules! builtin_impl {
    ($($types:tt ) *) => {
        $(
            impl Model for $types {
                fn model() -> TypeModel {
                    TypeModel::Primitive(stringify!($types))
                }
            }
        )*
    };
}

builtin_impl!(bool u8 i8 u64 i64 f64 u32 i32 f32 u16 i16);

impl<T, const N: usize> Model for [T; N]
where
    T: Model,
{
    fn model() -> TypeModel {
        TypeModel::Array(Box::new((T::model(), N)))
    }
}

impl<T, const N: usize> Model for Box<[T; N]>
where
    T: Model,
{
    fn model() -> TypeModel {
        TypeModel::Array(Box::new((T::model(), N)))
    }
}

impl<T: 'static> Model for PhantomData<T> {
    fn model() -> TypeModel {
        TypeModel::Empty
    }
}
