// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.data_feed;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class DataFeedConfigT {
  private int minimumTimeSinceLast;
  private solarxr_protocol.data_feed.device_data.DeviceDataMaskT dataMask;
  private solarxr_protocol.data_feed.tracker.TrackerDataMaskT syntheticTrackersMask;
  private boolean boneMask;
  private boolean stayAlignedPoseMask;

  public int getMinimumTimeSinceLast() { return minimumTimeSinceLast; }

  public void setMinimumTimeSinceLast(int minimumTimeSinceLast) { this.minimumTimeSinceLast = minimumTimeSinceLast; }

  public solarxr_protocol.data_feed.device_data.DeviceDataMaskT getDataMask() { return dataMask; }

  public void setDataMask(solarxr_protocol.data_feed.device_data.DeviceDataMaskT dataMask) { this.dataMask = dataMask; }

  public solarxr_protocol.data_feed.tracker.TrackerDataMaskT getSyntheticTrackersMask() { return syntheticTrackersMask; }

  public void setSyntheticTrackersMask(solarxr_protocol.data_feed.tracker.TrackerDataMaskT syntheticTrackersMask) { this.syntheticTrackersMask = syntheticTrackersMask; }

  public boolean getBoneMask() { return boneMask; }

  public void setBoneMask(boolean boneMask) { this.boneMask = boneMask; }

  public boolean getStayAlignedPoseMask() { return stayAlignedPoseMask; }

  public void setStayAlignedPoseMask(boolean stayAlignedPoseMask) { this.stayAlignedPoseMask = stayAlignedPoseMask; }


  public DataFeedConfigT() {
    this.minimumTimeSinceLast = 0;
    this.dataMask = null;
    this.syntheticTrackersMask = null;
    this.boneMask = false;
    this.stayAlignedPoseMask = false;
  }
}

