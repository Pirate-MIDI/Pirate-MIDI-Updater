import { encode, stringify } from 'querystring'
import Image from 'next/image';
import FadeIn from "react-fade-in";
import DeviceLogo from '../../components/DeviceLogo';
import { useRouter } from 'next/router';
import { ArrowRightIcon } from '@heroicons/react/24/outline';

import pirateMidiImage from '../../assets/piratemidi.png'

import type { ConnectedDevice } from '../../../src-tauri/bindings/ConnectedDevice'

function AvailableDevices({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();

    return (
        <FadeIn>
            <div className='flex items-center py-4 mx-4 mb-2 border-b'>
                <Image
                    width={75}
                    height={75}
                    src={pirateMidiImage}
                    alt='Pirate MIDI Logo'
                />
                <div className='mx-4'>
                    <p className='text-lg font-bold'>{devices.length} Devices Connected:</p>
                    <p className='text-xs'>You can connect multiple devices at the same time.</p>
                </div>
            </div>

            <ul className='w-full h-full p-0 px-4 py-2 overflow-y-auto'>
                {devices.map((device) => (
                    <li className='w-full' key={device.id}>
                        <button className='device-button' onClick={() => {
                            router.push({
                                pathname: '/releases',
                                query: { serial_number: device.serial_number }
                            }, '/releases')
                        }}>
                            <span className='mx-6 mt-1'>
                                <Image
                                    width={100}
                                    height={50}
                                    src={DeviceLogo(device)}
                                    alt={device.device_type + ' Logo'}
                                />
                            </span>
                            <div className='flex flex-col flex-grow pl-8 mx-3 text-left border-l'>
                                <span className='text-lg font-bold'>{device.device_details ? device.device_details.deviceName : "N/A"}</span>
                            </div>
                            <div className='flex flex-col mx-3 text-xs text-right'>
                                <span>UID: <strong>{device.device_details ? device.device_details.uid : "N/A"}</strong></span>
                                <span>Firmware Version: <strong>{device.device_details ? device.device_details.firmwareVersion : "N/A"}</strong> </span>
                                <span>Hardware Version: <strong>{device.device_details ? device.device_details.hardwareVersion : "N/A"}</strong> </span>
                            </div>
                            <ArrowRightIcon className='w-5 h-5 mx-6' />
                        </button>
                    </li>
                ))}
            </ul>
        </FadeIn>
    )
}

export default AvailableDevices;