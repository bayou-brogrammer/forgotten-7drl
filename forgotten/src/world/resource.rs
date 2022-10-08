use mopa::Any;
use shred::cell::{Ref, RefMut, TrustCell};
use std::{
    any::TypeId,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::World;

macro_rules! fetch_panic {
    () => {{
        panic!(
            "\
          Tried to fetch resource of type `{resource_name_simple}`[^1] from the `World`, but \
          the resource does not exist.\n\
\n\
          You may ensure the resource exists through one of the following methods:\n\
\n\
          * Inserting it when the world is created: `world.insert(..)`.\n\
          * If the resource implements `Default`, include it in a system's `SystemData`, \
            and ensure the system is registered in the dispatcher.\n\
          * If the resource does not implement `Default`, insert it in the world during \
            `System::setup`.\n\
\n\
          [^1]: Full type name: `{resource_name_full}`\
          ",
            resource_name_simple = tynm::type_name::<T>(),
            resource_name_full = std::any::type_name::<T>(),
        )
    }};
}

/// Allows to fetch a resource in a system immutably.
///
/// If the resource isn't strictly required, you should use `Option<Fetch<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct Fetch<'a, T: 'a> {
    inner: Ref<'a, dyn Resource>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for Fetch<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> Clone for Fetch<'a, T> {
    fn clone(&self) -> Self {
        Fetch { inner: self.inner.clone(), phantom: PhantomData }
    }
}

/// Allows to fetch a resource in a system mutably.
///
/// If the resource isn't strictly required, you should use
/// `Option<FetchMut<T>>`.
///
/// # Type parameters
///
/// * `T`: The type of the resource
pub struct FetchMut<'a, T: 'a> {
    inner: RefMut<'a, dyn Resource>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for FetchMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.downcast_mut_unchecked() }
    }
}

/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(feature = "parallel")]
pub trait Resource: Any + Send + Sync + 'static {}

/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(not(feature = "parallel"))]
pub trait Resource: Any + 'static {}

mod __resource_mopafy_scope {
    #![allow(clippy::all)]

    use mopa::mopafy;

    use super::Resource;

    mopafy!(Resource);
}

#[cfg(feature = "parallel")]
impl<T> Resource for T where T: Any + Send + Sync {}
#[cfg(not(feature = "parallel"))]
impl<T> Resource for T where T: Any {}

/// The id of a [`Resource`], which simply wraps a type id and a "dynamic ID".
/// The "dynamic ID" is usually just left `0`, and, unless such documentation
/// says otherwise, other libraries will assume that it is always `0`; non-zero
/// IDs are only used for special resource types that are specifically defined
/// in a more dynamic way, such that resource types can essentially be created
/// at run time, without having different static types.
///
/// [`Resource`]: trait.Resource.html
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId {
    type_id: TypeId,
    dynamic_id: u64,
}

impl ResourceId {
    /// Creates a new resource id from a given type.
    #[inline]
    pub fn new<T: Resource>() -> Self {
        ResourceId::new_with_dynamic_id::<T>(0)
    }

    /// Create a new resource id from a raw type ID.
    #[inline]
    pub fn from_type_id(type_id: TypeId) -> Self {
        ResourceId::from_type_id_and_dynamic_id(type_id, 0)
    }

    /// Creates a new resource id from a given type and a `dynamic_id`.
    ///
    /// This is usually not what you want (unless you're implementing scripting
    /// with `shred` or some similar mechanism to define resources at run-time).
    ///
    /// Creating resource IDs with a `dynamic_id` unequal to `0` is only
    /// recommended for special types that are specifically defined for
    /// scripting; most libraries will just assume that resources are
    /// identified only by their type.
    #[inline]
    pub fn new_with_dynamic_id<T: Resource>(dynamic_id: u64) -> Self {
        ResourceId::from_type_id_and_dynamic_id(TypeId::of::<T>(), dynamic_id)
    }

    /// Create a new resource id from a raw type ID and a "dynamic ID" (see type
    /// documentation).
    #[inline]
    pub fn from_type_id_and_dynamic_id(type_id: TypeId, dynamic_id: u64) -> Self {
        ResourceId { type_id, dynamic_id }
    }

