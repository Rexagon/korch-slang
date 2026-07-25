use crate::{AsBoxedComponentType, BoxedComponentTypeRef, SlangContext, com};

pub struct TypeConformance {
    pub(crate) inner: com::ITypeConformance,
    pub(crate) ctx: SlangContext,
}

impl AsBoxedComponentType for TypeConformance {
    fn as_boxed(&self) -> super::BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            ctx: &self.ctx,
        }
    }
}

impl AsRef<com::ITypeConformance> for TypeConformance {
    fn as_ref(&self) -> &com::ITypeConformance {
        &self.inner
    }
}
