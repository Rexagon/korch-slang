use anyhow::{Context, Result};

use crate::util::from_ffi_string;
use crate::{AsBoxedComponentType, BoxedComponentTypeRef, SlangContext, Stage, com, sys};

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
        let entry_point = self.get_entry_point_layout()?;
        let stage = unsafe { (self.ctx.vtable.reflection_entry_point_get_stage)(entry_point) };
        Stage::from_slang(stage).context("unknown stage")
    }

    pub fn get_compute_thread_size(&self) -> Result<(u64, u64, u64)> {
        let entry_point = self.get_entry_point_layout()?;
        let mut size = [1u64; 3];
        unsafe {
            (self.ctx.vtable.reflection_entry_point_get_thread_group_size)(
                entry_point,
                3,
                size.as_mut_ptr(),
            )
        };
        let [x, y, z] = size;
        Ok((x, y, z))
    }

    fn get_entry_point_layout(&self) -> Result<*mut sys::SlangEntryPointLayout> {
        let layout = self.get_layout()?;
        let entry_point_count = layout.entry_point_count();
        anyhow::ensure!(entry_point_count == 1, "invalid component layout");

        Ok(unsafe {
            (self.ctx.vtable.reflection_get_entry_point_by_index)(layout.inner.as_ptr(), 0)
        })
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
