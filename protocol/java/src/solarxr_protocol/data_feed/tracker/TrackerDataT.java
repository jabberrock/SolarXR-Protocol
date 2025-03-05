// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.data_feed.tracker;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class TrackerDataT {
  private solarxr_protocol.datatypes.TrackerIdT trackerId;
  private solarxr_protocol.data_feed.tracker.TrackerInfoT info;
  private int status;
  private solarxr_protocol.datatypes.math.QuatT rotation;
  private solarxr_protocol.datatypes.math.Vec3fT position;
  private solarxr_protocol.datatypes.math.Vec3fT rawAngularVelocity;
  private solarxr_protocol.datatypes.math.Vec3fT rawAcceleration;
  private solarxr_protocol.datatypes.TemperatureT temp;
  private solarxr_protocol.datatypes.math.Vec3fT linearAcceleration;
  private solarxr_protocol.datatypes.math.QuatT rotationReferenceAdjusted;
  private solarxr_protocol.datatypes.math.QuatT rotationIdentityAdjusted;
  private Integer tps;
  private solarxr_protocol.data_feed.StayAlignedStateT stayAligned;

  public solarxr_protocol.datatypes.TrackerIdT getTrackerId() { return trackerId; }

  public void setTrackerId(solarxr_protocol.datatypes.TrackerIdT trackerId) { this.trackerId = trackerId; }

  public solarxr_protocol.data_feed.tracker.TrackerInfoT getInfo() { return info; }

  public void setInfo(solarxr_protocol.data_feed.tracker.TrackerInfoT info) { this.info = info; }

  public int getStatus() { return status; }

  public void setStatus(int status) { this.status = status; }

  public solarxr_protocol.datatypes.math.QuatT getRotation() { return rotation; }

  public void setRotation(solarxr_protocol.datatypes.math.QuatT rotation) { this.rotation = rotation; }

  public solarxr_protocol.datatypes.math.Vec3fT getPosition() { return position; }

  public void setPosition(solarxr_protocol.datatypes.math.Vec3fT position) { this.position = position; }

  public solarxr_protocol.datatypes.math.Vec3fT getRawAngularVelocity() { return rawAngularVelocity; }

  public void setRawAngularVelocity(solarxr_protocol.datatypes.math.Vec3fT rawAngularVelocity) { this.rawAngularVelocity = rawAngularVelocity; }

  public solarxr_protocol.datatypes.math.Vec3fT getRawAcceleration() { return rawAcceleration; }

  public void setRawAcceleration(solarxr_protocol.datatypes.math.Vec3fT rawAcceleration) { this.rawAcceleration = rawAcceleration; }

  public solarxr_protocol.datatypes.TemperatureT getTemp() { return temp; }

  public void setTemp(solarxr_protocol.datatypes.TemperatureT temp) { this.temp = temp; }

  public solarxr_protocol.datatypes.math.Vec3fT getLinearAcceleration() { return linearAcceleration; }

  public void setLinearAcceleration(solarxr_protocol.datatypes.math.Vec3fT linearAcceleration) { this.linearAcceleration = linearAcceleration; }

  public solarxr_protocol.datatypes.math.QuatT getRotationReferenceAdjusted() { return rotationReferenceAdjusted; }

  public void setRotationReferenceAdjusted(solarxr_protocol.datatypes.math.QuatT rotationReferenceAdjusted) { this.rotationReferenceAdjusted = rotationReferenceAdjusted; }

  public solarxr_protocol.datatypes.math.QuatT getRotationIdentityAdjusted() { return rotationIdentityAdjusted; }

  public void setRotationIdentityAdjusted(solarxr_protocol.datatypes.math.QuatT rotationIdentityAdjusted) { this.rotationIdentityAdjusted = rotationIdentityAdjusted; }

  public Integer getTps() { return tps; }

  public void setTps(Integer tps) { this.tps = tps; }

  public solarxr_protocol.data_feed.StayAlignedStateT getStayAligned() { return stayAligned; }

  public void setStayAligned(solarxr_protocol.data_feed.StayAlignedStateT stayAligned) { this.stayAligned = stayAligned; }


  public TrackerDataT() {
    this.trackerId = null;
    this.info = null;
    this.status = 0;
    this.rotation = new solarxr_protocol.datatypes.math.QuatT();
    this.position = new solarxr_protocol.datatypes.math.Vec3fT();
    this.rawAngularVelocity = new solarxr_protocol.datatypes.math.Vec3fT();
    this.rawAcceleration = new solarxr_protocol.datatypes.math.Vec3fT();
    this.temp = new solarxr_protocol.datatypes.TemperatureT();
    this.linearAcceleration = new solarxr_protocol.datatypes.math.Vec3fT();
    this.rotationReferenceAdjusted = new solarxr_protocol.datatypes.math.QuatT();
    this.rotationIdentityAdjusted = new solarxr_protocol.datatypes.math.QuatT();
    this.tps = null;
    this.stayAligned = null;
  }
}

