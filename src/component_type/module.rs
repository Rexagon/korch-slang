use std::path::Path;

use anyhow::{Context, Result};
use windows_core::Interface;

use crate::util::{to_ffi_path, to_ffi_string};
use crate::{AsBoxedComponentType, BoxedComponentTypeRef, EntryPoint, Session, SlangContext, com};

#[derive(Clone)]
pub struct Module {
    inner: com::IModule,
    ctx: SlangContext,
}

impl Module {
    pub fn new(
        session: &Session,
        name: &str,
        path: Option<&Path>,
        source: Vec<u8>,
    ) -> Result<Self> {
        let name = to_ffi_string(name).context("invalid module name")?;
        let path = path
            .map(to_ffi_path)
            .transpose()
            .context("invalid module path")?;

        let blob = com::ISlangBlob::from(source);

        let mut diagnostics = None;
        let module = unsafe {
            session.inner.loadModuleFromSource(
                name.as_ptr(),
                path.as_ref().map(|p| p.as_ptr()).unwrap_or_default(),
                &blob,
                &mut diagnostics,
            )
        };
        session.ctx.log_diagnostics(diagnostics);

        let inner = unsafe { com::IModule::from_raw_borrowed(&module) }
            .context("module was not created")?;

        Ok(Self {
            inner: inner.clone(),
            ctx: session.ctx.clone(),
        })
    }

    pub fn entry_points_iter(&self) -> EntryPointsIter<'_> {
        let count = self.entry_point_count();
        EntryPointsIter {
            module: self,
            next_index: 0,
            count,
        }
    }

    pub fn entry_point_count(&self) -> usize {
        unsafe { self.inner.getDefinedEntryPointCount() as usize }
    }

    pub fn get_entry_point(&self, index: usize) -> Option<EntryPoint> {
        if index > i32::MAX as usize {
            return None;
        }

        let mut entry_point = None;
        let res = unsafe {
            self.inner
                .getDefinedEntryPoint(index as i32, &mut entry_point)
        };
        if res.is_err() {
            return None;
        }

        entry_point.map(|inner| EntryPoint {
            inner,
            ctx: self.ctx.clone(),
        })
    }

    pub fn find_entry_point(&self, name: &str) -> Option<EntryPoint> {
        let name = to_ffi_string(name).ok()?;
        let mut entry_point = None;
        unsafe {
            self.inner
                .findEntryPointByName(name.as_ptr(), &mut entry_point)
                .ok()?
        };
        entry_point.map(|inner| EntryPoint {
            inner,
            ctx: self.ctx.clone(),
        })
    }
}

impl AsBoxedComponentType for Module {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            ctx: &self.ctx,
        }
    }
}

impl AsRef<com::IComponentType> for Module {
    fn as_ref(&self) -> &com::IComponentType {
        &self.inner
    }
}

// === Iter ===

#[derive(Clone)]
pub struct EntryPointsIter<'a> {
    module: &'a Module,
    next_index: usize,
    count: usize,
}

impl Iterator for EntryPointsIter<'_> {
    type Item = EntryPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.count {
            return None;
        }

        let index = self.next_index;
        self.next_index += 1;
        self.module.get_entry_point(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for EntryPointsIter<'_> {
    fn len(&self) -> usize {
        self.count.saturating_sub(self.next_index)
    }
}
