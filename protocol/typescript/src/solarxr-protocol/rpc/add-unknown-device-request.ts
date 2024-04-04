// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class AddUnknownDeviceRequest implements flatbuffers.IUnpackableObject<AddUnknownDeviceRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):AddUnknownDeviceRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsAddUnknownDeviceRequest(bb:flatbuffers.ByteBuffer, obj?:AddUnknownDeviceRequest):AddUnknownDeviceRequest {
  return (obj || new AddUnknownDeviceRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsAddUnknownDeviceRequest(bb:flatbuffers.ByteBuffer, obj?:AddUnknownDeviceRequest):AddUnknownDeviceRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new AddUnknownDeviceRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

macAddress():string|null
macAddress(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
macAddress(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startAddUnknownDeviceRequest(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addMacAddress(builder:flatbuffers.Builder, macAddressOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, macAddressOffset, 0);
}

static endAddUnknownDeviceRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createAddUnknownDeviceRequest(builder:flatbuffers.Builder, macAddressOffset:flatbuffers.Offset):flatbuffers.Offset {
  AddUnknownDeviceRequest.startAddUnknownDeviceRequest(builder);
  AddUnknownDeviceRequest.addMacAddress(builder, macAddressOffset);
  return AddUnknownDeviceRequest.endAddUnknownDeviceRequest(builder);
}

unpack(): AddUnknownDeviceRequestT {
  return new AddUnknownDeviceRequestT(
    this.macAddress()
  );
}


unpackTo(_o: AddUnknownDeviceRequestT): void {
  _o.macAddress = this.macAddress();
}
}

export class AddUnknownDeviceRequestT implements flatbuffers.IGeneratedObject {
constructor(
  public macAddress: string|Uint8Array|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const macAddress = (this.macAddress !== null ? builder.createString(this.macAddress!) : 0);

  return AddUnknownDeviceRequest.createAddUnknownDeviceRequest(builder,
    macAddress
  );
}
}