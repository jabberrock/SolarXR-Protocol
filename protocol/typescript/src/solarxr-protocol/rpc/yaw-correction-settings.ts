// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class YawCorrectionSettings implements flatbuffers.IUnpackableObject<YawCorrectionSettingsT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):YawCorrectionSettings {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsYawCorrectionSettings(bb:flatbuffers.ByteBuffer, obj?:YawCorrectionSettings):YawCorrectionSettings {
  return (obj || new YawCorrectionSettings()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsYawCorrectionSettings(bb:flatbuffers.ByteBuffer, obj?:YawCorrectionSettings):YawCorrectionSettings {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new YawCorrectionSettings()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

enabled():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

amountInDegPerSec():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

/**
 * Relaxed body angles
 */
standingUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

standingLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

standingFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

sittingFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 18);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatUpperLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 20);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatLowerLegAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 22);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

flatFootAngle():number {
  const offset = this.bb!.__offset(this.bb_pos, 24);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : 0.0;
}

static startYawCorrectionSettings(builder:flatbuffers.Builder) {
  builder.startObject(11);
}

static addEnabled(builder:flatbuffers.Builder, enabled:boolean) {
  builder.addFieldInt8(0, +enabled, +false);
}

static addAmountInDegPerSec(builder:flatbuffers.Builder, amountInDegPerSec:number) {
  builder.addFieldFloat32(1, amountInDegPerSec, 0.0);
}

static addStandingUpperLegAngle(builder:flatbuffers.Builder, standingUpperLegAngle:number) {
  builder.addFieldFloat32(2, standingUpperLegAngle, 0.0);
}

static addStandingLowerLegAngle(builder:flatbuffers.Builder, standingLowerLegAngle:number) {
  builder.addFieldFloat32(3, standingLowerLegAngle, 0.0);
}

static addStandingFootAngle(builder:flatbuffers.Builder, standingFootAngle:number) {
  builder.addFieldFloat32(4, standingFootAngle, 0.0);
}

static addSittingUpperLegAngle(builder:flatbuffers.Builder, sittingUpperLegAngle:number) {
  builder.addFieldFloat32(5, sittingUpperLegAngle, 0.0);
}

static addSittingLowerLegAngle(builder:flatbuffers.Builder, sittingLowerLegAngle:number) {
  builder.addFieldFloat32(6, sittingLowerLegAngle, 0.0);
}

static addSittingFootAngle(builder:flatbuffers.Builder, sittingFootAngle:number) {
  builder.addFieldFloat32(7, sittingFootAngle, 0.0);
}

static addFlatUpperLegAngle(builder:flatbuffers.Builder, flatUpperLegAngle:number) {
  builder.addFieldFloat32(8, flatUpperLegAngle, 0.0);
}

static addFlatLowerLegAngle(builder:flatbuffers.Builder, flatLowerLegAngle:number) {
  builder.addFieldFloat32(9, flatLowerLegAngle, 0.0);
}

static addFlatFootAngle(builder:flatbuffers.Builder, flatFootAngle:number) {
  builder.addFieldFloat32(10, flatFootAngle, 0.0);
}

static endYawCorrectionSettings(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createYawCorrectionSettings(builder:flatbuffers.Builder, enabled:boolean, amountInDegPerSec:number, standingUpperLegAngle:number, standingLowerLegAngle:number, standingFootAngle:number, sittingUpperLegAngle:number, sittingLowerLegAngle:number, sittingFootAngle:number, flatUpperLegAngle:number, flatLowerLegAngle:number, flatFootAngle:number):flatbuffers.Offset {
  YawCorrectionSettings.startYawCorrectionSettings(builder);
  YawCorrectionSettings.addEnabled(builder, enabled);
  YawCorrectionSettings.addAmountInDegPerSec(builder, amountInDegPerSec);
  YawCorrectionSettings.addStandingUpperLegAngle(builder, standingUpperLegAngle);
  YawCorrectionSettings.addStandingLowerLegAngle(builder, standingLowerLegAngle);
  YawCorrectionSettings.addStandingFootAngle(builder, standingFootAngle);
  YawCorrectionSettings.addSittingUpperLegAngle(builder, sittingUpperLegAngle);
  YawCorrectionSettings.addSittingLowerLegAngle(builder, sittingLowerLegAngle);
  YawCorrectionSettings.addSittingFootAngle(builder, sittingFootAngle);
  YawCorrectionSettings.addFlatUpperLegAngle(builder, flatUpperLegAngle);
  YawCorrectionSettings.addFlatLowerLegAngle(builder, flatLowerLegAngle);
  YawCorrectionSettings.addFlatFootAngle(builder, flatFootAngle);
  return YawCorrectionSettings.endYawCorrectionSettings(builder);
}

unpack(): YawCorrectionSettingsT {
  return new YawCorrectionSettingsT(
    this.enabled(),
    this.amountInDegPerSec(),
    this.standingUpperLegAngle(),
    this.standingLowerLegAngle(),
    this.standingFootAngle(),
    this.sittingUpperLegAngle(),
    this.sittingLowerLegAngle(),
    this.sittingFootAngle(),
    this.flatUpperLegAngle(),
    this.flatLowerLegAngle(),
    this.flatFootAngle()
  );
}


unpackTo(_o: YawCorrectionSettingsT): void {
  _o.enabled = this.enabled();
  _o.amountInDegPerSec = this.amountInDegPerSec();
  _o.standingUpperLegAngle = this.standingUpperLegAngle();
  _o.standingLowerLegAngle = this.standingLowerLegAngle();
  _o.standingFootAngle = this.standingFootAngle();
  _o.sittingUpperLegAngle = this.sittingUpperLegAngle();
  _o.sittingLowerLegAngle = this.sittingLowerLegAngle();
  _o.sittingFootAngle = this.sittingFootAngle();
  _o.flatUpperLegAngle = this.flatUpperLegAngle();
  _o.flatLowerLegAngle = this.flatLowerLegAngle();
  _o.flatFootAngle = this.flatFootAngle();
}
}

export class YawCorrectionSettingsT implements flatbuffers.IGeneratedObject {
constructor(
  public enabled: boolean = false,
  public amountInDegPerSec: number = 0.0,
  public standingUpperLegAngle: number = 0.0,
  public standingLowerLegAngle: number = 0.0,
  public standingFootAngle: number = 0.0,
  public sittingUpperLegAngle: number = 0.0,
  public sittingLowerLegAngle: number = 0.0,
  public sittingFootAngle: number = 0.0,
  public flatUpperLegAngle: number = 0.0,
  public flatLowerLegAngle: number = 0.0,
  public flatFootAngle: number = 0.0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return YawCorrectionSettings.createYawCorrectionSettings(builder,
    this.enabled,
    this.amountInDegPerSec,
    this.standingUpperLegAngle,
    this.standingLowerLegAngle,
    this.standingFootAngle,
    this.sittingUpperLegAngle,
    this.sittingLowerLegAngle,
    this.sittingFootAngle,
    this.flatUpperLegAngle,
    this.flatLowerLegAngle,
    this.flatFootAngle
  );
}
}
