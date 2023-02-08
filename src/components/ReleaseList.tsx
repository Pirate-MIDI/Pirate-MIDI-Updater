import { CubeIcon, CubeTransparentIcon } from '@heroicons/react/24/outline';

function ReleaseList({ releases, selected, onSelect }) {

    const createReleaseButton = (release, selected) => {
        const Icon = release.prerelease ? CubeTransparentIcon : CubeIcon;
        const stylePrerelease = release.prerelease ? 'border-amber-500' : 'border-emerald-500';
        const styleSelected = (release && release.id === selected.id) ? release.prerelease ? 'text-slate-800 bg-amber-300' : 'text-slate-800 bg-emerald-300' : '';
        return (
            <button onClick={() => onSelect(release)} className={`w-full px-4 py-2 my-2 rounded border flex justify-between ${stylePrerelease} ${styleSelected}`}>
                <Icon className='w-4 h-4 mt-1' />
                <span>{release.tag_name}</span>
            </button>
        );
    }

    return (
        <div className="w-1/4 max-w-xs">
            <ul className="items-center h-screen p-3 overflow-y-auto">
                {releases.map((release) => (
                    <li key={release.id}>
                        {createReleaseButton(release, selected)}
                    </li>
                ))}
            </ul>
        </div>
    )
}

export default ReleaseList;