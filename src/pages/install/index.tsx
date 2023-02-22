import { useState, useEffect } from "react";
import { listen } from '@tauri-apps/api/event'
import FadeIn from "react-fade-in";
import { ConnectedDevice } from "../../../src-tauri/bindings/ConnectedDevice";
import { useRouter } from "next/router";
import ProgressBar from "../../components/ProgressBar";


function Install({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter();
    const [percent, setPercent] = useState<number>(0)

    // retrieve selected device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    // listen for install events
    useEffect(() => {
        const installListener = listen<number>('install_progress', event => setPercent(event.payload))

        // destructor
        return () => {
            installListener.then(f => f())
        }
    }, [])

    return (
        <FadeIn className="overflow-hidden">
            <div className="flex items-center justify-center w-screen h-screen overflow-hidden">
                {/* <span>Device will restart when finished</span> */}
                <div className="ldBar" data-preset="stripe"></div>
                <ProgressBar size={300} progress={percent} label={percent < 1 ? "Waiting for device..." : "Installing..."} />
            </div>
        </FadeIn>
    )
}

export default Install;