// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { DeviceDataMask, DeviceDataMaskT } from '../../solarxr-protocol/data-feed/device-data/device-data-mask.js';
import { TrackerDataMask, TrackerDataMaskT } from '../../solarxr-protocol/data-feed/tracker/tracker-data-mask.js';


/**
 * All information related to the configuration of a data feed. This may be sent
 * as part of a `StartFeed`.
 */
export class DataFeedConfig implements flatbuffers.IUnpackableObject<DataFeedConfigT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):DataFeedConfig {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsDataFeedConfig(bb:flatbuffers.ByteBuffer, obj?:DataFeedConfig):DataFeedConfig {
  return (obj || new DataFeedConfig()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsDataFeedConfig(bb:flatbuffers.ByteBuffer, obj?:DataFeedConfig):DataFeedConfig {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new DataFeedConfig()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * Minimum delay in milliseconds between new data updates. This value will be
 * ignored when used for a `PollDataFeed`.
 */
minimumTimeSinceLast():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : 0;
}

dataMask(obj?:DeviceDataMask):DeviceDataMask|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new DeviceDataMask()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

syntheticTrackersMask(obj?:TrackerDataMask):TrackerDataMask|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new TrackerDataMask()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

boneMask():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

stayAlignedPoseMask():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

static startDataFeedConfig(builder:flatbuffers.Builder) {
  builder.startObject(5);
}

static addMinimumTimeSinceLast(builder:flatbuffers.Builder, minimumTimeSinceLast:number) {
  builder.addFieldInt16(0, minimumTimeSinceLast, 0);
}

static addDataMask(builder:flatbuffers.Builder, dataMaskOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, dataMaskOffset, 0);
}

static addSyntheticTrackersMask(builder:flatbuffers.Builder, syntheticTrackersMaskOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, syntheticTrackersMaskOffset, 0);
}

static addBoneMask(builder:flatbuffers.Builder, boneMask:boolean) {
  builder.addFieldInt8(3, +boneMask, +false);
}

static addStayAlignedPoseMask(builder:flatbuffers.Builder, stayAlignedPoseMask:boolean) {
  builder.addFieldInt8(4, +stayAlignedPoseMask, +false);
}

static endDataFeedConfig(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): DataFeedConfigT {
  return new DataFeedConfigT(
    this.minimumTimeSinceLast(),
    (this.dataMask() !== null ? this.dataMask()!.unpack() : null),
    (this.syntheticTrackersMask() !== null ? this.syntheticTrackersMask()!.unpack() : null),
    this.boneMask(),
    this.stayAlignedPoseMask()
  );
}


unpackTo(_o: DataFeedConfigT): void {
  _o.minimumTimeSinceLast = this.minimumTimeSinceLast();
  _o.dataMask = (this.dataMask() !== null ? this.dataMask()!.unpack() : null);
  _o.syntheticTrackersMask = (this.syntheticTrackersMask() !== null ? this.syntheticTrackersMask()!.unpack() : null);
  _o.boneMask = this.boneMask();
  _o.stayAlignedPoseMask = this.stayAlignedPoseMask();
}
}

export class DataFeedConfigT implements flatbuffers.IGeneratedObject {
constructor(
  public minimumTimeSinceLast: number = 0,
  public dataMask: DeviceDataMaskT|null = null,
  public syntheticTrackersMask: TrackerDataMaskT|null = null,
  public boneMask: boolean = false,
  public stayAlignedPoseMask: boolean = false
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const dataMask = (this.dataMask !== null ? this.dataMask!.pack(builder) : 0);
  const syntheticTrackersMask = (this.syntheticTrackersMask !== null ? this.syntheticTrackersMask!.pack(builder) : 0);

  DataFeedConfig.startDataFeedConfig(builder);
  DataFeedConfig.addMinimumTimeSinceLast(builder, this.minimumTimeSinceLast);
  DataFeedConfig.addDataMask(builder, dataMask);
  DataFeedConfig.addSyntheticTrackersMask(builder, syntheticTrackersMask);
  DataFeedConfig.addBoneMask(builder, this.boneMask);
  DataFeedConfig.addStayAlignedPoseMask(builder, this.stayAlignedPoseMask);

  return DataFeedConfig.endDataFeedConfig(builder);
}
}
