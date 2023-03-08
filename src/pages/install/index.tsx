import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import FadeIn from 'react-fade-in'
import { InstallProgress } from '../../../src-tauri/bindings/InstallProgress'
import { ConnectedDeviceType } from '../../../src-tauri/bindings/ConnectedDeviceType'
import { useRouter } from 'next/router'
import ProgressBar from '../../components/ProgressBar'
import BridgeModal from '../../components/BridgeModal';

function Install() {
    const router = useRouter()
    const [percent, setPercent] = useState<number>(0)
    const [status, setStatus] = useState<String>("Waiting")
    const [isOpen, setIsOpen] = useState(false)

    const device_type = router.query.device_type as ConnectedDeviceType;

    const onClose = () => {
        setIsOpen(false)
    }

    const onAccept = async () => {
        onClose()
        await invoke('post_install')
    }

    const label = (status) => {
        switch (status) {
            case "Preparing":
                return "Preparing device..."
            case "Installing":
                return "Installing..."
            case "Waiting":
                return "Waiting for device..."
        }
    }

    // listen for install events
    useEffect(() => {
        const installListener = listen<InstallProgress>('install_progress', event => {
            console.log(event.payload)
            setStatus(event.payload.status)
            setPercent(event.payload.progress)
        })

        // destructor
        return () => {
            installListener.then(f => f())
        }
    }, [])

    // if we don't recieve an updated status, show the modal after 10 seconds
    useEffect(() => {
        const interval = setInterval(() => {
            console.log('interval triggered - status:', status, '\n device:', device_type)
            if (status === "Waiting" && (device_type === 'Bridge6' || device_type === 'Bridge4')) {
                setIsOpen(true)
            }
        }, 10000);

        return () => clearInterval(interval);
    }, [status]);

    return (
        <div className='flex flex-col items-center justify-center flex-shrink-0 w-screen h-screen overflow-hidden'>
            <FadeIn>
                <ProgressBar size={300} progress={percent} label={label(status)} />
                <BridgeModal show={isOpen} onClose={onClose} onAccept={() => onAccept()} />
            </FadeIn>
        </div>
    )
}

export default Install