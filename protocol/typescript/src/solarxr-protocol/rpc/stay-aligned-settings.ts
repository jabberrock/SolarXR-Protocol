// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class StayAlignedSettings implements flatbuffers.IUnpackableObject<StayAlignedSettingsT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):StayAlignedSettings {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsStayAlignedSettings(bb:flatbuffers.ByteBuffer, obj?:StayAlignedSettings):StayAlignedSettings {
  return (obj || new StayAlignedSettings()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsStayAlignedSettings(bb:flatbuffers.ByteBuffer, obj?:StayAlignedSettings):StayAlignedSettings {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new StayAlignedSettings()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

enabled():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

extraYawCorrection():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

hideYawCorrection():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

standingEnabled():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

standingUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

standingLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

standingFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingEnabled():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 18);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

sittingUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 20);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 22);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 24);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatEnabled():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 26);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

flatUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 28);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 30);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 32);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

static startStayAlignedSettings(builder:flatbuffers.Builder) {
  builder.startObject(15);
}

static addEnabled(builder:flatbuffers.Builder, enabled:boolean) {
  builder.addFieldInt8(0, +enabled, +false);
}

static addExtraYawCorrection(builder:flatbuffers.Builder, extraYawCorrection:boolean) {
  builder.addFieldInt8(1, +extraYawCorrection, +false);
}

static addHideYawCorrection(builder:flatbuffers.Builder, hideYawCorrection:boolean) {
  builder.addFieldInt8(2, +hideYawCorrection, +false);
}

static addStandingEnabled(builder:flatbuffers.Builder, standingEnabled:boolean) {
  builder.addFieldInt8(3, +standingEnabled, +false);
}

static addStandingUpperLegAngle(builder:flatbuffers.Builder, standingUpperLegAngle:number) {
  builder.addFieldFloat32(4, standingUpperLegAngle, 0.0);
}

static addStandingLowerLegAngle(builder:flatbuffers.Builder, standingLowerLegAngle:number) {
  builder.addFieldFloat32(5, standingLowerLegAngle, 0.0);
}

static addStandingFootAngle(builder:flatbuffers.Builder, standingFootAngle:number) {
  builder.addFieldFloat32(6, standingFootAngle, 0.0);
}

static addSittingEnabled(builder:flatbuffers.Builder, sittingEnabled:boolean) {
  builder.addFieldInt8(7, +sittingEnabled, +false);
}

static addSittingUpperLegAngle(builder:flatbuffers.Builder, sittingUpperLegAngle:number) {
  builder.addFieldFloat32(8, sittingUpperLegAngle, 0.0);
}

static addSittingLowerLegAngle(builder:flatbuffers.Builder, sittingLowerLegAngle:number) {
  builder.addFieldFloat32(9, sittingLowerLegAngle, 0.0);
}

static addSittingFootAngle(builder:flatbuffers.Builder, sittingFootAngle:number) {
  builder.addFieldFloat32(10, sittingFootAngle, 0.0);
}

static addFlatEnabled(builder:flatbuffers.Builder, flatEnabled:boolean) {
  builder.addFieldInt8(11, +flatEnabled, +false);
}

static addFlatUpperLegAngle(builder:flatbuffers.Builder, flatUpperLegAngle:number) {
  builder.addFieldFloat32(12, flatUpperLegAngle, 0.0);
}

static addFlatLowerLegAngle(builder:flatbuffers.Builder, flatLowerLegAngle:number) {
  builder.addFieldFloat32(13, flatLowerLegAngle, 0.0);
}

static addFlatFootAngle(builder:flatbuffers.Builder, flatFootAngle:number) {
  builder.addFieldFloat32(14, flatFootAngle, 0.0);
}

