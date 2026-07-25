use std::borrow::Cow;
use std::ptr::NonNull;

use anyhow::{Context, Result};

pub use self::conformance::*;
pub use self::entry_point::*;
pub use self::linked::*;
pub use self::module::*;
use crate::util::{from_ffi_string, to_ffi_string};
use crate::{SlangContext, Stage, TypeKind, com, sys};

mod conformance;
mod entry_point;
mod linked;
mod module;

pub trait AsBoxedComponentType {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_>;

    fn into_boxed(self) -> BoxedComponentType
    where
        Self: Sized,
    {
        let r = self.as_boxed();
        BoxedComponentType {
            inner: r.inner.clone(),
            ctx: r.ctx.clone(),
        }
    }

    fn link(&self) -> Result<LinkedModule> {
        let boxed = self.as_boxed();

        let mut linked = None;
        let mut diagnostics = None;
        let result = unsafe { boxed.inner.link(&mut linked, &mut diagnostics) };
        boxed.ctx.log_diagnostics(diagnostics);
        result?;

        Ok(LinkedModule {
            inner: linked.context("components were not linked")?,
            ctx: boxed.ctx.clone(),
        })
    }

    fn specialize(&self, args: &[SpecializationArg<'_>]) -> Result<BoxedComponentType> {
        let boxed = self.as_boxed();

        let mut strings = Vec::with_capacity(args.len());
        let mut specialization_args = Vec::with_capacity(args.len());
        for arg in args {
            match arg {
                SpecializationArg::Type(ty) => {
                    specialization_args.push(sys::slang_SpecializationArg {
                        kind: sys::slang_SpecializationArg_Kind_Type,
                        __bindgen_anon_1: sys::slang_SpecializationArg__bindgen_ty_1 {
                            type_: ty.inner.as_ptr().cast(),
                        },
                    });
                }
                SpecializationArg::Expr(cow) => {
                    let expr = strings.push_mut(to_ffi_string(cow.as_ref())?).as_ptr();
                    specialization_args.push(sys::slang_SpecializationArg {
                        kind: sys::slang_SpecializationArg_Kind_Expr,
                        __bindgen_anon_1: sys::slang_SpecializationArg__bindgen_ty_1 { expr },
                    });
                }
            }
        }

        let mut specialized = None;
        let mut diagnostics = None;
        let result = unsafe {
            boxed.inner.specialize(
                specialization_args.as_ptr(),
                specialization_args.len() as _,
                &mut specialized,
                &mut diagnostics,
            )
        };
        boxed.ctx.log_diagnostics(diagnostics);
        result?;

        Ok(BoxedComponentType {
            inner: specialized.context("component was not specialized")?,
            ctx: boxed.ctx.clone(),
        })
    }

    fn get_layout(&self) -> Result<ComponentLayout<'_>> {
        let boxed = self.as_boxed();

        let mut diagnostics = None;
        let layout = unsafe { boxed.inner.getLayout(0, &mut diagnostics) };
        boxed.ctx.log_diagnostics(diagnostics);
        Ok(ComponentLayout {
            parent: boxed.inner,
            ctx: boxed.ctx,
            inner: NonNull::new(layout).context("failed to get layout")?,
        })
    }
}

#[derive(Clone)]
pub struct BoxedComponentType {
    pub(crate) inner: com::IComponentType,
    pub(crate) ctx: SlangContext,
}

impl AsBoxedComponentType for BoxedComponentType {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            ctx: &self.ctx,
        }
    }

    fn into_boxed(self) -> BoxedComponentType {
        self
    }
}

impl AsRef<com::IComponentType> for BoxedComponentType {
    fn as_ref(&self) -> &com::IComponentType {
        &self.inner
    }
}

#[derive(Clone)]
pub struct BoxedComponentTypeRef<'a> {
    pub(crate) inner: &'a com::IComponentType,
    pub(crate) ctx: &'a SlangContext,
}

impl AsBoxedComponentType for BoxedComponentTypeRef<'_> {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: self.inner,
            ctx: self.ctx,
        }
    }
}

