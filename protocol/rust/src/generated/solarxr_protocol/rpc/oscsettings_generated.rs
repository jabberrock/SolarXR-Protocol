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
pub enum OSCSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

/// OSC Settings that are used in *any* osc application.
pub struct OSCSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OSCSettings<'a> {
  type Inner = OSCSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> OSCSettings<'a> {
  pub const VT_ENABLED: flatbuffers::VOffsetT = 4;
  pub const VT_PORT_IN: flatbuffers::VOffsetT = 6;
  pub const VT_PORT_OUT: flatbuffers::VOffsetT = 8;
  pub const VT_ADDRESS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    OSCSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args OSCSettingsArgs<'args>
  ) -> flatbuffers::WIPOffset<OSCSettings<'bldr>> {
    let mut builder = OSCSettingsBuilder::new(_fbb);
    if let Some(x) = args.address { builder.add_address(x); }
    builder.add_port_out(args.port_out);
    builder.add_port_in(args.port_in);
    builder.add_enabled(args.enabled);
    builder.finish()
  }


  #[inline]
  pub fn enabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(OSCSettings::VT_ENABLED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn port_in(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(OSCSettings::VT_PORT_IN, Some(0)).unwrap()}
  }
  #[inline]
  pub fn port_out(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(OSCSettings::VT_PORT_OUT, Some(0)).unwrap()}
  }
  #[inline]
  pub fn address(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(OSCSettings::VT_ADDRESS, None)}
  }
}

impl flatbuffers::Verifiable for OSCSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("enabled", Self::VT_ENABLED, false)?
     .visit_field::<u16>("port_in", Self::VT_PORT_IN, false)?
     .visit_field::<u16>("port_out", Self::VT_PORT_OUT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("address", Self::VT_ADDRESS, false)?
     .finish();
    Ok(())
  }
}
pub struct OSCSettingsArgs<'a> {
    pub enabled: bool,
    pub port_in: u16,
    pub port_out: u16,
    pub address: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for OSCSettingsArgs<'a> {
  #[inline]
  fn default() -> Self {
    OSCSettingsArgs {
      enabled: false,
      port_in: 0,
      port_out: 0,
      address: None,
    }
  }
}

pub struct OSCSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OSCSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_enabled(&mut self, enabled: bool) {
    self.fbb_.push_slot::<bool>(OSCSettings::VT_ENABLED, enabled, false);
  }
  #[inline]
  pub fn add_port_in(&mut self, port_in: u16) {
    self.fbb_.push_slot::<u16>(OSCSettings::VT_PORT_IN, port_in, 0);
  }
  #[inline]
  pub fn add_port_out(&mut self, port_out: u16) {
    self.fbb_.push_slot::<u16>(OSCSettings::VT_PORT_OUT, port_out, 0);
  }
  #[inline]
  pub fn add_address(&mut self, address: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(OSCSettings::VT_ADDRESS, address);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OSCSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OSCSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OSCSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for OSCSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("OSCSettings");
      ds.field("enabled", &self.enabled());
      ds.field("port_in", &self.port_in());
      ds.field("port_out", &self.port_out());
      ds.field("address", &self.address());
      ds.finish()
  }
}
