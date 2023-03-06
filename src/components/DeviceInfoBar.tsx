import Image from 'next/image'

import DeviceLogo from './DeviceLogo'
import { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice'

function DeviceInfoBar({ device }: { device: ConnectedDevice }) {
    return device ? (
        <div className='flex items-center justify-between p-4 mx-2 font-mono text-xs border-b h-1/6 border-slate-300'>
            <div>
                <p className='uppercase'>
                    <span className='text-emerald-500'>CONNECTED</span> - {device.device_type}
                </p>
                <div className={device.device_details ? '' : 'hidden'}>
                    <p>UID: {device.device_details ? device.device_details.uid : 'N/A'}</p>
                    <p>HARDWARE: {device.device_details ? device.device_details.hardwareVersion : 'N/A'} | FIRMWARE: {device.device_details ? device.device_details.firmwareVersion : 'N/A'}</p>
                </div>
            </div>
            <Image
                width={100}
                height={50}
                src={DeviceLogo(device)}
                alt={device.device_type + ' Logo'}
            />
        </div>
    ) : (
        <div className='flex items-center justify-center py-12 text-slate-400'>
            <span>Device is not available</span>
        </div>
    )
}

export default DeviceInfoBar