static endStayAlignedSettings(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createStayAlignedSettings(builder:flatbuffers.Builder, enabled:boolean, extraYawCorrection:boolean, hideYawCorrection:boolean, standingEnabled:boolean, standingUpperLegAngle:number, standingLowerLegAngle:number, standingFootAngle:number, sittingEnabled:boolean, sittingUpperLegAngle:number, sittingLowerLegAngle:number, sittingFootAngle:number, flatEnabled:boolean, flatUpperLegAngle:number, flatLowerLegAngle:number, flatFootAngle:number):flatbuffers.Offset {
  StayAlignedSettings.startStayAlignedSettings(builder);
  StayAlignedSettings.addEnabled(builder, enabled);
  StayAlignedSettings.addExtraYawCorrection(builder, extraYawCorrection);
  StayAlignedSettings.addHideYawCorrection(builder, hideYawCorrection);
  StayAlignedSettings.addStandingEnabled(builder, standingEnabled);
  StayAlignedSettings.addStandingUpperLegAngle(builder, standingUpperLegAngle);
  StayAlignedSettings.addStandingLowerLegAngle(builder, standingLowerLegAngle);
  StayAlignedSettings.addStandingFootAngle(builder, standingFootAngle);
  StayAlignedSettings.addSittingEnabled(builder, sittingEnabled);
  StayAlignedSettings.addSittingUpperLegAngle(builder, sittingUpperLegAngle);
  StayAlignedSettings.addSittingLowerLegAngle(builder, sittingLowerLegAngle);
  StayAlignedSettings.addSittingFootAngle(builder, sittingFootAngle);
  StayAlignedSettings.addFlatEnabled(builder, flatEnabled);
  StayAlignedSettings.addFlatUpperLegAngle(builder, flatUpperLegAngle);
  StayAlignedSettings.addFlatLowerLegAngle(builder, flatLowerLegAngle);
  StayAlignedSettings.addFlatFootAngle(builder, flatFootAngle);
  return StayAlignedSettings.endStayAlignedSettings(builder);
}

unpack(): StayAlignedSettingsT {
  return new StayAlignedSettingsT(
    this.enabled(),
    this.extraYawCorrection(),
    this.hideYawCorrection(),
    this.standingEnabled(),
    this.standingUpperLegAngle(),
    this.standingLowerLegAngle(),
    this.standingFootAngle(),
    this.sittingEnabled(),
    this.sittingUpperLegAngle(),
    this.sittingLowerLegAngle(),
    this.sittingFootAngle(),
    this.flatEnabled(),
    this.flatUpperLegAngle(),
    this.flatLowerLegAngle(),
    this.flatFootAngle()
  );
}


unpackTo(_o: StayAlignedSettingsT): void {
  _o.enabled = this.enabled();
  _o.extraYawCorrection = this.extraYawCorrection();
  _o.hideYawCorrection = this.hideYawCorrection();
  _o.standingEnabled = this.standingEnabled();
  _o.standingUpperLegAngle = this.standingUpperLegAngle();
  _o.standingLowerLegAngle = this.standingLowerLegAngle();
  _o.standingFootAngle = this.standingFootAngle();
  _o.sittingEnabled = this.sittingEnabled();
  _o.sittingUpperLegAngle = this.sittingUpperLegAngle();
  _o.sittingLowerLegAngle = this.sittingLowerLegAngle();
  _o.sittingFootAngle = this.sittingFootAngle();
  _o.flatEnabled = this.flatEnabled();
  _o.flatUpperLegAngle = this.flatUpperLegAngle();
  _o.flatLowerLegAngle = this.flatLowerLegAngle();
  _o.flatFootAngle = this.flatFootAngle();
}
}

export class StayAlignedSettingsT implements flatbuffers.IGeneratedObject {
constructor(
  public enabled: boolean = false,
  public extraYawCorrection: boolean = false,
  public hideYawCorrection: boolean = false,
  public standingEnabled: boolean = false,
  public standingUpperLegAngle: number = 0.0,
  public standingLowerLegAngle: number = 0.0,
  public standingFootAngle: number = 0.0,
  public sittingEnabled: boolean = false,
  public sittingUpperLegAngle: number = 0.0,
  public sittingLowerLegAngle: number = 0.0,
  public sittingFootAngle: number = 0.0,
  public flatEnabled: boolean = false,
  public flatUpperLegAngle: number = 0.0,
  public flatLowerLegAngle: number = 0.0,
  public flatFootAngle: number = 0.0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return StayAlignedSettings.createStayAlignedSettings(builder,
    this.enabled,
    this.extraYawCorrection,
    this.hideYawCorrection,
    this.standingEnabled,
    this.standingUpperLegAngle,
    this.standingLowerLegAngle,
    this.standingFootAngle,
    this.sittingEnabled,
    this.sittingUpperLegAngle,
    this.sittingLowerLegAngle,
    this.sittingFootAngle,
    this.flatEnabled,
    this.flatUpperLegAngle,
    this.flatLowerLegAngle,
    this.flatFootAngle
  );
}
}
