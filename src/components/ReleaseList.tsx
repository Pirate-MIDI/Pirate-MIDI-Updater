

function ReleaseList({ releases }) {
    return (
        <div className="w-1/4">
            <ul className="items-center h-screen p-3 overflow-y-auto">
                {releases.map((release) => (
                    <li key={release.id}><button className="block w-full my-2">{release.tag_name}</button></li>
                ))}
            </ul>
        </div>
    )
}

export default ReleaseList;