impl AsRef<com::IComponentType> for BoxedComponentTypeRef<'_> {
    fn as_ref(&self) -> &com::IComponentType {
        self.inner
    }
}

// === Specialization ===

pub enum SpecializationArg<'a> {
    Type(TypeLayout<'a>),
    Expr(Cow<'a, str>),
}

// === Reflection ===

#[derive(Clone)]
pub struct ComponentLayout<'a> {
    pub(crate) parent: &'a com::IComponentType,
    pub(crate) ctx: &'a SlangContext,
    pub(crate) inner: NonNull<sys::SlangProgramLayout>,
}

impl<'a> ComponentLayout<'a> {
    pub fn entry_point_count(&self) -> usize {
        unsafe { (self.ctx.vtable.reflection_get_entry_point_count)(self.inner.as_ptr()) as usize }
    }

    pub fn get_entry_point(&self, index: usize) -> Option<EntryPointLayout<'a>> {
        let entry_point = unsafe {
            (self.ctx.vtable.reflection_get_entry_point_by_index)(self.inner.as_ptr(), index as _)
        };
        Some(EntryPointLayout {
            parent: self.parent,
            ctx: self.ctx,
            inner: NonNull::new(entry_point)?,
        })
    }

    pub fn find_type(&self, name: &str) -> Option<TypeLayout<'_>> {
        let name = to_ffi_string(name).ok()?;
        let ty = unsafe {
            (self.ctx.vtable.reflection_find_type_by_name)(self.inner.as_ptr(), name.as_ptr())
        };
        Some(TypeLayout {
            parent: self.parent,
            ctx: self.ctx,
            inner: NonNull::new(ty)?,
        })
    }
}

#[derive(Clone)]
pub struct EntryPointLayout<'a> {
    #[expect(unused)]
    pub(crate) parent: &'a com::IComponentType,
    pub(crate) ctx: &'a SlangContext,
    pub(crate) inner: NonNull<sys::SlangEntryPointLayout>,
}

impl EntryPointLayout<'_> {
    pub fn get_name(&self) -> Result<String> {
        let name =
            unsafe { (self.ctx.vtable.reflection_entry_point_get_name)(self.inner.as_ptr()) };
        anyhow::ensure!(!name.is_null(), "failed to get entry point name");
        from_ffi_string(name)
    }

    pub fn get_stage(&self) -> Result<Stage> {
        let stage =
            unsafe { (self.ctx.vtable.reflection_entry_point_get_stage)(self.inner.as_ptr()) };
        Stage::from_slang(stage).context("unknown stage")
    }
}

#[derive(Clone)]
pub struct TypeLayout<'a> {
    #[expect(unused)]
    pub(crate) parent: &'a com::IComponentType,
    pub(crate) ctx: &'a SlangContext,
    pub(crate) inner: NonNull<sys::SlangReflectionType>,
}

impl TypeLayout<'_> {
    pub fn get_kind(&self) -> Result<TypeKind> {
        let kind = unsafe { (self.ctx.vtable.reflection_type_get_kind)(self.inner.as_ptr()) };
        TypeKind::from_slang(kind).context("unknown kind")
    }
}

// === Iter ===

#[derive(Clone)]
pub struct EntryPointsLayoutIter<'a, 'c> {
    layout: &'a ComponentLayout<'c>,
    next_index: usize,
    count: usize,
}

impl<'c> Iterator for EntryPointsLayoutIter<'_, 'c> {
    type Item = EntryPointLayout<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.count {
            return None;
        }

        let index = self.next_index;
        self.next_index += 1;

        let entry_point = unsafe {
            (self.layout.ctx.vtable.reflection_get_entry_point_by_index)(
                self.layout.inner.as_ptr(),
                index as _,
            )
        };
        Some(EntryPointLayout {
            parent: self.layout.parent,
            ctx: self.layout.ctx,
            inner: NonNull::new(entry_point)?,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for EntryPointsLayoutIter<'_, '_> {
    fn len(&self) -> usize {
        self.count.saturating_sub(self.next_index)
    }
}