    pub(crate) fn assert_same_type_id<R: Resource>(&self) {
        let res_id0 = ResourceId::new::<R>();
        assert_eq!(res_id0.type_id, self.type_id, "Passed a `ResourceId` with a wrong type ID");
    }
}

pub type Resources = HashMap<ResourceId, TrustCell<Box<dyn Resource>>>;

impl World {
    pub fn insert<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.insert_by_id(ResourceId::new::<R>(), r);
    }

    pub fn insert_by_id<R>(&mut self, id: ResourceId, r: R)
    where
        R: Resource,
    {
        id.assert_same_type_id::<R>();

        self.resources.insert(id, TrustCell::new(Box::new(r)));
    }

    pub fn remove<R>(&mut self) -> Option<R>
    where
        R: Resource,
    {
        self.remove_by_id(ResourceId::new::<R>())
    }

    pub fn remove_by_id<R>(&mut self, id: ResourceId) -> Option<R>
    where
        R: Resource,
    {
        // False-positive
        #![allow(clippy::redundant_closure)]

        id.assert_same_type_id::<R>();

        self.resources
            .remove(&id)
            .map(TrustCell::into_inner)
            .map(|x: Box<dyn Resource>| x.downcast())
            .map(|x: Result<Box<R>, _>| x.ok().unwrap())
            .map(|x| *x)
    }

    /// Fetches the resource with the specified type `T` or panics if it doesn't
    /// exist.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is being accessed mutably.
    pub fn fetch<T>(&self) -> Fetch<T>
    where
        T: Resource,
    {
        self.try_fetch().unwrap_or_else(|| {
            if self.resources.is_empty() {
                eprintln!(
                    "Note: Could not find a resource (see the following panic);\
                     the `World` is completely empty. Did you accidentally create a fresh `World`?"
                )
            }

            fetch_panic!()
        })
    }

    /// Like `fetch`, but returns an `Option` instead of inserting a default
    /// value in case the resource does not exist.
    pub fn try_fetch<T>(&self) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        let res_id = ResourceId::new::<T>();

        self.resources
            .get(&res_id)
            .map(|r| Fetch { inner: Ref::map(r.borrow(), Box::as_ref), phantom: PhantomData })
    }

    /// Like `try_fetch`, but fetches the resource by its `ResourceId` which
    /// allows using a dynamic ID.
    ///
    /// This is usually not what you need; please read the type-level
    /// documentation of `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `T`.
    pub fn try_fetch_by_id<T>(&self, id: ResourceId) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        id.assert_same_type_id::<T>();

        self.resources
            .get(&id)
            .map(|r| Fetch { inner: Ref::map(r.borrow(), Box::as_ref), phantom: PhantomData })
    }

    /// Fetches the resource with the specified type `T` mutably.
    ///
    /// Please see `fetch` for details.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is already being accessed.
    pub fn fetch_mut<T>(&self) -> FetchMut<T>
    where
        T: Resource,
    {
        self.try_fetch_mut().unwrap_or_else(|| fetch_panic!())
    }

    /// Like `fetch_mut`, but returns an `Option` instead of inserting a default
    /// value in case the resource does not exist.
    pub fn try_fetch_mut<T>(&self) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        let res_id = ResourceId::new::<T>();

        self.resources
            .get(&res_id)
            .map(|r| FetchMut { inner: RefMut::map(r.borrow_mut(), Box::as_mut), phantom: PhantomData })
    }

    /// Like `try_fetch_mut`, but fetches the resource by its `ResourceId` which
    /// allows using a dynamic ID.
    ///
    /// This is usually not what you need; please read the type-level
    /// documentation of `ResourceId`.
    ///
    /// # Panics
    ///
    /// This method panics if `id` refers to a different type ID than `T`.
    pub fn try_fetch_mut_by_id<T>(&self, id: ResourceId) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        id.assert_same_type_id::<T>();

        self.resources
            .get(&id)
            .map(|r| FetchMut { inner: RefMut::map(r.borrow_mut(), Box::as_mut), phantom: PhantomData })
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.get_mut_raw(ResourceId::new::<T>()).map(|res| unsafe { res.downcast_mut_unchecked() })
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut_raw(&mut self, id: ResourceId) -> Option<&mut dyn Resource> {
        self.resources.get_mut(&id).map(TrustCell::get_mut).map(Box::as_mut)
    }
}
