use anyhow::{Context, Result};

use crate::{AsBoxedComponentType, BoxedComponentTypeRef, Session, SlangContext, TypeLayout, com};

pub struct TypeConformanceDescriptor<'a> {
    pub interface: TypeLayout<'a>,
    pub ty: TypeLayout<'a>,
    pub override_id: Option<u64>,
}

pub struct TypeConformance {
    pub(crate) inner: com::ITypeConformance,
    pub(crate) session: com::ISession,
    pub(crate) ctx: SlangContext,
}

impl TypeConformance {
    pub fn new(session: &Session, desc: &TypeConformanceDescriptor<'_>) -> Result<Self> {
        let override_id = match desc.override_id {
            Some(id) => {
                anyhow::ensure!(id < i64::MAX as u64, "invalid override id");
                id as i64
            }
            None => -1,
        };

        let mut diagnostics = None;
        let mut conformance = None;
        let result = unsafe {
            session.inner.createTypeConformanceComponentType(
                desc.ty.inner.as_ptr(),
                desc.interface.inner.as_ptr(),
                &mut conformance,
                override_id,
                &mut diagnostics,
            )
        };
        session.ctx.log_diagnostics(diagnostics);
        result?;

        Ok(TypeConformance {
            inner: conformance.context("type conformance was not created")?,
            session: session.inner.clone(),
            ctx: session.ctx.clone(),
        })
    }
}

impl AsBoxedComponentType for TypeConformance {
    fn as_boxed(&self) -> super::BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            session: &self.session,
            ctx: &self.ctx,
        }
    }
}
