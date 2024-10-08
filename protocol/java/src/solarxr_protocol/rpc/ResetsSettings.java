// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class ResetsSettings extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static ResetsSettings getRootAsResetsSettings(ByteBuffer _bb) { return getRootAsResetsSettings(_bb, new ResetsSettings()); }
  public static ResetsSettings getRootAsResetsSettings(ByteBuffer _bb, ResetsSettings obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ResetsSettings __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public boolean resetMountingFeet() { int o = __offset(4); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public int armsMountingResetMode() { int o = __offset(6); return o != 0 ? bb.get(o + bb_pos) & 0xFF : 0; }
  public float yawResetSmoothTime() { int o = __offset(8); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }
  public boolean saveMountingReset() { int o = __offset(10); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean resetHmdPitch() { int o = __offset(12); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }

  public static int createResetsSettings(FlatBufferBuilder builder,
      boolean resetMountingFeet,
      int armsMountingResetMode,
      float yawResetSmoothTime,
      boolean saveMountingReset,
      boolean resetHmdPitch) {
    builder.startTable(5);
    ResetsSettings.addYawResetSmoothTime(builder, yawResetSmoothTime);
    ResetsSettings.addResetHmdPitch(builder, resetHmdPitch);
    ResetsSettings.addSaveMountingReset(builder, saveMountingReset);
    ResetsSettings.addArmsMountingResetMode(builder, armsMountingResetMode);
    ResetsSettings.addResetMountingFeet(builder, resetMountingFeet);
    return ResetsSettings.endResetsSettings(builder);
  }

  public static void startResetsSettings(FlatBufferBuilder builder) { builder.startTable(5); }
  public static void addResetMountingFeet(FlatBufferBuilder builder, boolean resetMountingFeet) { builder.addBoolean(0, resetMountingFeet, false); }
  public static void addArmsMountingResetMode(FlatBufferBuilder builder, int armsMountingResetMode) { builder.addByte(1, (byte) armsMountingResetMode, (byte) 0); }
  public static void addYawResetSmoothTime(FlatBufferBuilder builder, float yawResetSmoothTime) { builder.addFloat(2, yawResetSmoothTime, 0.0f); }
  public static void addSaveMountingReset(FlatBufferBuilder builder, boolean saveMountingReset) { builder.addBoolean(3, saveMountingReset, false); }
  public static void addResetHmdPitch(FlatBufferBuilder builder, boolean resetHmdPitch) { builder.addBoolean(4, resetHmdPitch, false); }
  public static int endResetsSettings(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ResetsSettings get(int j) { return get(new ResetsSettings(), j); }
    public ResetsSettings get(ResetsSettings obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ResetsSettingsT unpack() {
    ResetsSettingsT _o = new ResetsSettingsT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ResetsSettingsT _o) {
    boolean _oResetMountingFeet = resetMountingFeet();
    _o.setResetMountingFeet(_oResetMountingFeet);
    int _oArmsMountingResetMode = armsMountingResetMode();
    _o.setArmsMountingResetMode(_oArmsMountingResetMode);
    float _oYawResetSmoothTime = yawResetSmoothTime();
    _o.setYawResetSmoothTime(_oYawResetSmoothTime);
    boolean _oSaveMountingReset = saveMountingReset();
    _o.setSaveMountingReset(_oSaveMountingReset);
    boolean _oResetHmdPitch = resetHmdPitch();
    _o.setResetHmdPitch(_oResetHmdPitch);
  }
  public static int pack(FlatBufferBuilder builder, ResetsSettingsT _o) {
    if (_o == null) return 0;
    return createResetsSettings(
      builder,
      _o.getResetMountingFeet(),
      _o.getArmsMountingResetMode(),
      _o.getYawResetSmoothTime(),
      _o.getSaveMountingReset(),
      _o.getResetHmdPitch());
  }
}

