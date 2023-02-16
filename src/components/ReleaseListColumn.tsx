import { CubeIcon, CubeTransparentIcon } from '@heroicons/react/24/outline';

function ReleaseListColumn({ releases, selected, onSelect }) {
    const createReleaseButton = (release, selected) => {
        const Icon = release.prerelease ? CubeTransparentIcon : CubeIcon;
        const stylePrerelease = release.prerelease ? 'border-amber-500' : 'border-emerald-500';
        const styleSelected = (release && release.id === selected.id) ? release.prerelease ? 'font-bold text-slate-800 bg-amber-400' : 'font-bold text-slate-800 bg-emerald-400' : '';
        return (
            <button onClick={() => onSelect(release)} className={`w-full px-4 py-2 my-2 rounded border flex justify-between items-center ${stylePrerelease} ${styleSelected}`}>
                <Icon className='w-5 h-5' />
                <span>{release.tag_name}</span>
            </button>
        );
    }

    return (
        <ul className="h-screen p-3 overflow-y-auto dark:[color-scheme:dark]">
            {releases.map((release) => (
                <li key={release.id}>
                    {createReleaseButton(release, selected)}
                </li>
            ))}
        </ul>
    )
}

export default ReleaseListColumn;