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
pub enum ResetStayAlignedRelaxedPoseRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ResetStayAlignedRelaxedPoseRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ResetStayAlignedRelaxedPoseRequest<'a> {
  type Inner = ResetStayAlignedRelaxedPoseRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ResetStayAlignedRelaxedPoseRequest<'a> {
  pub const VT_POSE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ResetStayAlignedRelaxedPoseRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ResetStayAlignedRelaxedPoseRequestArgs
  ) -> flatbuffers::WIPOffset<ResetStayAlignedRelaxedPoseRequest<'bldr>> {
    let mut builder = ResetStayAlignedRelaxedPoseRequestBuilder::new(_fbb);
    builder.add_pose(args.pose);
    builder.finish()
  }


  #[inline]
  pub fn pose(&self) -> StayAlignedRelaxedPose {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<StayAlignedRelaxedPose>(ResetStayAlignedRelaxedPoseRequest::VT_POSE, Some(StayAlignedRelaxedPose::STANDING)).unwrap()}
  }
}

impl flatbuffers::Verifiable for ResetStayAlignedRelaxedPoseRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<StayAlignedRelaxedPose>("pose", Self::VT_POSE, false)?
     .finish();
    Ok(())
  }
}
pub struct ResetStayAlignedRelaxedPoseRequestArgs {
    pub pose: StayAlignedRelaxedPose,
}
impl<'a> Default for ResetStayAlignedRelaxedPoseRequestArgs {
  #[inline]
  fn default() -> Self {
    ResetStayAlignedRelaxedPoseRequestArgs {
      pose: StayAlignedRelaxedPose::STANDING,
    }
  }
}

pub struct ResetStayAlignedRelaxedPoseRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ResetStayAlignedRelaxedPoseRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_pose(&mut self, pose: StayAlignedRelaxedPose) {
    self.fbb_.push_slot::<StayAlignedRelaxedPose>(ResetStayAlignedRelaxedPoseRequest::VT_POSE, pose, StayAlignedRelaxedPose::STANDING);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ResetStayAlignedRelaxedPoseRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ResetStayAlignedRelaxedPoseRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ResetStayAlignedRelaxedPoseRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ResetStayAlignedRelaxedPoseRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ResetStayAlignedRelaxedPoseRequest");
      ds.field("pose", &self.pose());
      ds.finish()
  }
}
