import FadeIn from 'react-fade-in';
import { invoke } from '@tauri-apps/api/tauri';
import { useRouter } from 'next/router'
import { ArrowLeftIcon, ArrowRightIcon, CheckBadgeIcon, ChevronUpDownIcon } from '@heroicons/react/24/outline';
import { useState, useEffect } from 'react';

import ReleaseList from '../../components/ReleaseListColumn';
import DeviceInfo from '../../components/DeviceInfoBar';
import Placeholder from '../../components/Placeholder';
import ReleaseInfo from '../../components/ReleaseInfoBar';

import type { Release } from '../../../src-tauri/bindings/Release';
import type { ConnectedDevice } from '../../../src-tauri/bindings/ConnectedDevice';

function Releases({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();
    const [releases, setReleases] = useState([])
    const [selected, setSelected] = useState(undefined)
    const [showAllReleases, setShowAllReleases] = useState(false)

    // retrieve selected device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    const onRemoteInstall = async (connected: ConnectedDevice, release: Release) => {
        await invoke('remote_binary', { device: connected, release })
    }

    const stylePrerelease = (release) => {
        return release.prerelease ? 'bg-amber-400 border-amber-500' : 'bg-emerald-300 border-emerald-400 hover:bg-emerald-400';
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
        <FadeIn className='overflow-hidden'>
            <div className='flex flex-col h-screen overflow-hidden'>
                <div className='flex h-5/6'>
                    <div className={showAllReleases ? 'flex flex-col w-1/4 max-w-xs border-r' : 'hidden'}>
                        <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
                    </div>
                    <div className={showAllReleases ? 'w-3/4' : 'w-full'}>
                        <DeviceInfo device={device} />
                        <ReleaseInfo device={device} release={selected} />
                    </div>
                </div>
                <div className='flex items-center justify-between p-4 border-t h-1/6'>
                    <button onClick={() => setShowAllReleases(!showAllReleases)} className='flex items-center px-4 py-2 border rounded border-slate-300'>
                        <ChevronUpDownIcon className='icon-left' />
                        Select a different release
                    </button>
                    <div className='flex'>
                        <button onClick={() => router.back()} className='flex items-center px-4 py-2 m-2 border rounded border-slate-300'>
                            <ArrowLeftIcon className='icon-left' />
                            Back to Device List
                        </button>
                        <button onClick={() => onRemoteInstall(device, selected)} className={`flex items-center px-4 py-2 m-2 border rounded text-slate-800 ${stylePrerelease(selected)}`}>
                            <CheckBadgeIcon className='icon-left' />
                            Install {selected.name}
                            <ArrowRightIcon className='icon-right' />
                        </button>
                    </div>
                </div>
            </div>
        </FadeIn>
    )
}

export default Releases;