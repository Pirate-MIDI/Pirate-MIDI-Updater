import { invoke } from "@tauri-apps/api/tauri";
import { ArrowUpIcon, ArrowRightIcon, CheckBadgeIcon, DocumentIcon } from '@heroicons/react/24/outline';

function InstallBar({ release }) {
    const openFilePrompt = async () => {
        await invoke("prompt_local_file")
    }
    const stylePrerelease = (release) => {
        return release.prerelease ? 'hover:bg-amber-400 border-amber-500' : 'hover:bg-emerald-400 border-emerald-500';
    }

    return (
        <div className="flex items-center justify-end p-4 h-1/6">
            <button onClick={() => openFilePrompt()} className={`flex items-center px-4 py-2 m-4 font-bold border rounded border-blue-500 hover:bg-blue-400 hover:text-slate-800`}>
                <DocumentIcon className='w-5 h-5 mr-3' />
                Install Local File
                <ArrowUpIcon className='w-5 h-5 ml-3' />
            </button>
            <span>OR</span>
            <button className={`flex items-center px-4 py-2 m-4 font-bold border rounded hover:text-slate-800 ${stylePrerelease(release)}`}>
                <CheckBadgeIcon className='w-5 h-5 mr-3' />
                Install {release.name}
                <ArrowRightIcon className='w-5 h-5 ml-3' />
            </button>
        </div>
    )
}

export default InstallBar