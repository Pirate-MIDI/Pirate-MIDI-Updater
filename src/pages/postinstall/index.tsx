import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import FadeIn from 'react-fade-in'
import { ConnectedDevice } from '../../../src-tauri/bindings/ConnectedDevice'
import { useRouter } from 'next/router'
import ProgressBar from '../../components/ProgressBar'
import { ArrowUturnLeftIcon } from '@heroicons/react/24/outline'

function PostInstall() {
    const onClick = async () => {
        await invoke("post_install")
    }

    return (
        <div className='flex flex-col items-center justify-center flex-shrink-0 w-screen h-screen overflow-hidden'>
            <FadeIn className='flex flex-col items-center mt-2'>
                <p className='mt-4 text-xl font-bold'>Installation Complete!</p>
                <p className='mb-4 text-sm text-slate-500'>It is safe to unplug your device</p>
                <button onClick={onClick} className='flex items-center px-4 py-2 m-2 border border-blue-600 rounded hover:bg-blue-500 hover:text-white'>
                    Update another device
                    <ArrowUturnLeftIcon className='icon-right' />
                </button>
            </FadeIn>
        </div>
    )
}

export default PostInstall