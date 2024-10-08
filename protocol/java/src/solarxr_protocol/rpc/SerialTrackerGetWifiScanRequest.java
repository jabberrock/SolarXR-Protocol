// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Sends the GET WIFISCAN cmd to the current tracker on the serial monitor
 */
@SuppressWarnings("unused")
public final class SerialTrackerGetWifiScanRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static SerialTrackerGetWifiScanRequest getRootAsSerialTrackerGetWifiScanRequest(ByteBuffer _bb) { return getRootAsSerialTrackerGetWifiScanRequest(_bb, new SerialTrackerGetWifiScanRequest()); }
  public static SerialTrackerGetWifiScanRequest getRootAsSerialTrackerGetWifiScanRequest(ByteBuffer _bb, SerialTrackerGetWifiScanRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SerialTrackerGetWifiScanRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startSerialTrackerGetWifiScanRequest(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endSerialTrackerGetWifiScanRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SerialTrackerGetWifiScanRequest get(int j) { return get(new SerialTrackerGetWifiScanRequest(), j); }
    public SerialTrackerGetWifiScanRequest get(SerialTrackerGetWifiScanRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SerialTrackerGetWifiScanRequestT unpack() {
    SerialTrackerGetWifiScanRequestT _o = new SerialTrackerGetWifiScanRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SerialTrackerGetWifiScanRequestT _o) {
  }
  public static int pack(FlatBufferBuilder builder, SerialTrackerGetWifiScanRequestT _o) {
    if (_o == null) return 0;
    startSerialTrackerGetWifiScanRequest(builder);
    return endSerialTrackerGetWifiScanRequest(builder);
  }
}

