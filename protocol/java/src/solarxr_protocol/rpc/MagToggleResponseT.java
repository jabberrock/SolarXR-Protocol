// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class MagToggleResponseT {
  private solarxr_protocol.datatypes.TrackerIdT trackerId;
  private boolean enable;

  public solarxr_protocol.datatypes.TrackerIdT getTrackerId() { return trackerId; }

  public void setTrackerId(solarxr_protocol.datatypes.TrackerIdT trackerId) { this.trackerId = trackerId; }

  public boolean getEnable() { return enable; }

  public void setEnable(boolean enable) { this.enable = enable; }


  public MagToggleResponseT() {
    this.trackerId = null;
    this.enable = false;
  }
}

