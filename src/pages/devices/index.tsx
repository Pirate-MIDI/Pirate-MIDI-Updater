import { invoke } from "@tauri-apps/api/tauri";
import Image from 'next/image';
import FadeIn from "react-fade-in";
import DeviceLogo from '../../components/DeviceLogo';
import BridgeModal from '../../components/BridgeModal';
import { useRouter } from 'next/router';
import { DocumentIcon, ArrowUpIcon, ArrowRightIcon, CheckBadgeIcon } from '@heroicons/react/24/outline';

import pirateMidiImage from '../../assets/piratemidi.png'

import type { ConnectedDevice } from '../../../src-tauri/bindings/ConnectedDevice'
import { useState } from "react";

function AvailableDevices({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter()
    const [isOpen, setIsOpen] = useState(false)
    const [selected, setSelected] = useState<ConnectedDevice>(undefined)

    const onClose = () => {
        setIsOpen(false)
    }

    const onAccept = async () => {
        onClose()
        await invoke("local_binary", { device: selected })
    }

    const openFilePrompt = async (device: ConnectedDevice) => {
        // show the bridge cable diagram
        if (device.device_type === "Bridge6" || device.device_type === "Bridge4") {
            setIsOpen(true)
            setSelected(device)
        } else {
            await invoke("local_binary", { device: device })
        }
    }

    return (
        <FadeIn className="overflow-hidden">
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
                        <div className='device-button'>
                            <span className='mx-2 mt-1'>
                                <Image
                                    width={100}
                                    height={50}
                                    src={DeviceLogo(device)}
                                    alt={device.device_type + ' Logo'}
                                />
                            </span>
                            <div className='flex flex-col flex-grow pl-8 mx-2 text-xs text-left border-l'>
                                <span className='text-lg font-bold'>{device.device_details ? device.device_details.deviceName : "N/A"}</span>
                                <span>UID: <strong>{device.device_details ? device.device_details.uid : "N/A"}</strong></span>
                                <span>Firmware: <strong>{device.device_details ? device.device_details.firmwareVersion : "N/A"}</strong> </span>
                                <span>Hardware: <strong>{device.device_details ? device.device_details.hardwareVersion : "N/A"}</strong> </span>
                            </div>
                            <div className="flex flex-col items-center">
                                <p className="text-sm">Select an installation method:</p>
                                <div className="flex flex-row items-center">
                                    <button onClick={() => openFilePrompt(device)} className={`flex items-center px-4 py-2 m-2 text-xs border rounded border-blue-500 hover:bg-blue-400 hover:text-slate-800`}>
                                        <DocumentIcon className='w-5 h-5 mr-4' />
                                        Local File
                                        <ArrowUpIcon className='w-5 h-5 ml-4' />
                                    </button>
                                    <span>OR</span>
                                    <button onClick={() => {
                                        router.push({
                                            pathname: '/releases',
                                            query: { serial_number: device.serial_number }
                                        }, '/releases')
                                    }} className={`flex items-center px-4 py-2 m-2 text-xs border rounded hover:bg-emerald-400 border-emerald-500 hover:text-slate-800`}>
                                        <CheckBadgeIcon className='w-5 h-5 mr-4' />
                                        Official Release
                                        <ArrowRightIcon className='w-5 h-5 ml-4' />
                                    </button>
                                </div>
                            </div>
                        </div>
                    </li>
                ))}
            </ul>
            <BridgeModal show={isOpen} onClose={onClose} onAccept={onAccept} />
        </FadeIn>
    )
}

export default AvailableDevices;