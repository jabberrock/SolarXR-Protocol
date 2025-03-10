// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class StayAlignedSettings : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : StayAlignedSettings {
        __init(_i, _bb)
        return this
    }
    val enabled : Boolean
        get() {
            val o = __offset(4)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val extraYawCorrection : Boolean
        get() {
            val o = __offset(6)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val hideYawCorrection : Boolean
        get() {
            val o = __offset(8)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val standingUpperLegAngle : Float
        get() {
            val o = __offset(10)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val standingLowerLegAngle : Float
        get() {
            val o = __offset(12)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val standingFootAngle : Float
        get() {
            val o = __offset(14)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val sittingUpperLegAngle : Float
        get() {
            val o = __offset(16)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val sittingLowerLegAngle : Float
        get() {
            val o = __offset(18)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val sittingFootAngle : Float
        get() {
            val o = __offset(20)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val flatUpperLegAngle : Float
        get() {
            val o = __offset(22)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val flatLowerLegAngle : Float
        get() {
            val o = __offset(24)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val flatFootAngle : Float
        get() {
            val o = __offset(26)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsStayAlignedSettings(_bb: ByteBuffer): StayAlignedSettings = getRootAsStayAlignedSettings(_bb, StayAlignedSettings())
        @JvmStatic
        fun getRootAsStayAlignedSettings(_bb: ByteBuffer, obj: StayAlignedSettings): StayAlignedSettings {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createStayAlignedSettings(builder: FlatBufferBuilder, enabled: Boolean, extraYawCorrection: Boolean, hideYawCorrection: Boolean, standingUpperLegAngle: Float, standingLowerLegAngle: Float, standingFootAngle: Float, sittingUpperLegAngle: Float, sittingLowerLegAngle: Float, sittingFootAngle: Float, flatUpperLegAngle: Float, flatLowerLegAngle: Float, flatFootAngle: Float) : Int {
            builder.startTable(12)
            addFlatFootAngle(builder, flatFootAngle)
            addFlatLowerLegAngle(builder, flatLowerLegAngle)
            addFlatUpperLegAngle(builder, flatUpperLegAngle)
            addSittingFootAngle(builder, sittingFootAngle)
            addSittingLowerLegAngle(builder, sittingLowerLegAngle)
            addSittingUpperLegAngle(builder, sittingUpperLegAngle)
            addStandingFootAngle(builder, standingFootAngle)
            addStandingLowerLegAngle(builder, standingLowerLegAngle)
            addStandingUpperLegAngle(builder, standingUpperLegAngle)
            addHideYawCorrection(builder, hideYawCorrection)
            addExtraYawCorrection(builder, extraYawCorrection)
            addEnabled(builder, enabled)
            return endStayAlignedSettings(builder)
        }
        @JvmStatic
        fun startStayAlignedSettings(builder: FlatBufferBuilder) = builder.startTable(12)
        @JvmStatic
        fun addEnabled(builder: FlatBufferBuilder, enabled: Boolean) = builder.addBoolean(0, enabled, false)
        @JvmStatic
        fun addExtraYawCorrection(builder: FlatBufferBuilder, extraYawCorrection: Boolean) = builder.addBoolean(1, extraYawCorrection, false)
        @JvmStatic
        fun addHideYawCorrection(builder: FlatBufferBuilder, hideYawCorrection: Boolean) = builder.addBoolean(2, hideYawCorrection, false)
        @JvmStatic
        fun addStandingUpperLegAngle(builder: FlatBufferBuilder, standingUpperLegAngle: Float) = builder.addFloat(3, standingUpperLegAngle, 0.0)
        @JvmStatic
        fun addStandingLowerLegAngle(builder: FlatBufferBuilder, standingLowerLegAngle: Float) = builder.addFloat(4, standingLowerLegAngle, 0.0)
        @JvmStatic
        fun addStandingFootAngle(builder: FlatBufferBuilder, standingFootAngle: Float) = builder.addFloat(5, standingFootAngle, 0.0)
        @JvmStatic
        fun addSittingUpperLegAngle(builder: FlatBufferBuilder, sittingUpperLegAngle: Float) = builder.addFloat(6, sittingUpperLegAngle, 0.0)
        @JvmStatic
        fun addSittingLowerLegAngle(builder: FlatBufferBuilder, sittingLowerLegAngle: Float) = builder.addFloat(7, sittingLowerLegAngle, 0.0)
        @JvmStatic
        fun addSittingFootAngle(builder: FlatBufferBuilder, sittingFootAngle: Float) = builder.addFloat(8, sittingFootAngle, 0.0)
        @JvmStatic
        fun addFlatUpperLegAngle(builder: FlatBufferBuilder, flatUpperLegAngle: Float) = builder.addFloat(9, flatUpperLegAngle, 0.0)
        @JvmStatic
        fun addFlatLowerLegAngle(builder: FlatBufferBuilder, flatLowerLegAngle: Float) = builder.addFloat(10, flatLowerLegAngle, 0.0)
        @JvmStatic
        fun addFlatFootAngle(builder: FlatBufferBuilder, flatFootAngle: Float) = builder.addFloat(11, flatFootAngle, 0.0)
        @JvmStatic
        fun endStayAlignedSettings(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
