import { useState, useEffect } from "react";
import { listen } from '@tauri-apps/api/event'
import FadeIn from "react-fade-in";
import { ConnectedDevice } from "../../../src-tauri/bindings/ConnectedDevice";
import { useRouter } from "next/router";
// import { Asset } from "../../../src-tauri/bindings/Asset";


function Install({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();
    const [status, setStatus] = useState(undefined)
    // const [selected, setSelected] = useState(undefined)

    // retrieve selected device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    // listen for install events
    useEffect(() => {
        const installListener = listen<ConnectedDevice[]>('devices_update', event => {
            console.log(event)
            setStatus(event.payload)
        })

        // destructor
        return () => {
            installListener.then(f => f())
        }
    }, [])

    return (
        <FadeIn>
            <div className="flex h-screen overflow-hidden">
                {/* <ReleaseList releases={releases} selected={selected} onSelect={(release) => setSelected(release)} />
                <div className="w-3/4">
                    <DeviceInfo />
                    <ReleaseInfo release={selected} />
                    <InstallBar release={selected} />
                </div> */}
                <div className="flex items-center justify-center text-center">
                    <span>Installing to device! Device will restart when finished</span>
                </div>
            </div>
        </FadeIn>
    )
}

export default Install;