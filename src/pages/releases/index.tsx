import FadeIn from "react-fade-in";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from 'next/router'
import { ArrowLeftIcon, ArrowRightIcon, CheckBadgeIcon } from '@heroicons/react/24/outline';
import { useState, useEffect } from "react";

import ReleaseList from "../../components/ReleaseListColumn";
import DeviceInfo from "../../components/DeviceInfoBar";
import Placeholder from "../../components/Placeholder";
import ReleaseInfo from "../../components/ReleaseInfoBar";
import InstallBar from "../../components/InstallBar";

import type { Release } from "../../../src-tauri/bindings/Release";
import type { ConnectedDevice } from "../../../src-tauri/bindings/ConnectedDevice";
import BridgeModal from "../../components/BridgeModal";

// import { Asset } from "../../../src-tauri/bindings/Asset";

function Releases({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();
    const [releases, setReleases] = useState([])
    const [selected, setSelected] = useState(undefined)
    const [isOpen, setIsOpen] = useState(false)

    // retrieve selected device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    const onClose = () => {
        setIsOpen(false)
    }

    const onAccept = async (connected: ConnectedDevice, release: Release) => {
        onClose()
        await invoke("remote_binary", { device: connected, release })
    }

    const onRemoteInstall = async (connected: ConnectedDevice, release: Release) => {
        // show the bridge cable diagram
        if (connected.device_type === "Bridge6" || connected.device_type === "Bridge4") {
            setIsOpen(true)
        } else {
            await onAccept(connected, release)
        }
    }

    const stylePrerelease = (release) => {
        return release.prerelease ? 'hover:bg-amber-400 border-amber-500' : 'hover:bg-emerald-400 border-emerald-500';
    }

    // retrieve releases from Github and select the latest release available
    useEffect(() => {
        if (device && device.releases) {
            setReleases(device.releases)
            // get the latest stable version - not beta
            setSelected(device.releases.find((rel) => !rel.prerelease))
        }
    }, [device])

    return releases.length < 1 ? (
        <Placeholder />
    ) : (
        <FadeIn className="">
            <div className="flex flex-col h-screen ">
                <div className="flex h-5/6">
                    <div className='flex flex-col w-1/4 max-w-xs border-r'>
                        <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
                    </div>
                    <div className="w-3/4">
                        <DeviceInfo device={device} />
                        <ReleaseInfo release={selected} />
                    </div>
                </div>
                <div className="flex items-center justify-between p-4 border-t h-1/6">
                    <button onClick={() => router.back()} className='flex items-center px-4 py-2 border rounded border-slate-400'>
                        <ArrowLeftIcon className="icon-left" />
                        <span>Device List</span>
                    </button>
                    <button onClick={() => onRemoteInstall(device, selected)} className={`flex items-center px-4 py-2 font-bold border rounded hover:text-slate-800 ${stylePrerelease(selected)}`}>
                        <CheckBadgeIcon className='icon-left' />
                        Install {selected.name}
                        <ArrowRightIcon className='icon-right' />
                    </button>
                </div>
            </div>
            <BridgeModal show={isOpen} onClose={onClose} onAccept={() => onAccept(device, selected)} />
        </FadeIn>
    )
}

export default Releases;