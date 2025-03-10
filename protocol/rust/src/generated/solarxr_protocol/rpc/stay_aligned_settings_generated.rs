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
pub enum StayAlignedSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StayAlignedSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StayAlignedSettings<'a> {
  type Inner = StayAlignedSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> StayAlignedSettings<'a> {
  pub const VT_ENABLED: flatbuffers::VOffsetT = 4;
  pub const VT_EXTRAYAWCORRECTION: flatbuffers::VOffsetT = 6;
  pub const VT_HIDEYAWCORRECTION: flatbuffers::VOffsetT = 8;
  pub const VT_STANDINGENABLED: flatbuffers::VOffsetT = 10;
  pub const VT_STANDINGUPPERLEGANGLE: flatbuffers::VOffsetT = 12;
  pub const VT_STANDINGLOWERLEGANGLE: flatbuffers::VOffsetT = 14;
  pub const VT_STANDINGFOOTANGLE: flatbuffers::VOffsetT = 16;
  pub const VT_SITTINGENABLED: flatbuffers::VOffsetT = 18;
  pub const VT_SITTINGUPPERLEGANGLE: flatbuffers::VOffsetT = 20;
  pub const VT_SITTINGLOWERLEGANGLE: flatbuffers::VOffsetT = 22;
  pub const VT_SITTINGFOOTANGLE: flatbuffers::VOffsetT = 24;
  pub const VT_FLATENABLED: flatbuffers::VOffsetT = 26;
  pub const VT_FLATUPPERLEGANGLE: flatbuffers::VOffsetT = 28;
  pub const VT_FLATLOWERLEGANGLE: flatbuffers::VOffsetT = 30;
  pub const VT_FLATFOOTANGLE: flatbuffers::VOffsetT = 32;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    StayAlignedSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args StayAlignedSettingsArgs
  ) -> flatbuffers::WIPOffset<StayAlignedSettings<'bldr>> {
    let mut builder = StayAlignedSettingsBuilder::new(_fbb);
    builder.add_flatFootAngle(args.flatFootAngle);
    builder.add_flatLowerLegAngle(args.flatLowerLegAngle);
    builder.add_flatUpperLegAngle(args.flatUpperLegAngle);
    builder.add_sittingFootAngle(args.sittingFootAngle);
    builder.add_sittingLowerLegAngle(args.sittingLowerLegAngle);
    builder.add_sittingUpperLegAngle(args.sittingUpperLegAngle);
    builder.add_standingFootAngle(args.standingFootAngle);
    builder.add_standingLowerLegAngle(args.standingLowerLegAngle);
    builder.add_standingUpperLegAngle(args.standingUpperLegAngle);
    builder.add_flatEnabled(args.flatEnabled);
    builder.add_sittingEnabled(args.sittingEnabled);
    builder.add_standingEnabled(args.standingEnabled);
    builder.add_hideYawCorrection(args.hideYawCorrection);
    builder.add_extraYawCorrection(args.extraYawCorrection);
    builder.add_enabled(args.enabled);
    builder.finish()
  }


  #[inline]
  pub fn enabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_ENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn extraYawCorrection(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_EXTRAYAWCORRECTION, Some(false)).unwrap()}
  }
  #[inline]
  pub fn hideYawCorrection(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_HIDEYAWCORRECTION, Some(false)).unwrap()}
  }
  #[inline]
  pub fn standingEnabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_STANDINGENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn standingUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_STANDINGUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn standingLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_STANDINGLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn standingFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_STANDINGFOOTANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingEnabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_SITTINGENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn sittingUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_SITTINGUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_SITTINGLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn sittingFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_SITTINGFOOTANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatEnabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(StayAlignedSettings::VT_FLATENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn flatUpperLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_FLATUPPERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatLowerLegAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_FLATLOWERLEGANGLE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn flatFootAngle(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(StayAlignedSettings::VT_FLATFOOTANGLE, Some(0.0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for StayAlignedSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("enabled", Self::VT_ENABLED, false)?
     .visit_field::<bool>("extraYawCorrection", Self::VT_EXTRAYAWCORRECTION, false)?
     .visit_field::<bool>("hideYawCorrection", Self::VT_HIDEYAWCORRECTION, false)?
     .visit_field::<bool>("standingEnabled", Self::VT_STANDINGENABLED, false)?
     .visit_field::<f32>("standingUpperLegAngle", Self::VT_STANDINGUPPERLEGANGLE, false)?
     .visit_field::<f32>("standingLowerLegAngle", Self::VT_STANDINGLOWERLEGANGLE, false)?
     .visit_field::<f32>("standingFootAngle", Self::VT_STANDINGFOOTANGLE, false)?
     .visit_field::<bool>("sittingEnabled", Self::VT_SITTINGENABLED, false)?
     .visit_field::<f32>("sittingUpperLegAngle", Self::VT_SITTINGUPPERLEGANGLE, false)?
     .visit_field::<f32>("sittingLowerLegAngle", Self::VT_SITTINGLOWERLEGANGLE, false)?
     .visit_field::<f32>("sittingFootAngle", Self::VT_SITTINGFOOTANGLE, false)?
     .visit_field::<bool>("flatEnabled", Self::VT_FLATENABLED, false)?
     .visit_field::<f32>("flatUpperLegAngle", Self::VT_FLATUPPERLEGANGLE, false)?
     .visit_field::<f32>("flatLowerLegAngle", Self::VT_FLATLOWERLEGANGLE, false)?
     .visit_field::<f32>("flatFootAngle", Self::VT_FLATFOOTANGLE, false)?
     .finish();
    Ok(())
  }
}
pub struct StayAlignedSettingsArgs {
    pub enabled: bool,
    pub extraYawCorrection: bool,
    pub hideYawCorrection: bool,
    pub standingEnabled: bool,
    pub standingUpperLegAngle: f32,
    pub standingLowerLegAngle: f32,
    pub standingFootAngle: f32,
    pub sittingEnabled: bool,
    pub sittingUpperLegAngle: f32,
    pub sittingLowerLegAngle: f32,
    pub sittingFootAngle: f32,
    pub flatEnabled: bool,
    pub flatUpperLegAngle: f32,
    pub flatLowerLegAngle: f32,
    pub flatFootAngle: f32,
}
impl<'a> Default for StayAlignedSettingsArgs {
  #[inline]
  fn default() -> Self {
    StayAlignedSettingsArgs {
      enabled: false,
      extraYawCorrection: false,
      hideYawCorrection: false,
      standingEnabled: false,
      standingUpperLegAngle: 0.0,
      standingLowerLegAngle: 0.0,
      standingFootAngle: 0.0,
      sittingEnabled: false,
      sittingUpperLegAngle: 0.0,
      sittingLowerLegAngle: 0.0,
      sittingFootAngle: 0.0,
      flatEnabled: false,
      flatUpperLegAngle: 0.0,
      flatLowerLegAngle: 0.0,
      flatFootAngle: 0.0,
    }
  }
}

pub struct StayAlignedSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> StayAlignedSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_enabled(&mut self, enabled: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_ENABLED, enabled, false);
  }
  #[inline]
  pub fn add_extraYawCorrection(&mut self, extraYawCorrection: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_EXTRAYAWCORRECTION, extraYawCorrection, false);
  }
  #[inline]
  pub fn add_hideYawCorrection(&mut self, hideYawCorrection: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_HIDEYAWCORRECTION, hideYawCorrection, false);
  }
  #[inline]
  pub fn add_standingEnabled(&mut self, standingEnabled: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_STANDINGENABLED, standingEnabled, false);
  }
  #[inline]
  pub fn add_standingUpperLegAngle(&mut self, standingUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_STANDINGUPPERLEGANGLE, standingUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_standingLowerLegAngle(&mut self, standingLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_STANDINGLOWERLEGANGLE, standingLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_standingFootAngle(&mut self, standingFootAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_STANDINGFOOTANGLE, standingFootAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingEnabled(&mut self, sittingEnabled: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_SITTINGENABLED, sittingEnabled, false);
  }
  #[inline]
  pub fn add_sittingUpperLegAngle(&mut self, sittingUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_SITTINGUPPERLEGANGLE, sittingUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingLowerLegAngle(&mut self, sittingLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_SITTINGLOWERLEGANGLE, sittingLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_sittingFootAngle(&mut self, sittingFootAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_SITTINGFOOTANGLE, sittingFootAngle, 0.0);
  }
  #[inline]
  pub fn add_flatEnabled(&mut self, flatEnabled: bool) {
    self.fbb_.push_slot::<bool>(StayAlignedSettings::VT_FLATENABLED, flatEnabled, false);
  }
  #[inline]
  pub fn add_flatUpperLegAngle(&mut self, flatUpperLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_FLATUPPERLEGANGLE, flatUpperLegAngle, 0.0);
  }
  #[inline]
  pub fn add_flatLowerLegAngle(&mut self, flatLowerLegAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_FLATLOWERLEGANGLE, flatLowerLegAngle, 0.0);
  }
  #[inline]
  pub fn add_flatFootAngle(&mut self, flatFootAngle: f32) {
    self.fbb_.push_slot::<f32>(StayAlignedSettings::VT_FLATFOOTANGLE, flatFootAngle, 0.0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> StayAlignedSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    StayAlignedSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<StayAlignedSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for StayAlignedSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("StayAlignedSettings");
      ds.field("enabled", &self.enabled());
      ds.field("extraYawCorrection", &self.extraYawCorrection());
      ds.field("hideYawCorrection", &self.hideYawCorrection());
      ds.field("standingEnabled", &self.standingEnabled());
      ds.field("standingUpperLegAngle", &self.standingUpperLegAngle());
      ds.field("standingLowerLegAngle", &self.standingLowerLegAngle());
      ds.field("standingFootAngle", &self.standingFootAngle());
      ds.field("sittingEnabled", &self.sittingEnabled());
      ds.field("sittingUpperLegAngle", &self.sittingUpperLegAngle());
      ds.field("sittingLowerLegAngle", &self.sittingLowerLegAngle());
      ds.field("sittingFootAngle", &self.sittingFootAngle());
      ds.field("flatEnabled", &self.flatEnabled());
      ds.field("flatUpperLegAngle", &self.flatUpperLegAngle());
      ds.field("flatLowerLegAngle", &self.flatLowerLegAngle());
      ds.field("flatFootAngle", &self.flatFootAngle());
      ds.finish()
  }
}
