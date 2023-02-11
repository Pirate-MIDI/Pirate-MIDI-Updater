import ReactMarkdown from 'react-markdown'

// pub url: String,
// pub html_url: String,
// pub assets_url: String,
// pub upload_url: String,
// pub tarball_url: Option<String>,
// pub zipball_url: Option<String>,
// pub discussion_url: Option<String>,
// pub id: u64,
// pub node_id: String,
// pub tag_name: String,
// pub target_commitish: String,
// pub name: Option<String>,
// pub body: Option<String>,
// pub draft: bool,
// pub prerelease: bool,
// pub created_at: String,
// pub published_at: Option<String>,
// pub assets: Vec<Asset>,

function ReleaseInfoBar({ release }) {
    const published = release.published_at ? new Date(release.published_at).toDateString() : 'Unknown'

    const getChannel = (prerelease) => {
        return prerelease ? (
            <span className='text-amber-500'>Beta</span>
        ) : (
            <span className='text-emerald-500'>Stable</span>
        )
    }

    return release.body ? (
        <div className='mx-2 border-b h-4/6 border-slate-600'>
            <div className='flex items-center justify-between p-4 border-b h-1/6 border-slate-600'>
                <span className='text-xl font-bold'>{release.name}</span>
                <div className='text-right'>
                    <p className='text-sm text-slate-400'>Channel: <strong>{getChannel(release.prerelease)}</strong></p>
                    <p className='text-sm text-slate-400'>Published: <strong>{published}</strong></p>
                </div>
            </div>
            <ReactMarkdown className='p-4 overflow-y-auto h-5/6 markdown dark:[color-scheme:dark]' children={release.body}></ReactMarkdown>
        </div>
    ) : (
        <div className='flex items-center justify-center py-12 text-slate-400'>
            <span>This release does not contain any additional information</span>
        </div>
    )
}

export default ReleaseInfoBar;