// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class ResetRequestT {
  private int resetType;
  private Integer bodyPose;
  private Integer referenceTracker;
  private int[] trackers;

  public int getResetType() { return resetType; }

  public void setResetType(int resetType) { this.resetType = resetType; }

  public Integer getBodyPose() { return bodyPose; }

  public void setBodyPose(Integer bodyPose) { this.bodyPose = bodyPose; }

  public Integer getReferenceTracker() { return referenceTracker; }

  public void setReferenceTracker(Integer referenceTracker) { this.referenceTracker = referenceTracker; }

  public int[] getTrackers() { return trackers; }

  public void setTrackers(int[] trackers) { this.trackers = trackers; }


  public ResetRequestT() {
    this.resetType = 0;
    this.bodyPose = null;
    this.referenceTracker = null;
    this.trackers = null;
  }
}

