// automatically generated by the FlatBuffers compiler, do not modify

import { OTAFirmwareUpdate, OTAFirmwareUpdateT } from '../../solarxr-protocol/rpc/otafirmware-update.js';
import { SerialFirmwareUpdate, SerialFirmwareUpdateT } from '../../solarxr-protocol/rpc/serial-firmware-update.js';


export enum FirmwareUpdateMethod {
  NONE = 0,
  OTAFirmwareUpdate = 1,
  SerialFirmwareUpdate = 2
}

export function unionToFirmwareUpdateMethod(
  type: FirmwareUpdateMethod,
  accessor: (obj:OTAFirmwareUpdate|SerialFirmwareUpdate) => OTAFirmwareUpdate|SerialFirmwareUpdate|null
): OTAFirmwareUpdate|SerialFirmwareUpdate|null {
  switch(FirmwareUpdateMethod[type]) {
    case 'NONE': return null; 
    case 'OTAFirmwareUpdate': return accessor(new OTAFirmwareUpdate())! as OTAFirmwareUpdate;
    case 'SerialFirmwareUpdate': return accessor(new SerialFirmwareUpdate())! as SerialFirmwareUpdate;
    default: return null;
  }
}

export function unionListToFirmwareUpdateMethod(
  type: FirmwareUpdateMethod, 
  accessor: (index: number, obj:OTAFirmwareUpdate|SerialFirmwareUpdate) => OTAFirmwareUpdate|SerialFirmwareUpdate|null, 
  index: number
): OTAFirmwareUpdate|SerialFirmwareUpdate|null {
  switch(FirmwareUpdateMethod[type]) {
    case 'NONE': return null; 
    case 'OTAFirmwareUpdate': return accessor(index, new OTAFirmwareUpdate())! as OTAFirmwareUpdate;
    case 'SerialFirmwareUpdate': return accessor(index, new SerialFirmwareUpdate())! as SerialFirmwareUpdate;
    default: return null;
  }
}
