// automatically generated by the FlatBuffers compiler, do not modify

/**
 * What kind of data the tracker supports.The received data gets computed into a Quaternion rotation in any case.
 */
export enum TrackerDataType {
  /**
   * Rotation (e.g: IMUs or computed rotations in firmware)
   */
  ROTATION = 0,

  /**
   * Flex resistance (e.g: raw data from flex sensors or unscaled angle on a single axis)
   */
  FLEX_RESISTANCE = 1,

  /**
   * Flex angle (e.g: computed angle from flex sensors or angle on a single axis)
   */
  FLEX_ANGLE = 2
}