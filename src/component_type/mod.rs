use std::ptr::NonNull;

use anyhow::{Context, Result};

pub use self::entry_point::*;
pub use self::linked::*;
pub use self::module::*;
use crate::{SlangContext, com, sys};

mod entry_point;
mod linked;
mod module;

pub trait AsBoxedComponentType {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_>;

    fn into_boxed(self: Self) -> BoxedComponentType
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
        unsafe { boxed.inner.link(&mut linked, &mut diagnostics)? };
        boxed.ctx.log_diagnostics(diagnostics);
        Ok(LinkedModule {
            inner: linked.context("module was not linked")?,
            ctx: boxed.ctx.clone(),
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

    fn into_boxed(self: Self) -> BoxedComponentType {
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
        &self.inner
    }
}

// === Raw Stuff ===

pub(crate) trait IComponentTypeExt {
    fn get_raw_layout(&self, ctx: &SlangContext) -> Result<NonNull<sys::SlangProgramLayout>>;
}

impl<T: std::ops::Deref<Target = com::IComponentType>> IComponentTypeExt for T {
    fn get_raw_layout(&self, ctx: &SlangContext) -> Result<NonNull<sys::SlangProgramLayout>> {
        let mut diagnostics = None;
        let layout = unsafe { self.getLayout(0, &mut diagnostics) };
        ctx.log_diagnostics(diagnostics);
        NonNull::new(layout).context("failed to get layout")
    }
}
