import Image from 'next/image'

import DeviceLogo from './DeviceLogo'
import { ConnectedDevice } from '../../src-tauri/bindings/ConnectedDevice'

function DeviceInfoBar({ device }: { device: ConnectedDevice }) {
    return (
        <div className='flex items-center justify-between p-4 mx-2 border-b h-1/6 border-slate-600'>
            <div>
                <p className='uppercase'>
                    <span className='text-emerald-500'>CONNECTED</span> - {device.device_type}
                </p>
                <p>Serial: {device.serial_number}</p>
                <p>UID: {device.device_details ? device.device_details.uid : 'N/A'}</p>

            </div>
            <span className='mx-6 mt-1'>
                <Image
                    width={100}
                    height={50}
                    src={DeviceLogo(device)}
                    alt={device.device_type + ' Logo'}
                />
            </span>
        </div>
    )
}

export default DeviceInfoBar