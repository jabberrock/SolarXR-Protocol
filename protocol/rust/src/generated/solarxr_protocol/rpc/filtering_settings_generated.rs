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
pub enum FilteringSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FilteringSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FilteringSettings<'a> {
  type Inner = FilteringSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> FilteringSettings<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_AMOUNT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    FilteringSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args FilteringSettingsArgs
  ) -> flatbuffers::WIPOffset<FilteringSettings<'bldr>> {
    let mut builder = FilteringSettingsBuilder::new(_fbb);
    builder.add_amount(args.amount);
    builder.add_type_(args.type_);
    builder.finish()
  }


  #[inline]
  pub fn type_(&self) -> super::datatypes::FilteringType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::datatypes::FilteringType>(FilteringSettings::VT_TYPE_, Some(super::datatypes::FilteringType::NONE)).unwrap()}
  }
  /// 0 to 1. A higher value results in more smoothing or prediction
  #[inline]
  pub fn amount(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(FilteringSettings::VT_AMOUNT, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for FilteringSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::datatypes::FilteringType>("type_", Self::VT_TYPE_, false)?
     .visit_field::<f32>("amount", Self::VT_AMOUNT, false)?
     .finish();
    Ok(())
  }
}
pub struct FilteringSettingsArgs {
    pub type_: super::datatypes::FilteringType,
    pub amount: f32,
}
impl<'a> Default for FilteringSettingsArgs {
  #[inline]
  fn default() -> Self {
    FilteringSettingsArgs {
      type_: super::datatypes::FilteringType::NONE,
      amount: 0.0,
    }
  }
}

pub struct FilteringSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FilteringSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_type_(&mut self, type_: super::datatypes::FilteringType) {
    self.fbb_.push_slot::<super::datatypes::FilteringType>(FilteringSettings::VT_TYPE_, type_, super::datatypes::FilteringType::NONE);
  }
  #[inline]
  pub fn add_amount(&mut self, amount: f32) {
    self.fbb_.push_slot::<f32>(FilteringSettings::VT_AMOUNT, amount, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FilteringSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FilteringSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FilteringSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for FilteringSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("FilteringSettings");
      ds.field("type_", &self.type_());
      ds.field("amount", &self.amount());
      ds.finish()
  }
}
