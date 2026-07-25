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

/// A common interface for component types.
pub trait AsBoxedComponentType {
    /// Downcasts the component type to a base reference.
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_>;

    /// Downcasts the component type into a base type.
    fn into_boxed(self) -> BoxedComponentType
    where
        Self: Sized,
    {
        let r = self.as_boxed();
        BoxedComponentType {
            inner: r.inner.clone(),
            session: r.session.clone(),
            ctx: r.ctx.clone(),
        }
    }

    /// Links the component type, ensuring that the composed program has no missing dependencies.
    fn link(&self) -> Result<LinkedModule> {
        let boxed = self.as_boxed();

        let mut linked = None;
        let mut diagnostics = None;
        let result = unsafe { boxed.inner.link(&mut linked, &mut diagnostics) };
        boxed.ctx.log_diagnostics(diagnostics);
        result?;

        Ok(LinkedModule {
            inner: linked.context("components were not linked")?,
            session: boxed.session.clone(),
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
            session: boxed.session.clone(),
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

/// A component type is a unit of shader code layout, reflection, and linking.
///
/// A component type is a unit of shader code that can be included into
/// a linked and compiled shader program. Each component type may have:
///
/// - Zero or more uniform shader parameters, representing textures,
///   buffers, etc. that the code in the component depends on.
/// - Zero or more *specialization* parameters, which are type or
///   value parameters that can be used to synthesize specialized
///   versions of the component type.
/// - Zero or more entry points, which are the individually invocable
///   kernels that can have final code generated.
/// - Zero or more *requirements*, which are other component
///   types on which the component type depends.
///
/// One example of a component type is a module of Slang code:
///
/// - The global-scope shader parameters declared in the module are
///   the parameters when considered as a component type.
/// - Any global-scope generic or interface type parameters introduce
///   specialization parameters for the module.
/// - A module does not by default include any entry points when
///   considered as a component type (although the code of the
///   module might *declare* some entry points).
/// - Any other modules that are `import`ed in the source code
///   become requirements of the module, when considered as a
///   component type.
///
/// An entry point is another example of a component type:
///
/// - The `uniform` parameters of the entry point function are
///   its shader parameters when considered as a component type.
/// - Any generic or interface-type parameters of the entry point
///   introduce specialization parameters.
/// - An entry point component type exposes a single entry point (itself).
///   An entry point has one requirement for the module in which
///   it was defined.
///
/// Component types can be manipulated in a few ways:
///
/// - Multiple component types can be combined into a composite, which
///   combines all of their code, parameters, etc.
/// - A component type can be specialized, by "plugging in" types and
///   values for its specialization parameters.
/// - A component type can be laid out for a particular target, giving
///   offsets/bindings to the shader parameters it contains.
/// - Generated kernel code can be requested for entry points.
#[derive(Clone)]
pub struct BoxedComponentType {
    pub(crate) inner: com::IComponentType,
    pub(crate) session: com::ISession,
    pub(crate) ctx: SlangContext,
}

impl AsBoxedComponentType for BoxedComponentType {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: &self.inner,
            session: &self.session,
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

/// A non-owning version of [`BoxedComponentType`].
#[derive(Clone)]
pub struct BoxedComponentTypeRef<'a> {
    pub(crate) inner: &'a com::IComponentType,
    pub(crate) session: &'a com::ISession,
    pub(crate) ctx: &'a SlangContext,
}

impl AsBoxedComponentType for BoxedComponentTypeRef<'_> {
    fn as_boxed(&self) -> BoxedComponentTypeRef<'_> {
        BoxedComponentTypeRef {
            inner: self.inner,
            session: self.session,
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

/// A specialization argument for [`AsBoxedComponentType::specialize`].
pub enum SpecializationArg<'a> {
    Type(TypeLayout<'a>),
    Expr(Cow<'a, str>),
}

// === Reflection ===

/// Component layout reflection.
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

/// Entry point layout reflection.
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

/// Type layout reflection.
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

/// An iterator over entry points in [`ComponentLayout`].
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
