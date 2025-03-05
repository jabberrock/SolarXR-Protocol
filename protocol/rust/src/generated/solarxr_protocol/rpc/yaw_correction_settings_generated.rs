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
pub enum YawCorrectionSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct YawCorrectionSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for YawCorrectionSettings<'a> {
  type Inner = YawCorrectionSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> YawCorrectionSettings<'a> {
  pub const VT_ENABLED: flatbuffers::VOffsetT = 4;
  pub const VT_AMOUNTINDEGPERSEC: flatbuffers::VOffsetT = 6;
  pub const VT_STANDINGUPPERLEGANGLE: flatbuffers::VOffsetT = 8;
  pub const VT_STANDINGLOWERLEGANGLE: flatbuffers::VOffsetT = 10;
  pub const VT_STANDINGFOOTANGLE: flatbuffers::VOffsetT = 12;
  pub const VT_SITTINGUPPERLEGANGLE: flatbuffers::VOffsetT = 14;
  pub const VT_SITTINGLOWERLEGANGLE: flatbuffers::VOffsetT = 16;
  pub const VT_SITTINGFOOTANGLE: flatbuffers::VOffsetT = 18;
  pub const VT_FLATUPPERLEGANGLE: flatbuffers::VOffsetT = 20;
  pub const VT_FLATLOWERLEGANGLE: flatbuffers::VOffsetT = 22;
  pub const VT_FLATFOOTANGLE: flatbuffers::VOffsetT = 24;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    YawCorrectionSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args YawCorrectionSettingsArgs
  ) -> flatbuffers::WIPOffset<YawCorrectionSettings<'bldr>> {
    let mut builder = YawCorrectionSettingsBuilder::new(_fbb);
    builder.add_flatFootAngle(args.flatFootAngle);
    builder.add_flatLowerLegAngle(args.flatLowerLegAngle);
    builder.add_flatUpperLegAngle(args.flatUpperLegAngle);
    builder.add_sittingFootAngle(args.sittingFootAngle);
    builder.add_sittingLowerLegAngle(args.sittingLowerLegAngle);
    builder.add_sittingUpperLegAngle(args.sittingUpperLegAngle);
    builder.add_standingFootAngle(args.standingFootAngle);
    builder.add_standingLowerLegAngle(args.standingLowerLegAngle);
    builder.add_standingUpperLegAngle(args.standingUpperLegAngle);
    builder.add_amountInDegPerSec(args.amountInDegPerSec);
    builder.add_enabled(args.enabled);
    builder.finish()
  }


  #[inline]
  pub fn enabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(YawCorrectionSettings::VT_ENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn amountInDegPerSec(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_AMOUNTINDEGPERSEC, Some(0.0)).unwrap()}
  }
  /// Relaxed body angles
  #[inline]
  pub fn standingUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_STANDINGUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn standingLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_STANDINGLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn standingFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_STANDINGFOOTANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_SITTINGUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_SITTINGLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_SITTINGFOOTANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_FLATUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_FLATLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(YawCorrectionSettings::VT_FLATFOOTANGLE, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for YawCorrectionSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("enabled", Self::VT_ENABLED, false)?
     .visit_field::<f32>("amountInDegPerSec", Self::VT_AMOUNTINDEGPERSEC, false)?
     .visit_field::<f32>("standingUpperLegAngle", Self::VT_STANDINGUPPERLEGANGLE, false)?
     .visit_field::<f32>("standingLowerLegAngle", Self::VT_STANDINGLOWERLEGANGLE, false)?
     .visit_field::<f32>("standingFootAngle", Self::VT_STANDINGFOOTANGLE, false)?
     .visit_field::<f32>("sittingUpperLegAngle", Self::VT_SITTINGUPPERLEGANGLE, false)?
     .visit_field::<f32>("sittingLowerLegAngle", Self::VT_SITTINGLOWERLEGANGLE, false)?
     .visit_field::<f32>("sittingFootAngle", Self::VT_SITTINGFOOTANGLE, false)?
     .visit_field::<f32>("flatUpperLegAngle", Self::VT_FLATUPPERLEGANGLE, false)?
     .visit_field::<f32>("flatLowerLegAngle", Self::VT_FLATLOWERLEGANGLE, false)?
     .visit_field::<f32>("flatFootAngle", Self::VT_FLATFOOTANGLE, false)?
     .finish();
    Ok(())
  }
}
pub struct YawCorrectionSettingsArgs {
    pub enabled: bool,
    pub amountInDegPerSec: f32,
    pub standingUpperLegAngle: f32,
    pub standingLowerLegAngle: f32,
    pub standingFootAngle: f32,
    pub sittingUpperLegAngle: f32,
    pub sittingLowerLegAngle: f32,
    pub sittingFootAngle: f32,
    pub flatUpperLegAngle: f32,
    pub flatLowerLegAngle: f32,
    pub flatFootAngle: f32,
}
impl<'a> Default for YawCorrectionSettingsArgs {
  #[inline]
  fn default() -> Self {
    YawCorrectionSettingsArgs {
      enabled: false,
      amountInDegPerSec: 0.0,
      standingUpperLegAngle: 0.0,
      standingLowerLegAngle: 0.0,
      standingFootAngle: 0.0,
      sittingUpperLegAngle: 0.0,
      sittingLowerLegAngle: 0.0,
      sittingFootAngle: 0.0,
      flatUpperLegAngle: 0.0,
      flatLowerLegAngle: 0.0,
      flatFootAngle: 0.0,
    }
  }
}

