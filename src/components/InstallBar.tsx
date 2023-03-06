import { ArrowRightIcon, CheckBadgeIcon } from '@heroicons/react/24/outline';

function InstallBar({ release, onClick }) {
    const stylePrerelease = (release) => {
        return release.prerelease ? 'hover:bg-amber-400 border-amber-500' : 'hover:bg-emerald-400 border-emerald-500';
    }

    return (
        <div className="flex items-center justify-end w-full p-4 h-1/6">
            <button onClick={onClick} className={`flex items-center px-4 py-2 m-4 font-bold border rounded hover:text-slate-800 ${stylePrerelease(release)}`}>
                <CheckBadgeIcon className='icon-left' />
                Install {release.name}
                <ArrowRightIcon className='icon-right' />
            </button>
        </div>
    )
}

export default InstallBar