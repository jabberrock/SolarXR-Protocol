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
pub enum VRCConfigValuesOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct VRCConfigValues<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for VRCConfigValues<'a> {
  type Inner = VRCConfigValues<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> VRCConfigValues<'a> {
  pub const VT_LEGACY_MODE: flatbuffers::VOffsetT = 4;
  pub const VT_SHOULDER_TRACKING_DISABLED: flatbuffers::VOffsetT = 6;
  pub const VT_USER_HEIGHT: flatbuffers::VOffsetT = 8;
  pub const VT_CALIBRATION_RANGE: flatbuffers::VOffsetT = 10;
  pub const VT_CALIBRATION_VISUALS: flatbuffers::VOffsetT = 12;
  pub const VT_TRACKER_MODEL: flatbuffers::VOffsetT = 14;
  pub const VT_SPINE_MODE: flatbuffers::VOffsetT = 16;
  pub const VT_AVATAR_MEASUREMENT_TYPE: flatbuffers::VOffsetT = 18;
  pub const VT_SHOULDER_WIDTH_COMPENSATION: flatbuffers::VOffsetT = 20;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    VRCConfigValues { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args VRCConfigValuesArgs
  ) -> flatbuffers::WIPOffset<VRCConfigValues<'bldr>> {
    let mut builder = VRCConfigValuesBuilder::new(_fbb);
    builder.add_calibration_range(args.calibration_range);
    builder.add_user_height(args.user_height);
    builder.add_shoulder_width_compensation(args.shoulder_width_compensation);
    builder.add_avatar_measurement_type(args.avatar_measurement_type);
    builder.add_spine_mode(args.spine_mode);
    builder.add_tracker_model(args.tracker_model);
    builder.add_calibration_visuals(args.calibration_visuals);
    builder.add_shoulder_tracking_disabled(args.shoulder_tracking_disabled);
    builder.add_legacy_mode(args.legacy_mode);
    builder.finish()
  }


  #[inline]
  pub fn legacy_mode(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VRCConfigValues::VT_LEGACY_MODE, Some(false)).unwrap()}
  }
  #[inline]
  pub fn shoulder_tracking_disabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VRCConfigValues::VT_SHOULDER_TRACKING_DISABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn user_height(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(VRCConfigValues::VT_USER_HEIGHT, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn calibration_range(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(VRCConfigValues::VT_CALIBRATION_RANGE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn calibration_visuals(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VRCConfigValues::VT_CALIBRATION_VISUALS, Some(false)).unwrap()}
  }
  #[inline]
  pub fn tracker_model(&self) -> VRCTrackerModel {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<VRCTrackerModel>(VRCConfigValues::VT_TRACKER_MODEL, Some(VRCTrackerModel::UNKNOWN)).unwrap()}
  }
  #[inline]
  pub fn spine_mode(&self) -> VRCSpineMode {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<VRCSpineMode>(VRCConfigValues::VT_SPINE_MODE, Some(VRCSpineMode::UNKNOWN)).unwrap()}
  }
  #[inline]
  pub fn avatar_measurement_type(&self) -> VRCAvatarMeasurementType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<VRCAvatarMeasurementType>(VRCConfigValues::VT_AVATAR_MEASUREMENT_TYPE, Some(VRCAvatarMeasurementType::UNKNOWN)).unwrap()}
  }
  #[inline]
  pub fn shoulder_width_compensation(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(VRCConfigValues::VT_SHOULDER_WIDTH_COMPENSATION, Some(false)).unwrap()}
  }
}

