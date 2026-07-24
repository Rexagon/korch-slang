use anyhow::{Context, Result};

use crate::{AsBoxedComponentType, BoxedComponentTypeRef, SlangContext, com};

#[derive(Clone)]
pub struct LinkedModule {
    pub(crate) inner: com::IComponentType,
    pub(crate) ctx: SlangContext,
}

impl LinkedModule {
    // TODO: Check entrypoints.
    pub fn get_entry_point_code(
        &self,
        entry_point_index: usize,
        target_index: usize,
    ) -> Result<Vec<u8>> {
        anyhow::ensure!(
            entry_point_index <= i64::MAX as usize,
            "invalid entry point index"
        );
        anyhow::ensure!(target_index <= i64::MAX as usize, "invalid target index");

        let mut diagnostics = None;
        let mut code = None;
        unsafe {
            self.inner.getEntryPointCode(
                entry_point_index as i64,
                target_index as i64,
                &mut code,
                &mut diagnostics,
            )?;
        };
        self.ctx.log_diagnostics(diagnostics);
        let code = code.context("code was not created")?;
        let code = unsafe {
            std::slice::from_raw_parts(code.getBufferPointer().cast::<u8>(), code.getBufferSize())
        };
        Ok(code.to_vec())
    }
}

impl AsBoxedComponentType for LinkedModule {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            ctx: &self.ctx,
        }
    }

    fn link(&self) -> Result<LinkedModule> {
        Ok(self.clone())
    }
}

impl AsRef<com::IComponentType> for LinkedModule {
    fn as_ref(&self) -> &com::IComponentType {
        &self.inner
    }
}
