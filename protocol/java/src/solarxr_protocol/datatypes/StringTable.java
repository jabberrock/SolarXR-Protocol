// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class StringTable extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static StringTable getRootAsStringTable(ByteBuffer _bb) { return getRootAsStringTable(_bb, new StringTable()); }
  public static StringTable getRootAsStringTable(ByteBuffer _bb, StringTable obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public StringTable __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public String s() { int o = __offset(4); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer sAsByteBuffer() { return __vector_as_bytebuffer(4, 1); }
  public ByteBuffer sInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 4, 1); }

  public static int createStringTable(FlatBufferBuilder builder,
      int sOffset) {
    builder.startTable(1);
    StringTable.addS(builder, sOffset);
    return StringTable.endStringTable(builder);
  }

  public static void startStringTable(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addS(FlatBufferBuilder builder, int sOffset) { builder.addOffset(0, sOffset, 0); }
  public static int endStringTable(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public StringTable get(int j) { return get(new StringTable(), j); }
    public StringTable get(StringTable obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public StringTableT unpack() {
    StringTableT _o = new StringTableT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(StringTableT _o) {
    String _oS = s();
    _o.setS(_oS);
  }
  public static int pack(FlatBufferBuilder builder, StringTableT _o) {
    if (_o == null) return 0;
    int _s = _o.getS() == null ? 0 : builder.createString(_o.getS());
    return createStringTable(
      builder,
      _s);
  }
}

