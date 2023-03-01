import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import FadeIn from 'react-fade-in'
import { ConnectedDevice } from '../../../src-tauri/bindings/ConnectedDevice'
import { useRouter } from 'next/router'
import ProgressBar from '../../components/ProgressBar'
import { ArrowUturnLeftIcon } from '@heroicons/react/24/outline'



function Install({ devices }: { devices: ConnectedDevice[] }) {
    const router = useRouter()
    const [percent, setPercent] = useState<number>(0)

    // retrieve selected device from router
    const device: ConnectedDevice = devices.find((d) => d.serial_number === router.query.serial_number)

    // listen for install events
    useEffect(() => {
        const installListener = listen<number>('install_progress', event => {
            console.log(event.payload)
            setPercent(event.payload)
        })

        // destructor
        return () => {
            installListener.then(f => f())
        }
    }, [])

    const onClick = async () => {
        await invoke("post_install")
    }

    return (
        <div className='flex flex-col items-center justify-center w-screen h-screen overflow-hidden'>
            <FadeIn>
                <ProgressBar size={300} progress={percent} label={percent < 1 ? 'Preparing device...' : percent < 100 ? 'Installing...' : 'Installation Complete'} />
            </FadeIn>
            <FadeIn visible={percent > 99} className='flex flex-col items-center mt-2'>
                <p className='mt-4 text-xl font-bold'>Installation Complete!</p>
                <p className='mb-4 text-sm text-slate-500'>You may now unplug your device</p>
                <button onClick={onClick} className='flex items-center px-4 py-2 m-2 border border-blue-600 rounded hover:bg-blue-500 hover:text-white'>
                    Update another device
                    <ArrowUturnLeftIcon className='w-5 h-5 ml-4' />
                </button>
            </FadeIn>
        </div>
    )
}

export default Install