pub struct YawCorrectionSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> YawCorrectionSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_enabled(&mut self, enabled: bool) {
    self.fbb_.push_slot::<bool>(YawCorrectionSettings::VT_ENABLED, enabled, false);
  }
  #[inline]
  pub fn add_amountInDegPerSec(&mut self, amountInDegPerSec: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_AMOUNTINDEGPERSEC, amountInDegPerSec, 0.0);
  }
  #[inline]
  pub fn add_standingUpperLegAngle(&mut self, standingUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_STANDINGUPPERLEGANGLE, standingUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_standingLowerLegAngle(&mut self, standingLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_STANDINGLOWERLEGANGLE, standingLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_standingFootAngle(&mut self, standingFootAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_STANDINGFOOTANGLE, standingFootAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingUpperLegAngle(&mut self, sittingUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_SITTINGUPPERLEGANGLE, sittingUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingLowerLegAngle(&mut self, sittingLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_SITTINGLOWERLEGANGLE, sittingLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingFootAngle(&mut self, sittingFootAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_SITTINGFOOTANGLE, sittingFootAngle, 0.0);
  }
  #[inline]
  pub fn add_flatUpperLegAngle(&mut self, flatUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_FLATUPPERLEGANGLE, flatUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_flatLowerLegAngle(&mut self, flatLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_FLATLOWERLEGANGLE, flatLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_flatFootAngle(&mut self, flatFootAngle: f32) {
    self.fbb_.push_slot::<f32>(YawCorrectionSettings::VT_FLATFOOTANGLE, flatFootAngle, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> YawCorrectionSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    YawCorrectionSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<YawCorrectionSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for YawCorrectionSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("YawCorrectionSettings");
      ds.field("enabled", &self.enabled());
      ds.field("amountInDegPerSec", &self.amountInDegPerSec());
      ds.field("standingUpperLegAngle", &self.standingUpperLegAngle());
      ds.field("standingLowerLegAngle", &self.standingLowerLegAngle());
      ds.field("standingFootAngle", &self.standingFootAngle());
      ds.field("sittingUpperLegAngle", &self.sittingUpperLegAngle());
      ds.field("sittingLowerLegAngle", &self.sittingLowerLegAngle());
      ds.field("sittingFootAngle", &self.sittingFootAngle());
      ds.field("flatUpperLegAngle", &self.flatUpperLegAngle());
      ds.field("flatLowerLegAngle", &self.flatLowerLegAngle());
      ds.field("flatFootAngle", &self.flatFootAngle());
      ds.finish()
  }
}
