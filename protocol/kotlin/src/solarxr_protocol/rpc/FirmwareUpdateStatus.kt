// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

@Suppress("unused")
class FirmwareUpdateStatus private constructor() {
    companion object {
        /**
         * The server is downloading the firmware
         */
        const val DOWNLOADING: UByte = 0u
        /**
         * The server is waiting for the tracker to be rebooted by the user
         * Note that is is not the same as REBOOTING
         */
        const val NEEDMANUALREBOOT: UByte = 1u
        /**
         * The server tries to authenticate with the MCU
         */
        const val AUTHENTICATING: UByte = 2u
        /**
         * The server is uploading the firmware to the Device
         */
        const val UPLOADING: UByte = 3u
        /**
         * The serial flasher tries to sync with the MCU
         * You can use this event to prompt the user to press the boot btn
         */
        const val SYNCINGWITHMCU: UByte = 4u
        /**
         * The MCU is rebooting
         */
        const val REBOOTING: UByte = 5u
        /**
         * The server is provisioning the tracker
         */
        const val PROVISIONING: UByte = 6u
        const val DONE: UByte = 7u
        /**
         * Could not find the device
         */
        const val ERRORDEVICENOTFOUND: UByte = 8u
        /**
         * The operation timed out, > 1min
         */
        const val ERRORTIMEOUT: UByte = 9u
        /**
         * The firmware download failed
         */
        const val ERRORDOWNLOADFAILED: UByte = 10u
        /**
         * The server could not authenticate with the MCU
         */
        const val ERRORAUTHENTICATIONFAILED: UByte = 11u
        /**
         * Could not upload the firmware to the MUC
         */
        const val ERRORUPLOADFAILED: UByte = 12u
        /**
         * The provision of the tracker failed, usually wifi credentials
         */
        const val ERRORPROVISIONINGFAILED: UByte = 13u
        /**
         * An unsupported Flashing method was used
         */
        const val ERRORUNSUPPORTEDMETHOD: UByte = 14u
        const val ERRORUNKNOWN: UByte = 15u
        val names : Array<String> = arrayOf("DOWNLOADING", "NEED_MANUAL_REBOOT", "AUTHENTICATING", "UPLOADING", "SYNCING_WITH_MCU", "REBOOTING", "PROVISIONING", "DONE", "ERROR_DEVICE_NOT_FOUND", "ERROR_TIMEOUT", "ERROR_DOWNLOAD_FAILED", "ERROR_AUTHENTICATION_FAILED", "ERROR_UPLOAD_FAILED", "ERROR_PROVISIONING_FAILED", "ERROR_UNSUPPORTED_METHOD", "ERROR_UNKNOWN")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}