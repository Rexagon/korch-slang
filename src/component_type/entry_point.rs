use anyhow::{Context, Result};

use crate::util::from_ffi_string;
use crate::{AsBoxedComponentType, BoxedComponentTypeRef, SlangContext, Stage, com};

#[derive(Clone)]
pub struct EntryPoint {
    pub(crate) inner: com::IEntryPoint,
    pub(crate) ctx: SlangContext,
}

impl EntryPoint {
    pub fn get_name(&self) -> Result<String> {
        let name = unsafe {
            let reflection = self.inner.getFunctionReflection();
            (self.ctx.vtable.reflection_fn_get_name)(reflection)
        };
        anyhow::ensure!(!name.is_null(), "failed to get function name");
        from_ffi_string(name)
    }

    pub fn get_stage(&self) -> Result<Stage> {
        let layout = self.get_layout()?;
        let entry_point_count = layout.entry_point_count();
        anyhow::ensure!(entry_point_count == 1, "invalid component layout");

        let vtable = self.ctx.vtable.as_ref();

        let entry_point =
            unsafe { (vtable.reflection_get_entry_point_by_index)(layout.inner.as_ptr(), 0) };
        let stage = unsafe { (vtable.reflection_entry_point_get_stage)(entry_point) };
        Stage::from_slang(stage).context("unknown stage")
    }
}

impl AsBoxedComponentType for EntryPoint {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            ctx: &self.ctx,
        }
    }
}

impl AsRef<com::IEntryPoint> for EntryPoint {
    fn as_ref(&self) -> &com::IEntryPoint {
        &self.inner
    }
}
