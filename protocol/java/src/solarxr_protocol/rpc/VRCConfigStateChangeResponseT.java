// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class VRCConfigStateChangeResponseT {
  private boolean isSupported;
  private solarxr_protocol.rpc.VRCConfigValidityT validity;
  private solarxr_protocol.rpc.VRCConfigValuesT state;
  private solarxr_protocol.rpc.VRCConfigRecommendedValuesT recommended;

  public boolean getIsSupported() { return isSupported; }

  public void setIsSupported(boolean isSupported) { this.isSupported = isSupported; }

  public solarxr_protocol.rpc.VRCConfigValidityT getValidity() { return validity; }

  public void setValidity(solarxr_protocol.rpc.VRCConfigValidityT validity) { this.validity = validity; }

  public solarxr_protocol.rpc.VRCConfigValuesT getState() { return state; }

  public void setState(solarxr_protocol.rpc.VRCConfigValuesT state) { this.state = state; }

  public solarxr_protocol.rpc.VRCConfigRecommendedValuesT getRecommended() { return recommended; }

  public void setRecommended(solarxr_protocol.rpc.VRCConfigRecommendedValuesT recommended) { this.recommended = recommended; }


  public VRCConfigStateChangeResponseT() {
    this.isSupported = false;
    this.validity = null;
    this.state = null;
    this.recommended = null;
  }
}

