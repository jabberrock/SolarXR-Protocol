// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes.hardware_info

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Mostly static info about the device's hardware/firmware.
 */
@Suppress("unused")
class HardwareInfo : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : HardwareInfo {
        __init(_i, _bb)
        return this
    }
    val mcuId : UShort
        get() {
            val o = __offset(4)
            return if(o != 0) bb.getShort(o + bb_pos).toUShort() else 0u
        }
    /**
     * A human-friendly name to display as the name of the device.
     */
    val displayName : String?
        get() {
            val o = __offset(6)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val displayNameAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(6, 1)
    fun displayNameInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 6, 1)
    /**
     * A human-friendly string for the device model.
     */
    val model : String?
        get() {
            val o = __offset(8)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val modelAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(8, 1)
    fun modelInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 8, 1)
    /**
     * A human-friendly string for the manufacturer of the device.
     */
    val manufacturer : String?
        get() {
            val o = __offset(10)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val manufacturerAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(10, 1)
    fun manufacturerInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 10, 1)
    /**
     * The hardware version of the device. For example, pcb version.
     */
    val hardwareRevision : String?
        get() {
            val o = __offset(12)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val hardwareRevisionAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(12, 1)
    fun hardwareRevisionInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 12, 1)
    /**
     * The version of the slimevr firmware that the device is running.
     */
    val firmwareVersion : String?
        get() {
            val o = __offset(14)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val firmwareVersionAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(14, 1)
    fun firmwareVersionInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 14, 1)
    val hardwareAddress : solarxr_protocol.datatypes.hardware_info.HardwareAddress? get() = hardwareAddress(solarxr_protocol.datatypes.hardware_info.HardwareAddress())
    fun hardwareAddress(obj: solarxr_protocol.datatypes.hardware_info.HardwareAddress) : solarxr_protocol.datatypes.hardware_info.HardwareAddress? {
        val o = __offset(16)
        return if (o != 0) {
            obj.__assign(o + bb_pos, bb)
        } else {
            null
        }
    }
    val ipAddress : solarxr_protocol.datatypes.Ipv4Address? get() = ipAddress(solarxr_protocol.datatypes.Ipv4Address())
    fun ipAddress(obj: solarxr_protocol.datatypes.Ipv4Address) : solarxr_protocol.datatypes.Ipv4Address? {
        val o = __offset(18)
        return if (o != 0) {
            obj.__assign(o + bb_pos, bb)
        } else {
            null
        }
    }
    /**
     * A board type string that can be used to name a board. if possible you should use official board type
     */
    val boardType : String?
        get() {
            val o = __offset(20)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val boardTypeAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(20, 1)
    fun boardTypeInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 20, 1)
    /**
     * An enum listing all the board types supported by the firmware
     */
    val officialBoardType : UShort
        get() {
            val o = __offset(22)
            return if(o != 0) bb.getShort(o + bb_pos).toUShort() else 0u
        }
    /**
     * A unique identifier for the device. Depending on the type of device it can be the MAC address,
     * the IP address, or some other unique identifier like what USB device it is.
     */
    val hardwareIdentifier : String?
        get() {
            val o = __offset(24)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val hardwareIdentifierAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(24, 1)
    fun hardwareIdentifierInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 24, 1)
    /**
     * The version of the protocol it's using to communicate with server
     */
    val networkProtocolVersion : UShort?
        get() {
            val o = __offset(26)
            return if(o != 0) bb.getShort(o + bb_pos).toUShort() else null
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsHardwareInfo(_bb: ByteBuffer): HardwareInfo = getRootAsHardwareInfo(_bb, HardwareInfo())
        @JvmStatic
        fun getRootAsHardwareInfo(_bb: ByteBuffer, obj: HardwareInfo): HardwareInfo {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun startHardwareInfo(builder: FlatBufferBuilder) = builder.startTable(12)
        @JvmStatic
        fun addMcuId(builder: FlatBufferBuilder, mcuId: UShort) = builder.addShort(0, mcuId.toShort(), 0)
        @JvmStatic
        fun addDisplayName(builder: FlatBufferBuilder, displayName: Int) = builder.addOffset(1, displayName, 0)
        @JvmStatic
        fun addModel(builder: FlatBufferBuilder, model: Int) = builder.addOffset(2, model, 0)
        @JvmStatic
        fun addManufacturer(builder: FlatBufferBuilder, manufacturer: Int) = builder.addOffset(3, manufacturer, 0)
        @JvmStatic
        fun addHardwareRevision(builder: FlatBufferBuilder, hardwareRevision: Int) = builder.addOffset(4, hardwareRevision, 0)
        @JvmStatic
        fun addFirmwareVersion(builder: FlatBufferBuilder, firmwareVersion: Int) = builder.addOffset(5, firmwareVersion, 0)
        @JvmStatic
        fun addHardwareAddress(builder: FlatBufferBuilder, hardwareAddress: Int) = builder.addStruct(6, hardwareAddress, 0)
        @JvmStatic
        fun addIpAddress(builder: FlatBufferBuilder, ipAddress: Int) = builder.addStruct(7, ipAddress, 0)
        @JvmStatic
        fun addBoardType(builder: FlatBufferBuilder, boardType: Int) = builder.addOffset(8, boardType, 0)
        @JvmStatic
        fun addOfficialBoardType(builder: FlatBufferBuilder, officialBoardType: UShort) = builder.addShort(9, officialBoardType.toShort(), 0)
        @JvmStatic
        fun addHardwareIdentifier(builder: FlatBufferBuilder, hardwareIdentifier: Int) = builder.addOffset(10, hardwareIdentifier, 0)
        @JvmStatic
        fun addNetworkProtocolVersion(builder: FlatBufferBuilder, networkProtocolVersion: UShort) = builder.addShort(11, networkProtocolVersion.toShort(), 0)
        @JvmStatic
        fun endHardwareInfo(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
