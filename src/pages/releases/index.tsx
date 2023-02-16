import FadeIn from "react-fade-in";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from 'next/router'
import { ArrowLeftIcon } from '@heroicons/react/24/outline';
import { useState, useEffect } from "react";

import ReleaseList from "../../components/ReleaseListColumn";
import DeviceInfo from "../../components/DeviceInfoBar";
import Placeholder from "../../components/Placeholder";
import ReleaseInfo from "../../components/ReleaseInfoBar";
import InstallBar from "../../components/InstallBar";

import type { Release } from "../../../src-tauri/bindings/Release";
import type { ConnectedDevice } from "../../../src-tauri/bindings/ConnectedDevice";

// import { Asset } from "../../../src-tauri/bindings/Asset";

function Releases({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();
    const [spinner, setSpinner] = useState(true)
    const [releases, setReleases] = useState([])
    const [selected, setSelected] = useState(undefined)

    // retrieve device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    // Retrieve releases from Github and select the latest release available
    useEffect(() => {
        if (device) {
            const retrieveReleases = async () => {
                await invoke("fetch_releases", { deviceType: device.device_type }).then((fetched: Release[]) => {
                    setReleases(fetched)
                    setSelected(fetched[0])
                    setSpinner(false)
                })
            };
            retrieveReleases()
        }
    }, [])

    return spinner ? (
        <Placeholder />
    ) : (
        <FadeIn className="overflow-hidden">
            <div className="flex h-screen overflow-hidden">
                <div className='flex flex-col w-1/4 max-w-xs border-r'>
                    <div className="px-3 py-1 border-b">
                        <button onClick={() => router.push('/devices')} className='flex items-center justify-around w-full px-4 py-2 my-2 border rounded border-slate-400'>
                            <ArrowLeftIcon className="w-5 h-5" />
                            <span>Device List</span>
                        </button>
                    </div>
                    <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
                </div>
                <div className="w-3/4">
                    <DeviceInfo />
                    <ReleaseInfo release={selected} />
                    <InstallBar release={selected} />
                </div>
            </div>
        </FadeIn>
    )
}

export default Releases;