impl flatbuffers::Verifiable for VRCConfigValues<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("legacy_mode", Self::VT_LEGACY_MODE, false)?
     .visit_field::<bool>("shoulder_tracking_disabled", Self::VT_SHOULDER_TRACKING_DISABLED, false)?
     .visit_field::<f32>("user_height", Self::VT_USER_HEIGHT, false)?
     .visit_field::<f32>("calibration_range", Self::VT_CALIBRATION_RANGE, false)?
     .visit_field::<bool>("calibration_visuals", Self::VT_CALIBRATION_VISUALS, false)?
     .visit_field::<VRCTrackerModel>("tracker_model", Self::VT_TRACKER_MODEL, false)?
     .visit_field::<VRCSpineMode>("spine_mode", Self::VT_SPINE_MODE, false)?
     .visit_field::<VRCAvatarMeasurementType>("avatar_measurement_type", Self::VT_AVATAR_MEASUREMENT_TYPE, false)?
     .visit_field::<bool>("shoulder_width_compensation", Self::VT_SHOULDER_WIDTH_COMPENSATION, false)?
     .finish();
    Ok(())
  }
}
pub struct VRCConfigValuesArgs {
    pub legacy_mode: bool,
    pub shoulder_tracking_disabled: bool,
    pub user_height: f32,
    pub calibration_range: f32,
    pub calibration_visuals: bool,
    pub tracker_model: VRCTrackerModel,
    pub spine_mode: VRCSpineMode,
    pub avatar_measurement_type: VRCAvatarMeasurementType,
    pub shoulder_width_compensation: bool,
}
impl<'a> Default for VRCConfigValuesArgs {
  #[inline]
  fn default() -> Self {
    VRCConfigValuesArgs {
      legacy_mode: false,
      shoulder_tracking_disabled: false,
      user_height: 0.0,
      calibration_range: 0.0,
      calibration_visuals: false,
      tracker_model: VRCTrackerModel::UNKNOWN,
      spine_mode: VRCSpineMode::UNKNOWN,
      avatar_measurement_type: VRCAvatarMeasurementType::UNKNOWN,
      shoulder_width_compensation: false,
    }
  }
}

pub struct VRCConfigValuesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> VRCConfigValuesBuilder<'a, 'b> {
  #[inline]
  pub fn add_legacy_mode(&mut self, legacy_mode: bool) {
    self.fbb_.push_slot::<bool>(VRCConfigValues::VT_LEGACY_MODE, legacy_mode, false);
  }
  #[inline]
  pub fn add_shoulder_tracking_disabled(&mut self, shoulder_tracking_disabled: bool) {
    self.fbb_.push_slot::<bool>(VRCConfigValues::VT_SHOULDER_TRACKING_DISABLED, shoulder_tracking_disabled, false);
  }
  #[inline]
  pub fn add_user_height(&mut self, user_height: f32) {
    self.fbb_.push_slot::<f32>(VRCConfigValues::VT_USER_HEIGHT, user_height, 0.0);
  }
  #[inline]
  pub fn add_calibration_range(&mut self, calibration_range: f32) {
    self.fbb_.push_slot::<f32>(VRCConfigValues::VT_CALIBRATION_RANGE, calibration_range, 0.0);
  }
  #[inline]
  pub fn add_calibration_visuals(&mut self, calibration_visuals: bool) {
    self.fbb_.push_slot::<bool>(VRCConfigValues::VT_CALIBRATION_VISUALS, calibration_visuals, false);
  }
  #[inline]
  pub fn add_tracker_model(&mut self, tracker_model: VRCTrackerModel) {
    self.fbb_.push_slot::<VRCTrackerModel>(VRCConfigValues::VT_TRACKER_MODEL, tracker_model, VRCTrackerModel::UNKNOWN);
  }
  #[inline]
  pub fn add_spine_mode(&mut self, spine_mode: VRCSpineMode) {
    self.fbb_.push_slot::<VRCSpineMode>(VRCConfigValues::VT_SPINE_MODE, spine_mode, VRCSpineMode::UNKNOWN);
  }
  #[inline]
  pub fn add_avatar_measurement_type(&mut self, avatar_measurement_type: VRCAvatarMeasurementType) {
    self.fbb_.push_slot::<VRCAvatarMeasurementType>(VRCConfigValues::VT_AVATAR_MEASUREMENT_TYPE, avatar_measurement_type, VRCAvatarMeasurementType::UNKNOWN);
  }
  #[inline]
  pub fn add_shoulder_width_compensation(&mut self, shoulder_width_compensation: bool) {
    self.fbb_.push_slot::<bool>(VRCConfigValues::VT_SHOULDER_WIDTH_COMPENSATION, shoulder_width_compensation, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> VRCConfigValuesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    VRCConfigValuesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<VRCConfigValues<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for VRCConfigValues<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("VRCConfigValues");
      ds.field("legacy_mode", &self.legacy_mode());
      ds.field("shoulder_tracking_disabled", &self.shoulder_tracking_disabled());
      ds.field("user_height", &self.user_height());
      ds.field("calibration_range", &self.calibration_range());
      ds.field("calibration_visuals", &self.calibration_visuals());
      ds.field("tracker_model", &self.tracker_model());
      ds.field("spine_mode", &self.spine_mode());
      ds.field("avatar_measurement_type", &self.avatar_measurement_type());
      ds.field("shoulder_width_compensation", &self.shoulder_width_compensation());
      ds.finish()
  }
}
