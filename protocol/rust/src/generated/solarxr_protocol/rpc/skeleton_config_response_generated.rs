// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum SkeletonConfigResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SkeletonConfigResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SkeletonConfigResponse<'a> {
  type Inner = SkeletonConfigResponse<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> SkeletonConfigResponse<'a> {
  pub const VT_SKELETON_PARTS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SkeletonConfigResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SkeletonConfigResponseArgs<'args>
  ) -> flatbuffers::WIPOffset<SkeletonConfigResponse<'bldr>> {
    let mut builder = SkeletonConfigResponseBuilder::new(_fbb);
    if let Some(x) = args.skeleton_parts { builder.add_skeleton_parts(x); }
    builder.finish()
  }


  #[inline]
  pub fn skeleton_parts(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart>>>>(SkeletonConfigResponse::VT_SKELETON_PARTS, None)}
  }
}

impl flatbuffers::Verifiable for SkeletonConfigResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<SkeletonPart>>>>("skeleton_parts", Self::VT_SKELETON_PARTS, false)?
     .finish();
    Ok(())
  }
}
pub struct SkeletonConfigResponseArgs<'a> {
    pub skeleton_parts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart<'a>>>>>,
}
impl<'a> Default for SkeletonConfigResponseArgs<'a> {
  #[inline]
  fn default() -> Self {
    SkeletonConfigResponseArgs {
      skeleton_parts: None,
    }
  }
}

pub struct SkeletonConfigResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SkeletonConfigResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_skeleton_parts(&mut self, skeleton_parts: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<SkeletonPart<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SkeletonConfigResponse::VT_SKELETON_PARTS, skeleton_parts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SkeletonConfigResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SkeletonConfigResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SkeletonConfigResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SkeletonConfigResponse<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SkeletonConfigResponse");
      ds.field("skeleton_parts", &self.skeleton_parts());
      ds.finish()
  }
}
