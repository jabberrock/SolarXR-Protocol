// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.data_feed.stay_aligned;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class StayAlignedPose extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static StayAlignedPose getRootAsStayAlignedPose(ByteBuffer _bb) { return getRootAsStayAlignedPose(_bb, new StayAlignedPose()); }
  public static StayAlignedPose getRootAsStayAlignedPose(ByteBuffer _bb, StayAlignedPose obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public StayAlignedPose __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public float upperLegAngleInDeg() { int o = __offset(4); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }
  public float lowerLegAngleInDeg() { int o = __offset(6); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }
  public float footAngleInDeg() { int o = __offset(8); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }

  public static int createStayAlignedPose(FlatBufferBuilder builder,
      float upperLegAngleInDeg,
      float lowerLegAngleInDeg,
      float footAngleInDeg) {
    builder.startTable(3);
    StayAlignedPose.addFootAngleInDeg(builder, footAngleInDeg);
    StayAlignedPose.addLowerLegAngleInDeg(builder, lowerLegAngleInDeg);
    StayAlignedPose.addUpperLegAngleInDeg(builder, upperLegAngleInDeg);
    return StayAlignedPose.endStayAlignedPose(builder);
  }

  public static void startStayAlignedPose(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addUpperLegAngleInDeg(FlatBufferBuilder builder, float upperLegAngleInDeg) { builder.addFloat(0, upperLegAngleInDeg, 0.0f); }
  public static void addLowerLegAngleInDeg(FlatBufferBuilder builder, float lowerLegAngleInDeg) { builder.addFloat(1, lowerLegAngleInDeg, 0.0f); }
  public static void addFootAngleInDeg(FlatBufferBuilder builder, float footAngleInDeg) { builder.addFloat(2, footAngleInDeg, 0.0f); }
  public static int endStayAlignedPose(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public StayAlignedPose get(int j) { return get(new StayAlignedPose(), j); }
    public StayAlignedPose get(StayAlignedPose obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public StayAlignedPoseT unpack() {
    StayAlignedPoseT _o = new StayAlignedPoseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(StayAlignedPoseT _o) {
    float _oUpperLegAngleInDeg = upperLegAngleInDeg();
    _o.setUpperLegAngleInDeg(_oUpperLegAngleInDeg);
    float _oLowerLegAngleInDeg = lowerLegAngleInDeg();
    _o.setLowerLegAngleInDeg(_oLowerLegAngleInDeg);
    float _oFootAngleInDeg = footAngleInDeg();
    _o.setFootAngleInDeg(_oFootAngleInDeg);
  }
  public static int pack(FlatBufferBuilder builder, StayAlignedPoseT _o) {
    if (_o == null) return 0;
    return createStayAlignedPose(
      builder,
      _o.getUpperLegAngleInDeg(),
      _o.getLowerLegAngleInDeg(),
      _o.getFootAngleInDeg());
  }
}

