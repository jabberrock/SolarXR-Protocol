// automatically generated by the FlatBuffers compiler, do not modify

export { MessageBundle, MessageBundleT } from './solarxr-protocol/message-bundle.js';
export { Bone, BoneT } from './solarxr-protocol/data-feed/bone.js';
export { DataFeedConfig, DataFeedConfigT } from './solarxr-protocol/data-feed/data-feed-config.js';
export { DataFeedMessage, unionToDataFeedMessage, unionListToDataFeedMessage } from './solarxr-protocol/data-feed/data-feed-message.js';
export { DataFeedMessageHeader, DataFeedMessageHeaderT } from './solarxr-protocol/data-feed/data-feed-message-header.js';
export { DataFeedUpdate, DataFeedUpdateT } from './solarxr-protocol/data-feed/data-feed-update.js';
export { PollDataFeed, PollDataFeedT } from './solarxr-protocol/data-feed/poll-data-feed.js';
export { StartDataFeed, StartDataFeedT } from './solarxr-protocol/data-feed/start-data-feed.js';
export { DeviceData, DeviceDataT } from './solarxr-protocol/data-feed/device-data/device-data.js';
export { DeviceDataMask, DeviceDataMaskT } from './solarxr-protocol/data-feed/device-data/device-data-mask.js';
export { StayAlignedData, StayAlignedDataT } from './solarxr-protocol/data-feed/stay-aligned/stay-aligned-data.js';
export { StayAlignedTracker, StayAlignedTrackerT } from './solarxr-protocol/data-feed/stay-aligned/stay-aligned-tracker.js';
export { TrackerData, TrackerDataT } from './solarxr-protocol/data-feed/tracker/tracker-data.js';
export { TrackerDataMask, TrackerDataMaskT } from './solarxr-protocol/data-feed/tracker/tracker-data-mask.js';
export { TrackerInfo, TrackerInfoT } from './solarxr-protocol/data-feed/tracker/tracker-info.js';
export { BodyPart } from './solarxr-protocol/datatypes/body-part.js';
export { Bytes, BytesT } from './solarxr-protocol/datatypes/bytes.js';
export { DeviceId, DeviceIdT } from './solarxr-protocol/datatypes/device-id.js';
export { DeviceIdTable, DeviceIdTableT } from './solarxr-protocol/datatypes/device-id-table.js';
export { FilteringType } from './solarxr-protocol/datatypes/filtering-type.js';
export { FirmwareErrorCode } from './solarxr-protocol/datatypes/firmware-error-code.js';
export { HzF32, HzF32T } from './solarxr-protocol/datatypes/hz-f32.js';
export { Ipv4Address, Ipv4AddressT } from './solarxr-protocol/datatypes/ipv4-address.js';
export { LogData, LogDataT } from './solarxr-protocol/datatypes/log-data.js';
export { MagnetometerStatus } from './solarxr-protocol/datatypes/magnetometer-status.js';
export { StringTable, StringTableT } from './solarxr-protocol/datatypes/string-table.js';
export { Temperature, TemperatureT } from './solarxr-protocol/datatypes/temperature.js';
export { TrackerId, TrackerIdT } from './solarxr-protocol/datatypes/tracker-id.js';
export { TrackerStatus } from './solarxr-protocol/datatypes/tracker-status.js';
export { TransactionId, TransactionIdT } from './solarxr-protocol/datatypes/transaction-id.js';
export { BoardType } from './solarxr-protocol/datatypes/hardware-info/board-type.js';
export { FirmwareStatusMask, FirmwareStatusMaskT } from './solarxr-protocol/datatypes/hardware-info/firmware-status-mask.js';
export { HardwareAddress, HardwareAddressT } from './solarxr-protocol/datatypes/hardware-info/hardware-address.js';
export { HardwareInfo, HardwareInfoT } from './solarxr-protocol/datatypes/hardware-info/hardware-info.js';
export { HardwareStatus, HardwareStatusT } from './solarxr-protocol/datatypes/hardware-info/hardware-status.js';
export { ImuType } from './solarxr-protocol/datatypes/hardware-info/imu-type.js';
export { McuType } from './solarxr-protocol/datatypes/hardware-info/mcu-type.js';
export { TrackerDataType } from './solarxr-protocol/datatypes/hardware-info/tracker-data-type.js';
export { Quat, QuatT } from './solarxr-protocol/datatypes/math/quat.js';
export { Vec3f, Vec3fT } from './solarxr-protocol/datatypes/math/vec3f.js';
export { KeyValues, KeyValuesT } from './solarxr-protocol/pub-sub/key-values.js';
export { Message, MessageT } from './solarxr-protocol/pub-sub/message.js';
export { Payload, unionToPayload, unionListToPayload } from './solarxr-protocol/pub-sub/payload.js';
export { PubSubHeader, PubSubHeaderT } from './solarxr-protocol/pub-sub/pub-sub-header.js';
export { PubSubUnion, unionToPubSubUnion, unionListToPubSubUnion } from './solarxr-protocol/pub-sub/pub-sub-union.js';
export { SubscriptionRequest, SubscriptionRequestT } from './solarxr-protocol/pub-sub/subscription-request.js';
export { Topic, unionToTopic, unionListToTopic } from './solarxr-protocol/pub-sub/topic.js';
export { TopicHandle, TopicHandleT } from './solarxr-protocol/pub-sub/topic-handle.js';
export { TopicHandleRequest, TopicHandleRequestT } from './solarxr-protocol/pub-sub/topic-handle-request.js';
export { TopicId, TopicIdT } from './solarxr-protocol/pub-sub/topic-id.js';
export { TopicMapping, TopicMappingT } from './solarxr-protocol/pub-sub/topic-mapping.js';
export { AddUnknownDeviceRequest, AddUnknownDeviceRequestT } from './solarxr-protocol/rpc/add-unknown-device-request.js';
export { ArmsMountingResetMode } from './solarxr-protocol/rpc/arms-mounting-reset-mode.js';
export { AssignTrackerRequest, AssignTrackerRequestT } from './solarxr-protocol/rpc/assign-tracker-request.js';
export { AutoBoneApplyRequest, AutoBoneApplyRequestT } from './solarxr-protocol/rpc/auto-bone-apply-request.js';
export { AutoBoneCancelRecordingRequest, AutoBoneCancelRecordingRequestT } from './solarxr-protocol/rpc/auto-bone-cancel-recording-request.js';
export { AutoBoneEpochResponse, AutoBoneEpochResponseT } from './solarxr-protocol/rpc/auto-bone-epoch-response.js';
export { AutoBoneProcessRequest, AutoBoneProcessRequestT } from './solarxr-protocol/rpc/auto-bone-process-request.js';
export { AutoBoneProcessStatusResponse, AutoBoneProcessStatusResponseT } from './solarxr-protocol/rpc/auto-bone-process-status-response.js';
export { AutoBoneProcessType } from './solarxr-protocol/rpc/auto-bone-process-type.js';
export { AutoBoneSettings, AutoBoneSettingsT } from './solarxr-protocol/rpc/auto-bone-settings.js';
export { AutoBoneStopRecordingRequest, AutoBoneStopRecordingRequestT } from './solarxr-protocol/rpc/auto-bone-stop-recording-request.js';
export { ChangeMagToggleRequest, ChangeMagToggleRequestT } from './solarxr-protocol/rpc/change-mag-toggle-request.js';
export { ChangeSettingsRequest, ChangeSettingsRequestT } from './solarxr-protocol/rpc/change-settings-request.js';
export { ChangeSkeletonConfigRequest, ChangeSkeletonConfigRequestT } from './solarxr-protocol/rpc/change-skeleton-config-request.js';
export { ClearDriftCompensationRequest, ClearDriftCompensationRequestT } from './solarxr-protocol/rpc/clear-drift-compensation-request.js';
export { ClearMountingResetRequest, ClearMountingResetRequestT } from './solarxr-protocol/rpc/clear-mounting-reset-request.js';
export { CloseSerialRequest, CloseSerialRequestT } from './solarxr-protocol/rpc/close-serial-request.js';
export { ComputerDirectory } from './solarxr-protocol/rpc/computer-directory.js';
export { DetectStayAlignedRelaxedPoseRequest, DetectStayAlignedRelaxedPoseRequestT } from './solarxr-protocol/rpc/detect-stay-aligned-relaxed-pose-request.js';
export { DriftCompensationSettings, DriftCompensationSettingsT } from './solarxr-protocol/rpc/drift-compensation-settings.js';
export { EnableStayAlignedRequest, EnableStayAlignedRequestT } from './solarxr-protocol/rpc/enable-stay-aligned-request.js';
export { FilteringSettings, FilteringSettingsT } from './solarxr-protocol/rpc/filtering-settings.js';
export { FirmwarePart, FirmwarePartT } from './solarxr-protocol/rpc/firmware-part.js';
export { FirmwareUpdateDeviceId, unionToFirmwareUpdateDeviceId, unionListToFirmwareUpdateDeviceId } from './solarxr-protocol/rpc/firmware-update-device-id.js';
export { FirmwareUpdateMethod, unionToFirmwareUpdateMethod, unionListToFirmwareUpdateMethod } from './solarxr-protocol/rpc/firmware-update-method.js';
export { FirmwareUpdateRequest, FirmwareUpdateRequestT } from './solarxr-protocol/rpc/firmware-update-request.js';
export { FirmwareUpdateStatus } from './solarxr-protocol/rpc/firmware-update-status.js';
export { FirmwareUpdateStatusResponse, FirmwareUpdateStatusResponseT } from './solarxr-protocol/rpc/firmware-update-status-response.js';
export { FirmwareUpdateStopQueuesRequest, FirmwareUpdateStopQueuesRequestT } from './solarxr-protocol/rpc/firmware-update-stop-queues-request.js';
export { ForgetDeviceRequest, ForgetDeviceRequestT } from './solarxr-protocol/rpc/forget-device-request.js';
export { HeartbeatRequest, HeartbeatRequestT } from './solarxr-protocol/rpc/heartbeat-request.js';
export { HeartbeatResponse, HeartbeatResponseT } from './solarxr-protocol/rpc/heartbeat-response.js';
export { HeightRequest, HeightRequestT } from './solarxr-protocol/rpc/height-request.js';
export { HeightResponse, HeightResponseT } from './solarxr-protocol/rpc/height-response.js';
export { LegTweaksTmpChange, LegTweaksTmpChangeT } from './solarxr-protocol/rpc/leg-tweaks-tmp-change.js';
export { LegTweaksTmpClear, LegTweaksTmpClearT } from './solarxr-protocol/rpc/leg-tweaks-tmp-clear.js';
export { MagToggleRequest, MagToggleRequestT } from './solarxr-protocol/rpc/mag-toggle-request.js';
export { MagToggleResponse, MagToggleResponseT } from './solarxr-protocol/rpc/mag-toggle-response.js';
export { NewSerialDeviceResponse, NewSerialDeviceResponseT } from './solarxr-protocol/rpc/new-serial-device-response.js';
export { OSCRouterSettings, OSCRouterSettingsT } from './solarxr-protocol/rpc/oscrouter-settings.js';
export { OSCSettings, OSCSettingsT } from './solarxr-protocol/rpc/oscsettings.js';
export { OSCTrackersSetting, OSCTrackersSettingT } from './solarxr-protocol/rpc/osctrackers-setting.js';
export { OTAFirmwareUpdate, OTAFirmwareUpdateT } from './solarxr-protocol/rpc/otafirmware-update.js';
export { OpenSerialRequest, OpenSerialRequestT } from './solarxr-protocol/rpc/open-serial-request.js';
export { OverlayDisplayModeChangeRequest, OverlayDisplayModeChangeRequestT } from './solarxr-protocol/rpc/overlay-display-mode-change-request.js';
export { OverlayDisplayModeRequest, OverlayDisplayModeRequestT } from './solarxr-protocol/rpc/overlay-display-mode-request.js';
export { OverlayDisplayModeResponse, OverlayDisplayModeResponseT } from './solarxr-protocol/rpc/overlay-display-mode-response.js';
export { RecordBVHRequest, RecordBVHRequestT } from './solarxr-protocol/rpc/record-bvhrequest.js';
export { RecordBVHStatus, RecordBVHStatusT } from './solarxr-protocol/rpc/record-bvhstatus.js';
export { RecordBVHStatusRequest, RecordBVHStatusRequestT } from './solarxr-protocol/rpc/record-bvhstatus-request.js';
export { ResetBodyPose } from './solarxr-protocol/rpc/reset-body-pose.js';
export { ResetRequest, ResetRequestT } from './solarxr-protocol/rpc/reset-request.js';
export { ResetResponse, ResetResponseT } from './solarxr-protocol/rpc/reset-response.js';
export { ResetStatus } from './solarxr-protocol/rpc/reset-status.js';
export { ResetStayAlignedRelaxedPoseRequest, ResetStayAlignedRelaxedPoseRequestT } from './solarxr-protocol/rpc/reset-stay-aligned-relaxed-pose-request.js';
export { ResetType } from './solarxr-protocol/rpc/reset-type.js';
export { ResetsSettings, ResetsSettingsT } from './solarxr-protocol/rpc/resets-settings.js';
export { RpcMessage, unionToRpcMessage, unionListToRpcMessage } from './solarxr-protocol/rpc/rpc-message.js';
export { RpcMessageHeader, RpcMessageHeaderT } from './solarxr-protocol/rpc/rpc-message-header.js';
export { SaveFileNotification, SaveFileNotificationT } from './solarxr-protocol/rpc/save-file-notification.js';
export { SerialDevice, SerialDeviceT } from './solarxr-protocol/rpc/serial-device.js';
export { SerialDevicePort, SerialDevicePortT } from './solarxr-protocol/rpc/serial-device-port.js';
export { SerialDevicesRequest, SerialDevicesRequestT } from './solarxr-protocol/rpc/serial-devices-request.js';
export { SerialDevicesResponse, SerialDevicesResponseT } from './solarxr-protocol/rpc/serial-devices-response.js';
export { SerialFirmwareUpdate, SerialFirmwareUpdateT } from './solarxr-protocol/rpc/serial-firmware-update.js';
export { SerialTrackerFactoryResetRequest, SerialTrackerFactoryResetRequestT } from './solarxr-protocol/rpc/serial-tracker-factory-reset-request.js';
export { SerialTrackerGetInfoRequest, SerialTrackerGetInfoRequestT } from './solarxr-protocol/rpc/serial-tracker-get-info-request.js';
export { SerialTrackerGetWifiScanRequest, SerialTrackerGetWifiScanRequestT } from './solarxr-protocol/rpc/serial-tracker-get-wifi-scan-request.js';
export { SerialTrackerRebootRequest, SerialTrackerRebootRequestT } from './solarxr-protocol/rpc/serial-tracker-reboot-request.js';
export { SerialUpdateResponse, SerialUpdateResponseT } from './solarxr-protocol/rpc/serial-update-response.js';
export { ServerInfosRequest, ServerInfosRequestT } from './solarxr-protocol/rpc/server-infos-request.js';
export { ServerInfosResponse, ServerInfosResponseT } from './solarxr-protocol/rpc/server-infos-response.js';
export { SetPauseTrackingRequest, SetPauseTrackingRequestT } from './solarxr-protocol/rpc/set-pause-tracking-request.js';
export { SetWifiRequest, SetWifiRequestT } from './solarxr-protocol/rpc/set-wifi-request.js';
export { SettingsRequest, SettingsRequestT } from './solarxr-protocol/rpc/settings-request.js';
export { SettingsResetRequest, SettingsResetRequestT } from './solarxr-protocol/rpc/settings-reset-request.js';
export { SettingsResponse, SettingsResponseT } from './solarxr-protocol/rpc/settings-response.js';
export { SkeletonBone } from './solarxr-protocol/rpc/skeleton-bone.js';
export { SkeletonConfigRequest, SkeletonConfigRequestT } from './solarxr-protocol/rpc/skeleton-config-request.js';
export { SkeletonConfigResponse, SkeletonConfigResponseT } from './solarxr-protocol/rpc/skeleton-config-response.js';
export { SkeletonPart, SkeletonPartT } from './solarxr-protocol/rpc/skeleton-part.js';
export { SkeletonResetAllRequest, SkeletonResetAllRequestT } from './solarxr-protocol/rpc/skeleton-reset-all-request.js';
export { StartWifiProvisioningRequest, StartWifiProvisioningRequestT } from './solarxr-protocol/rpc/start-wifi-provisioning-request.js';
export { StatusData, unionToStatusData, unionListToStatusData } from './solarxr-protocol/rpc/status-data.js';
export { StatusMessage, StatusMessageT } from './solarxr-protocol/rpc/status-message.js';
export { StatusSteamVRDisconnected, StatusSteamVRDisconnectedT } from './solarxr-protocol/rpc/status-steam-vrdisconnected.js';
export { StatusSystemFixed, StatusSystemFixedT } from './solarxr-protocol/rpc/status-system-fixed.js';
export { StatusSystemRequest, StatusSystemRequestT } from './solarxr-protocol/rpc/status-system-request.js';
export { StatusSystemResponse, StatusSystemResponseT } from './solarxr-protocol/rpc/status-system-response.js';
export { StatusSystemUpdate, StatusSystemUpdateT } from './solarxr-protocol/rpc/status-system-update.js';
export { StatusTrackerError, StatusTrackerErrorT } from './solarxr-protocol/rpc/status-tracker-error.js';
export { StatusTrackerReset, StatusTrackerResetT } from './solarxr-protocol/rpc/status-tracker-reset.js';
export { StatusUnassignedHMD, StatusUnassignedHMDT } from './solarxr-protocol/rpc/status-unassigned-hmd.js';
export { StayAlignedRelaxedPose } from './solarxr-protocol/rpc/stay-aligned-relaxed-pose.js';
export { StayAlignedSettings, StayAlignedSettingsT } from './solarxr-protocol/rpc/stay-aligned-settings.js';
export { SteamVRTrackersSetting, SteamVRTrackersSettingT } from './solarxr-protocol/rpc/steam-vrtrackers-setting.js';
export { StopWifiProvisioningRequest, StopWifiProvisioningRequestT } from './solarxr-protocol/rpc/stop-wifi-provisioning-request.js';
export { TapDetectionSettings, TapDetectionSettingsT } from './solarxr-protocol/rpc/tap-detection-settings.js';
export { TapDetectionSetupNotification, TapDetectionSetupNotificationT } from './solarxr-protocol/rpc/tap-detection-setup-notification.js';
export { TrackingPauseStateRequest, TrackingPauseStateRequestT } from './solarxr-protocol/rpc/tracking-pause-state-request.js';
export { TrackingPauseStateResponse, TrackingPauseStateResponseT } from './solarxr-protocol/rpc/tracking-pause-state-response.js';
export { UnknownDeviceHandshakeNotification, UnknownDeviceHandshakeNotificationT } from './solarxr-protocol/rpc/unknown-device-handshake-notification.js';
export { VMCOSCSettings, VMCOSCSettingsT } from './solarxr-protocol/rpc/vmcoscsettings.js';
export { VRCOSCSettings, VRCOSCSettingsT } from './solarxr-protocol/rpc/vrcoscsettings.js';
export { WifiProvisioningStatus } from './solarxr-protocol/rpc/wifi-provisioning-status.js';
export { WifiProvisioningStatusResponse, WifiProvisioningStatusResponseT } from './solarxr-protocol/rpc/wifi-provisioning-status-response.js';
export { LegTweaksSettings, LegTweaksSettingsT } from './solarxr-protocol/rpc/settings/leg-tweaks-settings.js';
export { ModelRatios, ModelRatiosT } from './solarxr-protocol/rpc/settings/model-ratios.js';
export { ModelSettings, ModelSettingsT } from './solarxr-protocol/rpc/settings/model-settings.js';
export { ModelToggles, ModelTogglesT } from './solarxr-protocol/rpc/settings/model-toggles.js';
export { SkeletonHeight, SkeletonHeightT } from './solarxr-protocol/rpc/settings/skeleton-height.js';
