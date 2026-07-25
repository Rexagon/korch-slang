use anyhow::{Context, Result};

use crate::{AsBoxedComponentType, BoxedComponentTypeRef, SlangContext, com};

/// A typed representation of successfully linked component types.
#[derive(Clone)]
pub struct LinkedModule {
    pub(crate) inner: com::IComponentType,
    pub(crate) session: com::ISession,
    pub(crate) ctx: SlangContext,
}

impl LinkedModule {
    /// Compiles all entry points into a single blob.
    ///
    /// The `target_index` is the target's index in
    /// [`SessionDescriptor::targets`].
    ///
    /// [`SessionDescriptor::targets`]: crate::SessionDescriptor::targets
    pub fn get_target_code(&self, target_index: usize) -> Result<Vec<u8>> {
        anyhow::ensure!(target_index <= i64::MAX as usize, "invalid target index");

        let mut diagnostics = None;
        let mut code = None;
        let result = unsafe {
            self.inner
                .getTargetCode(target_index as i64, &mut code, &mut diagnostics)
        };
        self.ctx.log_diagnostics(diagnostics);
        result?;

        let code = code.context("code was not created")?;
        let code = unsafe {
            std::slice::from_raw_parts(code.getBufferPointer().cast::<u8>(), code.getBufferSize())
        };
        Ok(code.to_vec())
    }

    /// Compiles the specified entry point into a blob.
    ///
    /// The `target_index` is the target's index in
    /// [`SessionDescriptor::targets`].
    ///
    /// [`SessionDescriptor::targets`]: crate::SessionDescriptor::targets
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

        let layout = self.get_layout()?;
        let entry_point_count = layout.entry_point_count();
        anyhow::ensure!(
            entry_point_index < entry_point_count,
            "entry point index is out of range (component type has \
            {entry_point_count} entry points)",
        );

        let mut diagnostics = None;
        let mut code = None;
        let result = unsafe {
            self.inner.getEntryPointCode(
                entry_point_index as i64,
                target_index as i64,
                &mut code,
                &mut diagnostics,
            )
        };
        self.ctx.log_diagnostics(diagnostics);
        result?;

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
            session: &self.session,
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
